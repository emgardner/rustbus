use gui::app::run_app;
use iced;
use log::debug;

pub mod controller;
pub mod gui;

// RUST_LOG=rust_bus=debug cargo watch -x run

fn main() -> iced::Result {
    env_logger::init();
    debug!("STARTNG APPLICATION");
    run_app()
}
