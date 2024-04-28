use bevy::prelude::*;
use noise::{
    core::perlin::{perlin_2d, perlin_3d},
    permutationtable::PermutationTable,
    utils::PlaneMapBuilder,
    NoiseFn, Perlin,
};

use crate::{BOTTOM, HEIGHT, LEFT, RIGHT, TOP, WIDTH};

pub fn perlin_plugin(app: &mut App) {
    app.add_systems(PostStartup, setup_perlin)
        .add_systems(Update, (run_perlin, wraparound).chain());
}

pub fn setup_perlin(mut com: Commands) {
    let noise = Perlin::new(0);
    com.insert_resource(PerlinObject(noise));
    com.insert_resource(SeqTimer(Timer::from_seconds(0.02, TimerMode::Repeating)));
    com.insert_resource(Seq(0.0));
    com.insert_resource(StartTimer(Timer::from_seconds(3.0, TimerMode::Once)));

    let hasher = PermutationTable::new(rand::random::<u32>());

    // 2d perlin static
    // let f = 1.0;
    // let map = PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
    //     .set_size(WIDTH as usize, HEIGHT as usize)
    //     .set_x_bounds(-1.0 * f, 1.0 * f)
    //     .set_y_bounds(-1.0 * f, 1.0 * f)
    //     .build();

    // for (i, val) in map.iter().enumerate() {
    //     // println!("val: {}", val * 0.5 + 0.5);
    //     if rand::random::<f32>() < 0.01 {
    //         let mag = ((*val + f) / (2.0 * f)) as f32;
    //         // between bright blue to dim red
    //         // let r = 1.0 - magnitude;
    //         // let b = 1.0 - r;

    //         com.spawn(SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::rgba(1.0, mag, 1.0 - 0.5*mag, mag),
    //                 // color: Color::rgba(r, 0., b, magnitude),
    //                 // color: Color::rgba(1., 1., 1., magnitude),
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(
    //                 (i % WIDTH as usize) as f32 - (WIDTH as f32 / 2.0),
    //                 (i / WIDTH as usize) as f32 - (HEIGHT as f32 / 2.0),
    //                 0.0,
    //             )
    //             .with_scale(Vec3::splat(2.0)),
    //             ..default()
    //         });
    //     }
    // }

    // 600 step animation loop
    // 60 fps => 10 seconds
    // keep a frame counter, modulus 600
    let step = 15;
    for y in (0..HEIGHT as i32).step_by(step) {
        for x in (0..WIDTH as i32).step_by(step) {
            // let val = noise.get([x as f64 / (WIDTH * 0.1), y as f64 / (HEIGHT * 0.1)]);
            // let val = ((val + 1.0) / 2.0) as f32;

            com.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    // color: Color::rgba(1.0, 1.0, 1.0, val),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x as f32 - (WIDTH as f32 / 2.0),
                    y as f32 - (HEIGHT as f32 / 2.0),
                    0.0,
                )
                .with_scale(Vec3::splat(2.0)),
                ..default()
            });
        }
    }
}

pub fn run_perlin(
    mut com: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Transform)>,
    noise: Res<PerlinObject>,
    mut timer: ResMut<SeqTimer>,
    mut seq: ResMut<Seq>,
    mut start_timer: ResMut<StartTimer>,
) {
    // every 0.25 seconds update brightness via noise function
    // each noise function run increments sequence by 1/40 for a total of 40 unique frames over 8 seconds
    // restart seq to 0 after hitting 40
    if !start_timer.0.tick(time.delta()).finished() {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        // force applied to all
        let fdx = noise.0.get([seq.0, 0.0]) * 2.0;
        let fdy = noise.0.get([seq.0, 1.0]) * 2.0;
        for (i, (mut sprite, mut transform)) in query.iter_mut().enumerate() {
            let dx = noise
                .0
                .get([transform.translation.x as f64, seq.0, i as f64])
                * 1.5
                + fdx;
            let dy = noise
                .0
                .get([seq.0, transform.translation.y as f64, i as f64])
                * 1.5
                + fdy;

            let x = transform.translation.x;
            let y = transform.translation.y;

            transform.translation.x += dx as f32;
            transform.translation.y += dy as f32;

            let val = noise.0.get([
                x as f64 / (WIDTH * 0.1),
                y as f64 / (HEIGHT * 0.1),
                seq.0 as f64,
            ]) as f32;

            sprite.color = Color::rgba(1.0, 1.0, 1.0, val);
        }

        let dz = 1.0 / 100.0;
        seq.0 += dz;
        if seq.0 >= 1.0 / dz {
            seq.0 = 0.0 + rand::random::<f64>();
        }
    }
}

#[derive(Resource)]
pub struct PerlinObject(pub Perlin);

#[derive(Resource)]
pub struct SeqTimer(Timer);

#[derive(Resource)]
pub struct StartTimer(Timer);

#[derive(Resource)]
pub struct Seq(pub f64);

pub fn wraparound(mut query: Query<&mut Transform>, start_timer: Res<StartTimer>) {
    if !start_timer.0.finished() {
        return;
    }
    for mut transform in query.iter_mut() {
        if transform.translation.y > TOP as f32 {
            transform.translation.y = BOTTOM as f32;
        }
        if transform.translation.y < BOTTOM as f32 {
            transform.translation.y = TOP as f32;
        }
        if transform.translation.x > RIGHT as f32 {
            transform.translation.x = LEFT as f32;
        }
        if transform.translation.x < LEFT as f32 {
            transform.translation.x = RIGHT as f32;
        }
    }
}
