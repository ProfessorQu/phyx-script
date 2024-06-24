use core::panic;
use std::collections::HashMap;

use nalgebra::SimdComplexField;
use nannou::prelude::*;
use rapier2d::prelude::*;
use rand::Rng;

use crate::{frontend::ShapeType, runtime::RuntimeValue};

use super::physics::Physics;

pub struct ObjectBuilder {
    pub shape: ShapeType,
    pub width: f32,
    pub height: f32,
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
            width: 10.0,
            height: 10.0,
            stroke_weight: 3.0,
            color: WHITE,

            bounciness: 0.5,

            pos: vector![0.0, 0.0],
            vel: vector![0.0, 0.0],
            gravity: 0.0,
            fixed: false,
        }
    }

    pub fn from_map(map: HashMap<String, RuntimeValue>, physics: &mut Physics) -> Object {
        let mut builder = ObjectBuilder::new();
        for (key, value) in map {
            builder = match (key.as_str(), value.clone()) {
                ("size", RuntimeValue::Number(number)) => builder.size(number),
                ("width", RuntimeValue::Number(number)) => builder.width(number),
                ("height", RuntimeValue::Number(number)) => builder.height(number),
                ("gravity", RuntimeValue::Number(number)) => builder.gravity(number),
                ("speed", RuntimeValue::Number(number)) => builder.speed(number),
                ("stroke", RuntimeValue::Number(number)) => builder.stroke(number),
                ("x", RuntimeValue::Number(number)) => builder.x(number),
                ("y", RuntimeValue::Number(number)) => builder.y(number),
                ("bounciness", RuntimeValue::Number(number)) => builder.bounciness(number),
                ("color", RuntimeValue::Color(color)) => builder.color(color),
                ("fixed", RuntimeValue::Boolean(boolean)) => builder.fixed(boolean),
                ("shape", RuntimeValue::Shape(shape)) => builder.shape(shape),
                _ => panic!("Invalid key-value pair: {:?}: {:?}", key, value)
            }
        }

        builder.build(physics)
    }

    pub fn shape(mut self, shape: ShapeType) -> ObjectBuilder {
        self.shape = shape;
        self
    }

    pub fn size(mut self, size: f32) -> ObjectBuilder {
        self.width = size;
        self.height = size;
        self
    }

    pub fn width(mut self, width: f32) -> ObjectBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> ObjectBuilder {
        self.height = height;
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

        if self.shape != ShapeType::Rect && self.width != self.height {
            panic!("A circle and ring must have the same width and height")
        }

        Object {
            shape: self.shape,
            width: self.width,
            height: self.height,
            stroke_weight: self.stroke_weight,
            color: self.color,

            handle
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    shape: ShapeType,
    width: f32,
    height: f32,
    stroke_weight: f32,
    color: Rgb<u8>,

    handle: RigidBodyHandle,
}

impl Object {
    pub fn draw(&self, draw: &Draw, physics: &Physics) {
        let rigidbody = match physics.bodies.get(self.handle) {
            Some(rb) => rb,
            None => panic!("Object doesn't have associated handle")
        };
        let pos = rigidbody.position().translation;

        match self.shape {
            ShapeType::Circle => {
                draw.ellipse()
                    .x_y(pos.x, pos.y)
                    .radius(self.width)
                    .color(self.color);
            }
            ShapeType::Rect => {
                let rot = rigidbody.rotation().simd_to_polar().1;

                draw.rect()
                    .x_y(pos.x, pos.y)
                    .w_h(2.0 * self.width, 2.0 * self.height)
                    .rotate(rot)
                    .color(self.color);
            }
            ShapeType::Ring => {
                let points = (0..=360).map(|i| {
                    let radian = deg_to_rad(i as f32);

                    let x = pos.x + radian.sin() * self.width;
                    let y = pos.y + radian.cos() * self.width;

                    (pt2(x, y), self.color)
                });

                draw.polyline().stroke_weight(self.stroke_weight).points_colored(points);
            }
        }
    }
}
