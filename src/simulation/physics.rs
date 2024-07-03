use nannou::math::deg_to_rad;
use rapier2d::prelude::*;

use crate::frontend::ShapeType;

use super::ObjectBuilder;

pub struct Physics {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: Box<dyn BroadPhase>,
    narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: Box::new(BroadPhaseMultiSap::new()),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new()
        }
    }

    pub fn remove_colliders(&mut self, colliders: &[ColliderHandle]) {
        for collider in colliders {
            self.colliders.remove(*collider, &mut self.island_manager, &mut self.bodies, true);
        }
    }

    pub fn add_collider(&mut self, handle: RigidBodyHandle, shape: ShapeType, bounciness: f32, width: f32, height: f32, stroke_weight: f32) {
        let half_stroke = stroke_weight / 2.0;
        let width = width + half_stroke;
        let height = height + half_stroke;

        match shape {
            ShapeType::Circle => {
                let collider = ColliderBuilder::ball(width)
                    .restitution(bounciness)
                    .active_events(ActiveEvents::all())
                    .build();
                self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
            },
            ShapeType::Rect => {
                let collider = ColliderBuilder::cuboid(width, height)
                    .restitution(bounciness)
                    .active_events(ActiveEvents::all())
                    .build();
                self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
            },
            ShapeType::Ring => {
                let mut vertices = vec![];

                for i in (0..=360).step_by(2) {
                    let radian = deg_to_rad(i as f32);
                    vertices.push(point![radian.sin(), radian.cos()] * width);
                }

                for point in vertices {
                    let collider = ColliderBuilder::ball(stroke_weight / 2.0)
                        .position(point.into())
                        .restitution(bounciness)
                        .active_events(ActiveEvents::all())
                        .build();

                    self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
                }
            }
        }
    }

    pub fn add(&mut self, builder: &ObjectBuilder) -> RigidBodyHandle {
        let rigidbody = if builder.fixed {
            RigidBodyBuilder::fixed()
        } else {
            RigidBodyBuilder::dynamic()
        }.ccd_enabled(true).linvel(builder.vel).gravity_scale(builder.gravity).position(builder.pos.into()).build();

        let handle = self.bodies.insert(rigidbody);

        self.add_collider(handle, builder.shape, builder.bounciness, builder.width, builder.height, builder.stroke_weight);

        handle
    }

    pub fn step(&mut self) -> Vec<(ColliderHandle, ColliderHandle)> {
        let hooks = ();

        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, _contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            self.broad_phase.as_mut(),
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &hooks,
            &event_handler
        );

        let mut collisions = vec![];

        while let Ok(collision_event) = collision_recv.try_recv() {
            if collision_event.started() {
                collisions.push((collision_event.collider1(), collision_event.collider2()));
            }
        }

        collisions
    }
}
