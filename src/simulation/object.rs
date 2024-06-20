use nannou::prelude::*;
use rapier2d::prelude::*;
use rand::Rng;

use crate::frontend::ShapeType;

use super::physics::Physics;

pub struct ObjectBuilder {
    pub shape: ShapeType,
    pub size: f32,
    pub stroke_weight: f32,
    pub bounciness: f32,
    pub color: Rgb<u8>,

    pub pos: Vector<Real>,
    pub vel: Vector<Real>,
    pub gravity: f32,
    pub fixed: bool,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            shape: ShapeType::Circle,
            size: 10.0,
            stroke_weight: 3.0,
            color: WHITE,

            bounciness: 0.5,

            pos: vector![0.0, 0.0],
            vel: vector![0.0, 0.0],
            gravity: 0.0,
            fixed: false,
        }
    }

    pub fn shape(mut self, shape: ShapeType) -> ObjectBuilder {
        self.shape = shape;
        self
    }

    pub fn size(mut self, size: f32) -> ObjectBuilder {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Rgb<u8>) -> ObjectBuilder {
        self.color = color;
        self
    }

    pub fn stroke(mut self, stroke_weight: f32) -> ObjectBuilder {
        self.stroke_weight = stroke_weight;
        self
    }

    pub fn x(mut self, x: f32) -> ObjectBuilder {
        self.pos.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> ObjectBuilder {
        self.pos.y = y;
        self
    }

    pub fn speed(mut self, speed: f32) -> ObjectBuilder {
        let vel = vector!(
            rand::thread_rng().gen_range(-1.0..=1.0),
                rand::thread_rng().gen_range(-1.0..=1.0)).normalize() * speed;
        self.vel = vel;
        self
    }

    pub fn gravity(mut self, gravity: f32) -> ObjectBuilder {
        self.gravity = gravity;
        self
    }

    pub fn bounciness(mut self, bounciness: f32) -> ObjectBuilder {
        self.bounciness = bounciness;
        self
    }

    pub fn fixed(mut self, fixed: bool) -> ObjectBuilder {
        self.fixed = fixed;
        self
    }

    pub fn build(self, physics: &mut Physics) -> Object {
        let handle = physics.add(&self);

        Object {
            shape: self.shape,
            size: self.size,
            stroke_weight: self.stroke_weight,
            color: self.color,

            handle
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    shape: ShapeType,
    size: f32,
    stroke_weight: f32,
    color: Rgb<u8>,

    handle: RigidBodyHandle,
}

impl Object {
    pub fn draw(&self, draw: &Draw, physics: &Physics) {
        match self.shape {
            ShapeType::Circle => {
                if let Some(rigidbody) = physics.bodies.get(self.handle) {
                    let pos = rigidbody.position().translation;

                    draw.ellipse()
                        .x_y(pos.x, pos.y)
                        .radius(self.size)
                        .color(self.color);
                }
            }
            ShapeType::Square => {
                if let Some(rigidbody) = physics.bodies.get(self.handle) {
                    let pos = rigidbody.position().translation;

                    draw.rect()
                        .x_y(pos.x, pos.y)
                        .w_h(self.size, self.size)
                        .color(self.color);
                }
            }
            ShapeType::Ring => {
                if let Some(rigidbody) = physics.bodies.get(self.handle) {
                    let pos = rigidbody.position().translation;

                    let points = (0..=360).map(|i| {
                        let radian = deg_to_rad(i as f32);

                        let x = pos.x + radian.sin() * self.size;
                        let y = pos.y + radian.cos() * self.size;

                        (pt2(x, y), self.color)
                    });

                    draw.polyline().stroke_weight(self.stroke_weight).points_colored(points);
                }
            }
        }
    }
}
