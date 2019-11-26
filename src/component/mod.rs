use amethyst::{
    core::{
        math::{Point3, Vector3},
    },
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    tiles::Map,
};

use chrono::Duration;
use minterpolate::{linear_interpolate, InterpolationPrimitive};
use std::{ops::Add, time::Instant};

use crate::state::TileMap;

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct MovingObject {
    start_time: Instant,
    duration: Duration,
    start: Vector3<f32>,
    end: Vector3<f32>,
    pub end_p: Position,
}

impl MovingObject {
    pub fn new(duration: Duration, tilemap: &TileMap, s: Position, e: Position) -> Self {
        let start = tilemap.to_world(&s.0, None);
        let end = tilemap.to_world(&e.0, None);
        Self {
            start_time: Instant::now(),
            duration,
            start,
            end,
            end_p: e,
        }
    }

    pub fn interpolate(&self, now: Instant) -> Vector3<f32> {
        if !self.is_done(now) {
            linear_interpolate(
                (now - self.start_time).as_secs_f32(),
                &[0.0, self.duration.to_std().unwrap().as_secs_f32()],
                &[Vec(self.start), Vec(self.end)],
                false,
            )
            .0
        } else {
            self.end
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        self.start_time + self.duration.to_std().unwrap() < now
    }
}

impl Component for MovingObject {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct Position(pub Point3<u32>);

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Position(Point3::new(
            self.0.x + other.0.x,
            self.0.y + other.0.y,
            self.0.z + other.0.z,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec(Vector3<f32>);

impl InterpolationPrimitive for Vec {
    fn add(&self, other: &Self) -> Self {
        Self(Vector3::new(
            self.0.x + other.0.x,
            self.0.y + other.0.y,
            self.0.z + other.0.z,
        ))
    }

    fn sub(&self, other: &Self) -> Self {
        Self(Vector3::new(
            self.0.x - other.0.x,
            self.0.y - other.0.y,
            self.0.z - other.0.z,
        ))
    }

    fn mul(&self, other: f32) -> Self {
        Self(Vector3::new(
            self.0.x * other,
            self.0.y * other,
            self.0.z * other,
        ))
    }

    fn dot(&self, other: &Self) -> f32 {
        (self.0.x * other.0.x) + (self.0.y * other.0.y) + (self.0.z * other.0.z)
    }

    fn magnitude2(&self) -> f32 {
        self.dot(self)
    }
}
