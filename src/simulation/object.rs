use core::panic;
use std::{collections::HashMap, path::Path};

use nalgebra::SimdComplexField;
use nannou::prelude::*;
use rapier2d::prelude::*;
use rand::Rng;

use crate::{frontend::{ast::Statement, ShapeType}, runtime::{eval_runtime_object_expr, Environment, Function, RuntimeValue}};

use super::{physics::Physics, Audio};

pub struct ObjectBuilder {
    pub shape: ShapeType,

    pub pos: Vector<Real>,
    pub vel: Vector<Real>,

    pub width: f32,
    pub height: f32,

    pub gravity: f32,
    pub bounciness: f32,
    pub fixed: bool,

    pub color: Rgb<u8>,
    pub stroke_color: Rgb<u8>,
    pub stroke_weight: f32,

    pub hit_note: String,
    pub hit_note_volume: f32,

    pub frames_per_trail_obj: Option<u128>,

    pub update_fn: Option<Function>,
    pub hit_fn: Option<Function>,

    pub others: HashMap<String, RuntimeValue>
}

impl ObjectBuilder {
    pub fn new() -> Self {
        Self {
            shape: ShapeType::Circle,

            pos: vector![0.0, 0.0],
            vel: vector![0.0, 0.0],

            width: 10.0,
            height: 10.0,

            gravity: 0.0,
            bounciness: 0.5,
            fixed: false,

            color: WHITE,
            stroke_color: WHITE,
            stroke_weight: 3.0,

            hit_note: "A0vH".to_string(),
            hit_note_volume: 0.0,

            frames_per_trail_obj: None,
            
            update_fn: None,
            hit_fn: None,

            others: HashMap::new()
        }
    }

    pub fn from_map(map: HashMap<String, RuntimeValue>, physics: &mut Physics) -> Object {
        let mut builder = ObjectBuilder::new();
        for (key, value) in map {
            builder = match (key.as_str(), value) {
                ("shape", RuntimeValue::Shape(shape)) => builder.shape(shape),

                ("x", RuntimeValue::Number(number)) => builder.x(number),
                ("y", RuntimeValue::Number(number)) => builder.y(number),
                ("speed", RuntimeValue::Number(number)) => builder.speed(number),

                ("width", RuntimeValue::Number(number)) => builder.width(number),
                ("height", RuntimeValue::Number(number)) => builder.height(number),
                ("size", RuntimeValue::Number(number)) => builder.size(number),

                ("gravity", RuntimeValue::Number(number)) => builder.gravity(number),
                ("bounciness", RuntimeValue::Number(number)) => builder.bounciness(number),
                ("fixed", RuntimeValue::Boolean(boolean)) => builder.fixed(boolean),

                ("color", RuntimeValue::Color(color)) => builder.color(color),
                ("stroke_color", RuntimeValue::Color(color)) => builder.stroke_color(color),
                ("stroke_weight", RuntimeValue::Number(number)) => builder.stroke_weight(number),

                ("hit_note", RuntimeValue::Note(note)) => builder.hit_note(note),
                ("hit_note_volume", RuntimeValue::Number(number)) => builder.hit_note_volume(number),

                ("trail", RuntimeValue::Number(number)) => builder.frames_per_trail_obj(number),

                ("update", RuntimeValue::Function(Function { name, parameters, body, declaration_env })) => builder.update(name, parameters, body, declaration_env),
                ("hit", RuntimeValue::Function(Function { name, parameters, body, declaration_env })) => builder.hit(name, parameters, body, declaration_env),

                (key, value) => builder.other(key.to_string(), value)
            }
        }

        builder.build(physics)
    }

