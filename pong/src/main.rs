use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

// pong.rsからデータを読み込む
// mod = moduleの意
mod pong;
use crate::pong::Pong;

mod systems;

// amethyst お作法
fn main() -> amethyst::Result<()> {
    use amethyst::input::{InputBundle, StringBindings};

    // デバッグをターミナルに出力してくれる
    amethyst::start_logger(Default::default());

    // display.ronの内容を読み取って反映する。　パスを決めている app_rootはお作法
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    // println!("{:?}", app_root);　// ディレクトリルート要素が出力されます。
    let binding_path = config_dir.join("bindings.ron");

    // ユーザーの入力を受け付けるようにする処理
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;


    //ゲームのデータ
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);
        

    // Applicationはゲームエンジンのルートオブジェクト
    // ここに必要なデータを入れる。 他のゲーム実行中に必要なグローバル変数は必要ない
    // Application::new()を追加居、初期状態のAssetのロード、state,GameDataを変数として入れる
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();


    // amethyst お作法
    Ok(())
}

