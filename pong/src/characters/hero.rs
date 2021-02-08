use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::{
    transform::Transform,
    timing::Time,
  },
  ecs::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Hero;

pub const HERO_WIDTH: f32 = 16.0;
pub const HERO_HEIGHT: f32 = 16.0

// 第一引数にwolrd(ゲーム画面の上方)
// 第二引数に画像
pub fn init_hero(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  // 配置処理 Amethystが提供しているTransformを使う
  let mut hero_transform = Transform::default();

  // 画像を呼び出す処理
  let sprite_render = SpriteRender::new(sprite_sheet_handle, 0); 

  // 画面上のどこに配置するか
  // 画面の高さに応じてキャラクターの大きさは可変する
  let y = ARENA_HEIGHT / 2.0;
  hero_transform.set_translation_xyz(HERO_WIDTH * 0.5, y, 0.0);

  	// Create a left plank entity.
	world
  .create_entity()
  .with(sprite_render.clone())
  .with(hero_transform)
  .build();
}