    pub fn shape(mut self, shape: ShapeType) -> ObjectBuilder {
        self.shape = shape;
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

    pub fn width(mut self, width: f32) -> ObjectBuilder {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> ObjectBuilder {
        self.height = height;
        self
    }

    pub fn size(self, size: f32) -> ObjectBuilder {
        self.width(size).height(size)
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

    pub fn color(mut self, color: Rgb<u8>) -> ObjectBuilder {
        self.color = color;
        self
    }

    pub fn stroke_color(mut self, stroke_color: Rgb<u8>) -> ObjectBuilder {
        self.stroke_color = stroke_color;
        self
    }

    pub fn stroke_weight(mut self, stroke_weight: f32) -> ObjectBuilder {
        self.stroke_weight = stroke_weight;
        self
    }
    
    pub fn hit_note(mut self, hit_note: String) -> ObjectBuilder {
        self.hit_note = hit_note;
        self
    }

    pub fn hit_note_volume(mut self, hit_note_volume: f32) -> ObjectBuilder {
        self.hit_note_volume = hit_note_volume;
        self
    }

    pub fn frames_per_trail_obj(mut self, frames_per_trail_obj: f32) -> ObjectBuilder {
        self.frames_per_trail_obj = Some(frames_per_trail_obj as u128);
        self
    }

    pub fn update(mut self, name: String, parameters: Vec<String>, body: Vec<Statement>, declaration_env: Environment) -> ObjectBuilder {
        self.update_fn = Some(Function::new(name, parameters, body, declaration_env));
        self
    }

    pub fn hit(mut self, name: String, parameters: Vec<String>, body: Vec<Statement>, declaration_env: Environment) -> ObjectBuilder {
        self.hit_fn = Some(Function::new(name, parameters, body, declaration_env));
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

            color: self.color,
            stroke_color: self.stroke_color,
            stroke_weight: self.stroke_weight,

            hit_note: self.hit_note,
            hit_note_volume: self.hit_note_volume,

            frames_per_trail_obj: self.frames_per_trail_obj,
            trail_objs: vec![],

            bounciness: self.bounciness,

            update_fn: self.update_fn,
            hit_fn: self.hit_fn,

            handle,

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

    color: Rgb<u8>,
    stroke_color: Rgb<u8>,
    stroke_weight: f32,

    hit_note: String,
    hit_note_volume: f32,

    frames_per_trail_obj: Option<u128>,
    trail_objs: Vec<(Translation<f32>, f32, Object)>,

    update_fn: Option<Function>,
    hit_fn: Option<Function>,

    handle: RigidBodyHandle,

    others: HashMap<String, RuntimeValue>
}

impl Object {
    pub fn update(&mut self, physics: &mut Physics, elapsed_frames: u128) {
        if let Some(frames_req) = self.frames_per_trail_obj {
            if elapsed_frames / frames_req > self.trail_objs.len() as u128 {
                let (pos, rot) = self.get_pos_and_rot(physics);
                let mut obj_clone = self.clone();
                obj_clone.trail_objs = vec![];

                self.trail_objs.push((pos, rot, obj_clone));
            }
        }

        let object_map = self.to_map(physics);
        let object = RuntimeValue::Object(object_map);

        let func = match &mut self.update_fn {
            Some(func) => func,
            None => return
        };

        let new_map = match eval_runtime_object_expr(object, func) {
            RuntimeValue::Object(map) => map,
            _ => panic!("Invalid object")
        };

        self.update_map(new_map, physics);
    }

    pub fn hit(&mut self, assets_path: &Path, physics: &mut Physics, audio_stream: &mut nannou_audio::Stream<Audio>) {
        let mut note_path = assets_path.join("notes");
        note_path.push(self.hit_note.clone() + ".wav");

        let note = audrey::open(note_path).expect("Failed to load sound");
        let volume = self.hit_note_volume;

        audio_stream.send(move |audio| audio.play_note(note, volume)).expect("Failed to send to audio stream");
        // audio_stream.send(move |audio| audio.add_note(hit_note)).expect("Failed to send note to audio stream");

        let object_map = self.to_map(physics);
        let object = RuntimeValue::Object(object_map);

        let func = match &mut self.hit_fn {
            Some(func) => func,
            None => return
        };

        let new_map = match eval_runtime_object_expr(object, func) {
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

        map.insert("shape".to_string(), RuntimeValue::Shape(self.shape));

        map.insert("x".to_string(), RuntimeValue::Number(pos.x));
        map.insert("y".to_string(), RuntimeValue::Number(pos.y));

        map.insert("width".to_string(), RuntimeValue::Number(self.width));
        map.insert("height".to_string(), RuntimeValue::Number(self.height));
        map.insert("size".to_string(), RuntimeValue::Number(self.width));

        map.insert("gravity".to_string(), RuntimeValue::Number(gravity));
        map.insert("bounciness".to_string(), RuntimeValue::Number(self.bounciness));

        map.insert("color".to_string(), RuntimeValue::Color(self.color));
        map.insert("stroke_color".to_string(), RuntimeValue::Color(self.stroke_color));
        map.insert("stroke_weight".to_string(), RuntimeValue::Number(self.stroke_weight));

        map.insert("hit_note".to_string(), RuntimeValue::Note(self.hit_note.clone()));
        map.insert("hit_note_volume".to_string(), RuntimeValue::Number(self.hit_note_volume));

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
                ("shape", RuntimeValue::Shape(shape)) => self.shape = shape,

                ("x", RuntimeValue::Number(number)) => pos.translation.x = number,
                ("y", RuntimeValue::Number(number)) => pos.translation.y = number,

                ("width", RuntimeValue::Number(number)) => self.width = number,
                ("height", RuntimeValue::Number(number)) => self.height = number,
                ("size", RuntimeValue::Number(number)) => {
                    self.width = number;
                    self.height = number;
                },

                ("gravity", RuntimeValue::Number(number)) => rigidbody.set_gravity_scale(number, wake_up),
                ("bounciness", RuntimeValue::Number(number)) => self.bounciness = number,

                ("color", RuntimeValue::Color(color)) => self.color = color,
                ("stroke_color", RuntimeValue::Color(color)) => self.stroke_color = color,
                ("stroke_weight", RuntimeValue::Number(number)) => self.stroke_weight = number,

                ("hit_note", RuntimeValue::Note(note)) => self.hit_note = note,
                ("hit_note_volume", RuntimeValue::Number(number)) => self.hit_note_volume = number,

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

        physics.remove_colliders(rigidbody.colliders());
        physics.add_collider(self.handle, self.shape, self.bounciness, self.width, self.height, self.stroke_weight);
    }

    pub fn test_collider(&self, physics: &Physics, collider: ColliderHandle) -> bool {
        let rigidbody = match physics.bodies.get(self.handle) {
            Some(rb) => rb,
            None => panic!("Object doesn't have associated handle")
        };

        rigidbody.colliders().contains(&collider)
    }

    pub fn draw(&self, draw: &Draw, physics: &Physics) {
        for (pos, rot, obj) in &self.trail_objs {
            obj.draw_obj(draw, *pos, *rot);
        }

        let (pos, rot) = self.get_pos_and_rot(physics);
        self.draw_obj(draw, pos, rot);
    }

    fn draw_obj(&self, draw: &Draw, pos: Translation<f32>, rot: f32) {
        match self.shape {
            ShapeType::Circle => {
                draw.ellipse()
                    .x_y(pos.x, pos.y)
                    .radius(self.width)
                    .color(self.color)
                    .stroke_color(self.stroke_color)
                    .stroke_weight(self.stroke_weight);
            }
            ShapeType::Rect => {
                draw.rect()
                    .x_y(pos.x, pos.y)
                    .w_h(2.0 * self.width, 2.0 * self.height)
                    .rotate(rot)
                    .color(self.color)
                    .stroke_color(self.stroke_color)
                    .stroke_weight(self.stroke_weight);
            }
            ShapeType::Ring => {
                let points = (0..=360).map(|i| {
                    let radian = deg_to_rad(i as f32);

                    let x = pos.x + radian.sin() * self.width;
                    let y = pos.y + radian.cos() * self.width;

                    (pt2(x, y), self.color)
                });

                draw.polyline()
                    .stroke_weight(self.stroke_weight)
                    .points_colored(points);
            }
        }
    }

    fn get_pos_and_rot(&self, physics: &Physics) -> (Translation<f32>, f32) {
        let rigidbody = match physics.bodies.get(self.handle) {
            Some(rb) => rb,
            None => panic!("Object doesn't have associated handle")
        };
        let pos = rigidbody.position().translation;
        let rot = rigidbody.rotation().simd_to_polar().1;

        (pos, rot)
    }
}
