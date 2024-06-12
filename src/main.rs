#![allow(dead_code)]

use simulation::{update, model, view};

mod frontend;
mod runtime;
mod simulation;

fn main() -> Result<(), String> {
    nannou::app(model).update(update).simple_window(view).run();

    Ok(())
}
