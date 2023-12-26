use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_xpbd_2d::components::{Collider, RigidBody};

#[derive(Component)]
pub struct Cell {
    pub speed: f32,
    pub velocity: Vec2,
    pub digestion_easiness: f32,
    pub digestion_efficiency: f32,
    pub digestion_rate: f32,
    pub food: f32,
    pub food_storage: f32,
    pub food_difficulty: f32,
    pub atp: f32,
    pub atp_storage: f32,
}

impl Cell {
    pub fn energy_use(&self) -> f32 {
        let size_cost = self.size() * self.size();
        let vel_cost = self.velocity.length_squared();
        let digestion_cost: f32 = if self.food > 0. {
            self.digestion_rate * self.digestion_easiness * self.digestion_efficiency
        } else {
            0.
        };

        size_cost + vel_cost + digestion_cost
    }

    pub fn size(&self) -> f32 {
        self.food_storage
            + self.atp_storage
            + self.speed / 4.
            + self.digestion_easiness / 4.
            + self.digestion_rate / 4.
    }

    pub fn update(&mut self, dt: f32) {
        self.atp -= self.energy_use() * dt;
        let food_digested_cap = self.food_difficulty / self.digestion_easiness * dt;
        let food_digested = food_digested_cap.min(self.food * self.digestion_rate * dt);
        self.food -= food_digested;
        self.atp += food_digested * self.digestion_rate;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 1.,
            velocity: Vec2::new(0., 0.),
            digestion_easiness: 1.,
            digestion_rate: 1.,
            food: 1.,
            food_storage: 1.,
            food_difficulty: 1.,
            atp: 1.,
            atp_storage: 1.,
            digestion_efficiency: 1.,
        }
    }
}

fn bound_circle_pos(pos: &mut Vec3, radius: f32, window: &Window) {
    let min = radius;
    let max = Vec2::new(window.width(), window.height()) - radius;

    if pos.x < min {
        pos.x = min;
    } else if pos.x > max.x {
        pos.x = max.x;
    }

    if pos.y > max.y {
        pos.y = max.y;
    } else if pos.y < min {
        pos.y = min;
    }
}
