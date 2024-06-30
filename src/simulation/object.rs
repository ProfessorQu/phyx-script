use core::panic;
use std::collections::HashMap;

use nalgebra::SimdComplexField;
use nannou::prelude::*;
use rapier2d::prelude::*;
use rand::Rng;

use crate::{frontend::{ast::Statement, ShapeType}, runtime::{eval_object_update_expr, Environment, Function, RuntimeValue}};

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

    pub update_fn: Option<Function>,

    pub others: HashMap<String, RuntimeValue>
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
            
            update_fn: None,

            others: HashMap::new()
        }
    }

    pub fn from_map(map: HashMap<String, RuntimeValue>, physics: &mut Physics) -> Object {
        let mut builder = ObjectBuilder::new();
        for (key, value) in map {
            builder = match (key.as_str(), value) {
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
                ("update", RuntimeValue::Function(Function { name, parameters, body, declaration_env })) => builder.update(name, parameters, body, declaration_env),
                (key, value) => builder.other(key.to_string(), value)
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

    pub fn update(mut self, name: String, parameters: Vec<String>, body: Vec<Statement>, declaration_env: Environment) -> ObjectBuilder {
        self.update_fn = Some(Function::new(name, parameters, body, declaration_env));
        self
    }

    pub fn other(mut self, key: String, value: RuntimeValue) -> ObjectBuilder {
        self.others.insert(key, value);
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
            bounciness: self.bounciness,
            stroke_weight: self.stroke_weight,
            color: self.color,

            handle,

            update_fn: self.update_fn,

            others: self.others
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    shape: ShapeType,
    width: f32,
    height: f32,
    bounciness: f32,
    stroke_weight: f32,
    color: Rgb<u8>,

    handle: RigidBodyHandle,

    update_fn: Option<Function>,

    others: HashMap<String, RuntimeValue>
}

impl Object {
    pub fn update(&mut self, physics: &mut Physics) {
        let object_map = self.to_map(physics);
        let object = RuntimeValue::Object(object_map);

        let func = match &mut self.update_fn {
            Some(func) => func,
            None => return
        };

        let new_map = match eval_object_update_expr(object, func) {
            RuntimeValue::Object(map) => map,
            _ => panic!("Invalid object")
        };

        self.update_map(new_map, physics);
    }

    pub fn to_map(&self, physics: &Physics) -> HashMap<String, RuntimeValue> {
        let mut map = HashMap::new();

        let rigidbody = physics.bodies.get(self.handle).expect("Invalid handle");
        let pos = rigidbody.position().translation;
        let gravity = rigidbody.gravity_scale();

        map.insert("x".to_string(), RuntimeValue::Number(pos.x));
        map.insert("y".to_string(), RuntimeValue::Number(pos.y));
        map.insert("width".to_string(), RuntimeValue::Number(self.width));
        map.insert("height".to_string(), RuntimeValue::Number(self.height));
        map.insert("bounciness".to_string(), RuntimeValue::Number(self.bounciness));
        map.insert("color".to_string(), RuntimeValue::Color(self.color));
        map.insert("size".to_string(), RuntimeValue::Number(self.width));
        map.insert("gravity".to_string(), RuntimeValue::Number(gravity));
        map.insert("stroke".to_string(), RuntimeValue::Number(self.stroke_weight));
        map.insert("shape".to_string(), RuntimeValue::Shape(self.shape));

        for (key, value) in self.others.clone() {
            map.insert(key, value);
        }

        map
    }

    pub fn update_map(&mut self, new_map: HashMap<String, RuntimeValue>, physics: &mut Physics) {
        let rigidbody = physics.bodies.get_mut(self.handle).expect("Failed to get rigidbody");
        let mut pos = *rigidbody.position();

        let wake_up = !rigidbody.is_sleeping();

        for (key, value) in new_map {
            match (key.as_str(), value) {
                ("x", RuntimeValue::Number(number)) => pos.translation.x = number,
                ("y", RuntimeValue::Number(number)) => pos.translation.y = number,
                ("width", RuntimeValue::Number(number)) => self.width = number,
                ("height", RuntimeValue::Number(number)) => self.height = number,
                ("bounciness", RuntimeValue::Number(number)) => self.bounciness = number,
                ("color", RuntimeValue::Color(color)) => self.color = color,
                ("size", RuntimeValue::Number(number)) => self.width = number,
                ("gravity", RuntimeValue::Number(number)) => rigidbody.set_gravity_scale(number, wake_up),
                ("stroke", RuntimeValue::Number(number)) => self.stroke_weight = number,
                ("shape", RuntimeValue::Shape(shape)) => self.shape = shape,
                (key, value) => {
                    if self.others.contains_key(key) {
                        self.others.insert(key.to_string(), value);
                    } else {
                        panic!("Invalid key-value pair to update object: {}-{}", key, value);
                    }
                }
            }
        }

        rigidbody.set_position(pos, wake_up);
        self.update_shape(physics);
    }

    pub fn update_shape(&mut self, physics: &mut Physics) {
        let rigidbody = physics.bodies.get(self.handle).expect("Failed to get rigidbody").clone();
        
        for collider in rigidbody.colliders() {
            physics.remove_colliders(collider);
        }

        physics.add_collider(self.handle, self.shape, self.bounciness, self.width, self.height, self.stroke_weight);
    }

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
