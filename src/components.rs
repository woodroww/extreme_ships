use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component, Clone, Copy)]
pub struct MoveDir(pub Vec2);

#[derive(Component)]
pub struct Bullet;

