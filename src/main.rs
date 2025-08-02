use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResized;
use bevy_svg::prelude::*;
use clap::Parser;

// A resource to store the previous window size
// Assumes a single window
#[derive(Resource, Default)]
struct PreviousWindowSize {
    width: f32,
    height: f32,
}

fn on_resize_system(
    mut prev_size: ResMut<PreviousWindowSize>,
    mut sprites: Query<(&mut Transform, &Svg2d)>,
    mut resize_reader: EventReader<WindowResized>,
    svgs: Res<Assets<Svg>>,
) {
    for e in resize_reader.read() {
        // When resolution is being changed
        println!("previous: {}, {}", prev_size.width, prev_size.height);
        println!("window: {}, {}", e.width, e.height);
        for (sprite_transform, svg2d) in &mut sprites {
            println!("{}", sprite_transform.scale);
            if let Some(svg) = svgs.get(&svg2d.0) {
                println!("{}", svg.size)
            }
        }
        prev_size.width = e.width;
        prev_size.height = e.height;
    }
}

const DEFAULT_INTERNET: &str = "icons/000000/transparent/1x1/delapouite/wireframe-globe.svg";
// icons/000000/transparent/1x1/delapouite/cloud-upload.svg
// icons/000000/transparent/1x1/delapouite/cloud-download.svg
const DEFAULT_WIFI: &str = "icons/000000/transparent/1x1/delapouite/wifi-router.svg";
const DEFAULT_COMPUTER: &str = "icons/000000/transparent/1x1/skoll/pc.svg";
const INITIAL_WIDTH: f32 = 2000.;
const INITIAL_HEIGHT: f32 = 1000.;

/// Configure the application at runtime
#[derive(Parser, Debug, Resource)]
#[command(version, about, long_about = None)]
struct Args {
    /// internet sprite svg path relative to assets folder
    #[arg(short, long, default_value_t = String::from(DEFAULT_INTERNET))]
    internet: String,

    /// computer sprite svg path relative to assets folder
    #[arg(short, long, default_value_t = String::from(DEFAULT_WIFI))]
    wifi: String,

    /// computer sprite svg path relative to assets folder
    #[arg(short, long, default_value_t = String::from(DEFAULT_COMPUTER))]
    computer: String,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, args: Res<Args>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Svg2d(assets.load(&args.internet)),
        Transform { translation: Vec3::new(-INITIAL_WIDTH / 2., INITIAL_HEIGHT / 2., 0.), ..Default::default() },
        Origin::TopLeft,
    ));
    commands.spawn((Svg2d(assets.load(&args.wifi)), Origin::Center));
    commands.spawn((Svg2d(assets.load(&args.computer)), Origin::Center));
}

fn main() {
    App::new()
        .insert_resource(Args::parse())
        .insert_resource(PreviousWindowSize::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Buffering".to_string(),
                    resolution: (INITIAL_WIDTH, INITIAL_HEIGHT).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            bevy_svg::prelude::SvgPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_system)
        .run();
}
