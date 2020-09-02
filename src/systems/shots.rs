use crate::components::{
    Int2Ops,Position, Robbo, ShootingDir, GunType,
};
use crate::entities::*;
use crate::game_events::{GameEvents, GameEvent};
use crate::frame_cnt::FrameCnt;

use bevy::prelude::*;
use rand::random;
use std::collections::HashSet;

pub fn shot_system(
    mut commands: Commands,
    mut events: ResMut<GameEvents>,
    frame_cnt: Res<FrameCnt>,
    mut items: Query<&Position>,
    mut shooting_items: Query<(&Position, &ShootingDir)>,
    mut robbo_query: Query<With<Robbo, Entity>>,
) {
    if !frame_cnt.do_shooting() {
        return;
    }
    let mut occupied = HashSet::new();
    for pos in &mut items.iter() {
        occupied.insert(*pos);
    }
    for (pos, dir) in &mut shooting_items.iter() {
        if random::<f32>() >= dir.propability {
            continue;
        }
        let bullet_pos = pos.add(dir);
        if !occupied.contains(&bullet_pos) {
            match dir.gun_type {
                GunType::Solid => {
                    create_laser_head(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
                GunType::Burst | _ => {
                    create_bullet(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
            }
        } else {
            events.send(GameEvent::Damage(bullet_pos, false));
        }
    }
    for entity in &mut robbo_query.iter() {
        commands.remove_one::<ShootingDir>(entity);
    }
}