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

    pub fn add(&mut self, builder: &ObjectBuilder) -> RigidBodyHandle {
        let rigidbody = if builder.fixed {
            RigidBodyBuilder::fixed()
        } else {
            RigidBodyBuilder::dynamic()
        }.ccd_enabled(true).linvel(builder.vel).gravity_scale(builder.gravity).position(builder.pos.into()).build();

        let handle = self.bodies.insert(rigidbody);

        match builder.shape {
            ShapeType::Circle => {
                let collider = ColliderBuilder::ball(builder.width).restitution(builder.bounciness).build();

                self.colliders.insert_with_parent(collider, handle, &mut self.bodies);

            },
            ShapeType::Rect => {
                let collider = ColliderBuilder::cuboid(builder.width, builder.height).restitution(builder.bounciness).build();

                self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
            },
            ShapeType::Ring => {
                let mut vertices = vec![];

                for i in (0..=360).step_by(2) {
                    let radian = deg_to_rad(i as f32);
                    vertices.push(point![radian.sin(), radian.cos()] * builder.width);
                }

                for point in vertices {
                    let collider = ColliderBuilder::ball(builder.stroke_weight / 2.0).position(point.into()).restitution(builder.bounciness).build();

                    self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
                }
            }
        }

        handle
    }

    pub fn step(&mut self) {
        let hooks = ();
        let events = ();

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
            &events
        );
    }
}
