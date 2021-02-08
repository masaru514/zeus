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

// キャラクターデータ
mod characters;

// 構造体をこちらで管理
#[derive(Default)]
pub struct Pong {
	//　開始前タイマー
	ball_spawn_timer: Option<f32>,
	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
	//ゲームスタート時の構造
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		// world作成
		let world = data.world;

		//発火まで１秒を待つ
		self.ball_spawn_timer.replace(1.0);

		// 画像の読み込み
		let sprite_sheet_handle = load_sprite_sheet(world);
		self.sprite_sheet_handle.replace(load_sprite_sheet(world));

		world.register::<Ball>();
		// パドルの作成
		init_paddles(world, sprite_sheet_handle);

		// キャラクターの作成
		characters::init_hero(world);

		// カメラの状態作成
		init_camera(world);
	}

	// 起動後すぐの挙動 一度だけ発火される
	fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
		if let Some(mut timer) = self.ball_spawn_timer.take() {
			// If the timer isn't expired yet, subtract the time that passed since the last update.
			{
					let time = data.world.fetch::<Time>();
					timer -= time.delta_seconds();
			}
			if timer <= 0.0 {
					// When timer expire, spawn the ball
					// ボール作成のタイミングがupdate後にずれる もともとはstartに
					init_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
			} else {
					// If timer is not expired yet, put it back onto the state.
					self.ball_spawn_timer.replace(timer);
			}
		}
    Trans::None
	}
}

// カメラの再生可能領域
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

// カメラの関数
fn init_camera(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
	world
			.create_entity()
			.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
			.with(transform)
			.build();
}

// パドルの作成
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one paddle on the left, and one paddle on the right.
fn init_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	let mut left_transform = Transform::default();
	let mut right_transform = Transform::default();

	let sprite_render = SpriteRender::new(sprite_sheet_handle, 0); 

	// Correctly position the paddles.
	let y = ARENA_HEIGHT / 2.0;
	left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
	right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

	// Create a left plank entity.
	world
			.create_entity()
			.with(sprite_render.clone())
			.with(Paddle::new(Side::Left))
			.with(left_transform)
			.build();

	// Create right plank entity.
	world
			.create_entity()
			.with(sprite_render)
			.with(Paddle::new(Side::Right))
			.with(right_transform)
			.build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
		// `texture_handle` is a cloneable reference to the texture
		
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let texture_storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load(
				"texture/pong_spritesheet.png",
				ImageFormat::default(),
				(),
				&texture_storage,
		)
	};

	let loader = world.read_resource::<Loader>();
	let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
			"texture/pong_spritesheet.ron", // Here we load the associated ron file
			SpriteSheetFormat(texture_handle),
			(),
			&sprite_sheet_store,
	)
}

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
	pub velocity: [f32; 2],
	pub radius: f32,
}

impl Component for Ball {
	type Storage = DenseVecStorage<Self>;
}

fn init_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
	// Create the translation.
	let mut local_transform = Transform::default();
	local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

	// Assign the sprite for the ball. The ball is the second sprite in the sheet.
	let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

	world
			.create_entity()
			.with(sprite_render)
			.with(Ball {
					radius: BALL_RADIUS,
					velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
			})
			.with(local_transform)
			.build();
}