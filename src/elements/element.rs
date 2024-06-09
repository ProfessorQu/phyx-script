use nannou::glam::Vec2;
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
    size: f64,
    dir: Vec2,
    speed: f64,
    gravity: f64
}

impl ElementBuilder {
    pub fn new() -> Self {
        Self {
            shape: ShapeType::Circle,
            size: 10.0,
            dir: Vec2::new(rand::thread_rng().gen(), rand::thread_rng().gen()).normalize(),
            speed: 1.0,
            gravity: 0.0
        }
    }

    pub fn shape(mut self, shape: String) -> ElementBuilder {
        self.shape = shape.try_into().expect("Failed to parse into shape");
        self
    }

    pub fn size(mut self, size: f64) -> ElementBuilder {
        self.size = size;
        self
    }

    pub fn speed(mut self, speed: f64) -> ElementBuilder {
        self.speed = speed;
        self
    }

    pub fn gravity(mut self, gravity: f64) -> ElementBuilder {
        self.gravity = gravity;
        self
    }

    pub fn build(&self) -> Element {
        Element {
            shape: self.shape,
            size: self.size,
            dir: self.dir,
            speed: self.speed,
            gravity: self.gravity
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Element {
    shape: ShapeType,
    size: f64,
    dir: Vec2,
    speed: f64,
    gravity: f64
}

impl Element {

}