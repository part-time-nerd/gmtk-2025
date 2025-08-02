use bevy::color::palettes::css::LIGHT_STEEL_BLUE;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_svg::prelude::*;
use clap::Parser;

const SPRITE_WINDOW_RATIO: f32 = 0.2;

fn on_resize_system(
    mut sprites: Query<(&mut Transform, &Svg2d)>,
    mut resize_reader: EventReader<WindowResized>,
    svgs: Res<Assets<Svg>>,
) {
    // Ho
    for e in resize_reader.read() {
        // Assuming a single window and that the sprites are square
        // We scale to the smaller of width/height of the window
        let window_scale = e.width.min(e.height);
        let target_sprite_dimensions = window_scale * SPRITE_WINDOW_RATIO;
        for (mut sprite_transform, svg2d) in &mut sprites {
            if let Some(svg) = svgs.get(&svg2d.0) {
                // Assumes the svg is square. Otherwise this might be a weird scaling.
                sprite_transform.scale = Vec3::from((target_sprite_dimensions / svg.size, 1.0));
                sprite_transform.translation = Vec3::new(
                    sprite_transform.translation.x.signum() * e.width / 2.,
                    sprite_transform.translation.y.signum() * e.height / 2.,
                    0.,
                )
            }
        }
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
    // Assumes the sprites are 512 x 512
    let starting_scale = Vec3::new(
        SPRITE_WINDOW_RATIO * INITIAL_WIDTH.min(INITIAL_HEIGHT) / 512.,
        SPRITE_WINDOW_RATIO * INITIAL_WIDTH.min(INITIAL_HEIGHT) / 512.,
        1.,
    );
    commands.spawn((
        Svg2d(assets.load(&args.internet)),
        Transform {
            translation: Vec3::new(-INITIAL_WIDTH / 2., INITIAL_HEIGHT / 2., 0.),
            scale: starting_scale,
            ..Default::default()
        },
        Origin::TopLeft,
    ));
    commands.spawn((
        Svg2d(assets.load(&args.wifi)),
        Transform {
            translation: Vec3::new(-INITIAL_WIDTH / 2., -INITIAL_HEIGHT / 2., 0.),
            scale: starting_scale,
            ..Default::default()
        },
        Origin::BottomLeft,
    ));
    commands.spawn((
        Svg2d(assets.load(&args.computer)),
        Transform {
            translation: Vec3::new(INITIAL_WIDTH / 2., -INITIAL_HEIGHT / 2., 0.),
            scale: starting_scale,
            ..Default::default()
        },
        Origin::BottomRight,
    ));
}

fn main() {
    App::new()
        .insert_resource(Args::parse())
        .insert_resource(ClearColor(LIGHT_STEEL_BLUE.into()))
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
