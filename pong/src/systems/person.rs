use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// systemdesc
#[derive(SystemDesc)]
pub struct PersonSystem;

impl<'s> System<'s> for PersonSystem {
  type SystemData = ();
}