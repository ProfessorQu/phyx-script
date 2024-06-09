use nannou::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum ShapeType {
    Circle,
    Square
}

impl TryFrom<String> for ShapeType {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "circle" => Ok(Self::Circle),
            "square" => Ok(Self::Square),
            _ => Err(format!("{:?} is not a valid shape", value))
        }
    }
}

pub struct ElementBuilder {
    shape: ShapeType,
    size: f32,
    color: Rgb<u8>,
    pos: Vec2,

    dir: Vec2,
    speed: f32,
    gravity: f32
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            shape: ShapeType::Circle,
            size: 10.0,
            color: WHITE,
            dir: Vec2::new(rand::thread_rng().gen(), rand::thread_rng().gen()).normalize(),
            pos: Vec2::new(0.0, 0.0),
            speed: 1.0,
            gravity: 0.0
        }
    }

    pub fn shape(mut self, shape: ShapeType) -> ElementBuilder {
        self.shape = shape;
        self
    }

    pub fn size(mut self, size: f32) -> ElementBuilder {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Rgb<u8>) -> ElementBuilder {
        self.color = color;
        self
    }

    pub fn speed(mut self, speed: f32) -> ElementBuilder {
        self.speed = speed;
        self
    }

    pub fn gravity(mut self, gravity: f32) -> ElementBuilder {
        self.gravity = gravity;
        self
    }

    pub fn build(&self) -> Element {
        Element {
            shape: self.shape,
            size: self.size,
            color: self.color,
            dir: self.dir,
            pos: self.pos,
            speed: self.speed,
            gravity: self.gravity
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Element {
    shape: ShapeType,
    size: f32,
    color: Rgb<u8>,
    pos: Vec2,

    dir: Vec2,
    speed: f32,
    gravity: f32
}

impl Element {
    pub fn update(&mut self) {
        self.pos.x += self.dir.x * self.speed;
        self.pos.y += self.dir.y * self.speed;
    }

    pub fn draw(&self, draw: &Draw) {
        match self.shape {
            ShapeType::Circle => {
                draw.ellipse().xy(self.pos).radius(self.size).color(self.color);
            }
            ShapeType::Square => {
                draw.rect().xy(self.pos).w_h(self.size, self.size).color(self.color);
            }
        }
    }
}
