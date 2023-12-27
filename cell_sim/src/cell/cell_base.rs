use bevy::{prelude::*, log};

#[derive(Component)]
pub struct Cell {
    pub speed: f32,
    pub velocity: Vec2,
    internal_components: Vec<InternalComponent>,
    membrane_components: Vec<MembraneComponent>,
    pub membrane: Membrane,
    pub internal: Internal,
}

pub struct InternalComponent {
    pub size: f32,
    pub run: fn(&mut Internal, f32) -> Option<InternalComponent>,
}

pub struct MembraneComponent {
    pub size: f32,
    pub run: fn(&mut Membrane, f32) -> Option<MembraneComponent>,
}

pub struct Membrane {
    pub strength: f32,
    pub permeability: f32,
}

pub struct Internal {
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

        size_cost + vel_cost
    }

    pub fn size(&self) -> f32 {
        self.internal.food_storage + self.internal.atp_storage + self.speed / 4.
    }

    pub fn update(&mut self, dt: f32) {
        self.internal.food += self.internal.food_storage * 10. * dt;
        self.internal.atp -= self.energy_use() * dt;
        log::info!("food: {}", self.internal.food);
        log::info!("atp: {}", self.internal.atp);
        self.run_components(dt);
    }

    fn run_components(&mut self, dt: f32) {
        // TODO: abstract this. Getting the types and traits right will require a refactor. If
        // preformance becomes a concern, we can add an array instead of a vec and use vec as
        // fallover

        // INTERNAL
        let mut new_components: Vec<(InternalComponent, usize)> = Vec::with_capacity(self.internal_components.len() / 4);
        for (counter, component) in self.internal_components.iter().enumerate() {
            if let Some(new_component) = (component.run)(&mut self.internal, dt) {
                new_components.push((new_component, counter))
            }
        }
        for (component, index) in new_components {
            self.internal_components[index] = component;
        }

        // MEMBRANE
        let mut new_components: Vec<(MembraneComponent, usize)> = Vec::with_capacity(self.membrane_components.len() / 4);
        for (counter, component) in self.membrane_components.iter().enumerate() {
            if let Some(new_component) = (component.run)(&mut self.membrane, dt) {
                new_components.push((new_component, counter))
            }
        }
        for (component, index) in new_components {
            self.membrane_components[index] = component;
        }
    }

    pub fn inject_digestor(&mut self) {
        self.internal_components.push(InternalComponent {
            size: 1.,
            run: |internal, dt| {
                if internal.food > 0. {
                    internal.food -= 1. * dt;
                    internal.atp += 1. * dt;
                }
                None
            },
        });
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 1.,
            velocity: Vec2::new(0., 0.),
            internal_components: vec![InternalComponent {
                size: 1.,
                run: |internal, dt| {
                    if internal.food > 0. {
                        internal.food -= 100. * dt;
                        internal.atp += 100. * dt;
                    }
                    None
                },
            }],
            membrane_components: vec![],
            membrane: Membrane {
                strength: 1.,
                permeability: 1.,
            },
            internal: Internal {
                food: 0.,
                food_storage: 1.,
                food_difficulty: 1.,
                atp: 0.,
                atp_storage: 1.,
            },
        }
    }
}
