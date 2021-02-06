use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// 構造体をこちらで管理
pub struct Pong;
impl SimpleState for Pong {
	//ゲームスタート時の構造
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		// world作成
		let world = data.world;

		// 画像の読み込み
		let sprite_sheet_handle = load_sprite_sheet(world);

		// パドルを読み込む前にストレージの設定
		// world.register::<Paddle>();

		// パドルの作成
		init_paddles(world, sprite_sheet_handle);

		// カメラの状態作成
		init_camera(world);
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
