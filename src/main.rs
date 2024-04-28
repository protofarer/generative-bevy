#![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use fps::*;

mod fps;
mod perlin;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);
const WIDTH: f64 = 2560.;
const HEIGHT: f64 = 1440.;
const LEFT: f64 = WIDTH / -2.;
const RIGHT: f64 = WIDTH / 2.;
const BOTTOM: f64 = HEIGHT / -2.;
const TOP: f64 = HEIGHT / 2.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Generative".to_string(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(ParticleSystemPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (setup_fps_counter, load_assets).chain()) // setup_play here while no scenes impl'd
        .add_systems(
            Update,
            (
                fps_text_update_system,
                fps_counter_showhide,
                bevy::window::close_on_esc,
            ),
        )
        .add_plugins(perlin::perlin_plugin)
        .run();
}

pub fn load_assets(mut com: Commands, assets: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    let particle_texture = assets.load("px.png").into();
    com.insert_resource(ParticleTexture(particle_texture));
    com.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct ParticleTexture(pub Handle<Image>);
