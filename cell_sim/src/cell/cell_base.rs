use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub speed: f32,
    pub velocity: Vec2,
    pub digestion_easiness: f32,
    pub digestion_efficiency: f32,
    pub digestion_rate: f32,
    pub membrane: Membrane,
    pub internal: Internal,
}

type F = fn(&mut Cell, u8) -> ();
pub struct Membrane {
    pub strength: f32,
    pub permeability: f32,
    pub components: Vec<F>,
}

pub struct Internal {
    pub food: f32,
    pub food_storage: f32,
    pub food_difficulty: f32,
    pub atp: f32,
    pub atp_storage: f32,
    pub components: Vec<F>,
}

impl Cell {
    pub fn energy_use(&self) -> f32 {
        let size_cost = self.size() * self.size();
        let vel_cost = self.velocity.length_squared();
        let digestion_cost: f32 = if self.internal.food > 0. {
            self.digestion_rate * self.digestion_easiness * self.digestion_efficiency
        } else {
            0.
        };

        size_cost + vel_cost + digestion_cost
    }

    pub fn size(&self) -> f32 {
        self.internal.food_storage
            + self.internal.atp_storage
            + self.speed / 4.
            + self.digestion_easiness / 4.
            + self.digestion_rate / 4.
    }

    pub fn update(&mut self, dt: f32) {
        self.internal.food += self.internal.food_storage * 10. * dt;
        self.internal.atp -= self.energy_use() * dt;
        let food_digested_cap = self.internal.food_difficulty / self.digestion_easiness * dt;
        let food_digested = food_digested_cap.min(self.internal.food * self.digestion_rate * dt);
        self.internal.food -= food_digested;
        self.internal.atp += food_digested * self.digestion_rate;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 1.,
            velocity: Vec2::new(0., 0.),
            digestion_easiness: 1.,
            digestion_efficiency: 1.,
            digestion_rate: 1.,
            membrane: Membrane {
                strength: 1.,
                permeability: 1.,
                components: vec![],
            },
            internal: Internal {
                food: 0.,
                food_storage: 1.,
                food_difficulty: 1.,
                atp: 0.,
                atp_storage: 1.,
                components: vec![],
            },
        }
    }
}
