use std::{cmp::Ordering, env, fs};

use crate::{frontend::Parser, runtime::{evaluate, Environment, RuntimeValue}, simulation::ObjectBuilder};

use super::{physics::Physics, Audio, Object};

use nannou::{prelude::*, winit::window::Icon};
use nannou_audio::Buffer;

static FPS: u128 = 60;
static SECS_PER_FRAME: u128 = 1_000_000 / FPS;

pub struct Model {
    num_updates: u128,
    physics: Physics,
    objects: Vec<Object>,
    audio_stream: nannou_audio::Stream<Audio>,
    background_color: Rgb<u8>
}

pub fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Less => panic!("Please input a file to run"),
        Ordering::Greater => panic!("Too many arguments"),
        Ordering::Equal => ()
    }

    let filename = &args[1];
    let title = "Phyx - ".to_string() + filename.split('/').last().expect("Filename is empty");
    app.main_window().set_title(title.as_str());

    app.main_window().set_maximized(true);

    let decoder = png::Decoder::new(fs::File::open("assets/icon.png").unwrap());
    let mut reader = decoder.read_info().expect("Failed to read info of icon");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Failed to read the next frame");
    let bytes = &buf[..info.buffer_size()].to_vec();
    let icon = Icon::from_rgba(bytes.clone(), 180, 180).expect("Failed to create icon");
    app.main_window().set_window_icon(Some(icon));

    let code = fs::read_to_string(filename).expect("Failed to read file");
    let mut parser = Parser::new();
    let mut global_env = Environment::new_global();

    let ast = parser.produce_ast(code);
    evaluate(ast, &mut global_env);

    let mut physics = Physics::new();
    let values = match global_env.lookup_var("objects".to_string()) {
        RuntimeValue::Objects(objects) => objects,
        _ => panic!("Invalid 'objects'")
    };

    let mut objects = vec![];
    add_objects(&values, &mut objects, &mut physics);

    let background_color = match global_env.lookup_var("background_color".to_string()) {
        RuntimeValue::Color(color) => color,
        value => panic!("Invalid value for background: {:?}", value)
    };

    let audio = Audio::new();

    let audio_host = nannou_audio::Host::new();
    let stream = audio_host
        .new_output_stream(audio)
        .render(play_audio)
        .build()
        .expect("Failed to build stream");

    stream.play().expect("Failed to play");

    Model {
        num_updates: 0,
        physics,
        objects,
        audio_stream: stream,
        background_color
    }
}

fn add_objects(values: &Vec<RuntimeValue>, objects: &mut Vec<Object>, physics: &mut Physics) {
    for value in values {
        if let RuntimeValue::Object(object_map) = value {
            objects.push(ObjectBuilder::from_map(object_map.clone(), physics));
        } else if let RuntimeValue::Objects(values) = value {
            add_objects(values, objects, physics);
        } else {
            panic!("Not an object: {:?}", value);
        }
    }
}

fn play_audio(audio: &mut Audio, buffer: &mut Buffer) {
    if let Some(note) = audio.get_note() {
        for frame in buffer.frames_mut() {
            for channel in frame {
                *channel = note;
            }
        }
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let elapsed_frames = app.duration.since_start.as_nanos() / SECS_PER_FRAME;
    if elapsed_frames < model.num_updates {
        return
    }

    for object in &mut model.objects {
        object.update(&mut model.physics, model.num_updates);
    }

    let collisions = model.physics.step();

    for (collider1, collider2) in collisions {
        for object in &mut model.objects {
            if object.test_collider(&model.physics, collider1) || object.test_collider(&model.physics, collider2) {
                model.audio_stream.send(|audio| audio.add_note(1.0)).unwrap();
                object.hit(&mut model.physics);
            }
        }
    }

    model.num_updates += 1;
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.background_color);

    for object in &model.objects {
        object.draw(&draw, &model.physics);
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}
