use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  core::transform::Transform,
  ecs::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Zeus;

impl SimpleState for Zeus {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    let sprite_sheet_handle = load_sprite_sheet(world);

    world.register::<Brave>();

    init_brave(world, sprite_sheet_handle);
    init_camera(world);
  }
}

fn init_camera(world: &mut World) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
    .create_entity()
    .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
    .with(transform)
    .build();
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const BRAVE_HEIGHT: f32 = 4.0;
pub const BRAVE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Position {
  TransX,
  TransY,
}

pub struct Brave {
  pub position: Position,
  pub width: f32,
  pub height: f32,
}

impl Brave {
  fn new(position: Position) -> Brave {
    Brave {
      position,
      width: BRAVE_WIDTH,
      height: BRAVE_HEIGHT,
    }
  }
}

impl Component for Brave {
  type Storage = DenseVecStorage<Self>;
}

fn init_brave(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);
  let mut center_transform = Transform::default();

  // キャラクターの位置
  let y = ARENA_HEIGHT / 2.0;
  let x = ARENA_WIDTH / 2.0;
  center_transform.set_translation_xyz(x, y, 0.0);

  world
    .create_entity()
    .with(sprite_render)
    .with(Brave::new(Position::TransX))
    .with(Brave::new(Position::TransY))
    .with(center_transform)
    .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/brave.png",
      ImageFormat::default(),
      (),
      &texture_storage,
    )
  };

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/brave_spritesheet.ron", // Here we load the associated ron file
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
