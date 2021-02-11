use amethyst::{
  core::{SystemDesc, Transform},
  derive::SystemDesc,
  ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
  input::{InputHandler, StringBindings},
};

use crate::zeus::{Brave, Position, ARENA_HEIGHT, BRAVE_HEIGHT};

#[derive(SystemDesc)]
pub struct BraveSystem;

impl<'s> System<'s> for BraveSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Brave>,
    Read<'s, InputHandler<StringBindings>>,
  );

  fn run(&mut self, (mut transforms, brave, input): Self::SystemData) {
    for (brave, transform) in (&brave, &mut transforms).join() {
      let movement = match brave.position {
        Position::TransX => input.axis_value("brave_x"),
        Position::TransY => input.axis_value("brave_y"),
      };
      if let Some(mv_amount) = movement {
        if mv_amount != 0.0 {
          let center_name = match brave.position {
            Position::TransX => "x",
            Position::TransY => "y",
          };
          println!("Side {:?} moving {}", center_name, mv_amount)
        }
      }
    }
  }
}
