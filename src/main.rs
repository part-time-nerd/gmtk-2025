// https://bevy.org/examples/ui-user-interface/button/

use bevy::{color::palettes::basic::*, color::palettes::css::GOLD, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .add_systems(Update, loop_counter_value_update_system)
        .add_systems(Update, loop_counter_text_update_system)
        .run();
}

#[derive(Component, Default)]
struct LoopCounter(u64);

#[derive(Component)]
struct LoopCounterButton;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    commands.spawn(loop_counter_button(&assets));
    commands.spawn(counter(&assets));
}

fn counter(asset_server: &AssetServer) -> impl Bundle {
    (
        // Create a Text with multiple child spans.
        Text::new("L∞ps: "),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 42.0,
            ..default()
        },
        children![(
            LoopCounter::default(),
            TextSpan::default(),
            TextFont { font: asset_server.load("fonts/FiraMono-Medium.ttf"), font_size: 33.0, ..Default::default() },
            TextColor(GOLD.into()),
        )],
    )
}

fn loop_counter_value_update_system(
    mut loop_counters: Query<&mut LoopCounter>,
    loop_counter_button_interactions: Query<&Interaction, With<LoopCounterButton>>,
) {
    for mut loop_counter in &mut loop_counters {
        for interaction in loop_counter_button_interactions {
            if let Interaction::Pressed = interaction {
                loop_counter.0 += 1
            }
        }
    }
}
fn loop_counter_text_update_system(mut query: Query<(&mut TextSpan, &LoopCounter)>) {
    for (mut span, LoopCounter(loops)) in &mut query {
        **span = format!("{loops}");
    }
}

fn loop_counter_button(asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            LoopCounterButton,
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Button"),
                TextFont { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 33.0, ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}
