use bevy::prelude::*;

/// Structure defining a cell. See [super::cell_bundle::CellBundle] for how the game sees the cell.
/// Requires [Cell::update] to be called each frame.
///
/// The cell is made up of a [CellComponent]s in the internal or membrane and [CellData].
///
/// The membrane is the casing of the cell and is responsible for interaction with the outside world.
/// For insance, only the membrane can get food from the environment (although this is not enforced).
/// The internal structure is the inside of the cell and is responsible for the cell's internal processes.
/// For instance, the internal structure can digest food and turn it into energy.
///
/// The [CellData] is the data and state of the cell, but does not contain the [CellComponent]s or any logic.
///
/// The [CellComponent]s are the logic of the cell. They are responsible for updating the [CellData] and can
/// mutate themselves. They will weither be in the membrane or internal structure.
#[derive(Component)]
pub struct Cell {
    /// Accelleration the cell can move at
    pub speed: f32,
    pub velocity: Vec2,
    /// Components that make up the internal structure of the cell
    internal_components: Vec<CellComponent>,
    /// Components that make up the membrane of the cell
    membrane_components: Vec<CellComponent>,
    pub data: CellData,
}

/// Represents a [CellComponent] that is either in the membrane or internal structure.
pub enum CellComponentType {
    Internal(CellComponent),
    Membrane(CellComponent),
}

/// Represents the data of the cell. This data is seperarate so that it can more freely be mutated
/// by the [CellComponent]s.
pub struct CellData {
    pub membrane_strength: f32,
    pub membrane_permeability: f32,
    pub food: f32,
    pub food_storage: f32,
    pub food_difficulty: f32,
    pub atp: f32,
    pub atp_storage: f32,
}
/// A physical component of a [Cell], either in the Membrane or Internal structure.
/// [CellComponent::size] represents the space it takes up in either the membrane or internal
/// structure. [CellComponent::run] is a function that sohuld be called each frame, potentially
/// mutating [CellData].
pub struct CellComponent {
    pub size: f32,
    /// Function that should be called each frame. This function takes in [CellData] and a delta
    /// and returns a [CellComponent] if it needs to update itself. The function will contain the
    /// state of the [CellComponent].
    pub run: fn(&mut CellData, f32) -> Option<CellComponent>,
}

impl Cell {
    fn energy_use(&self) -> f32 {
        let size_cost = self.size() * self.size();
        let vel_cost = self.velocity.length_squared();

        size_cost + vel_cost
    }

    pub fn size(&self) -> f32 {
        self.data.food_storage + self.data.atp_storage + self.speed / 4.
    }

    /// Update the cell. This will run all the [InternalComponent]s and [MembraneComponent]s.
    pub fn update(&mut self, dt: f32) {
        self.data.food += self.data.food_storage * 10. * dt;
        self.data.atp -= self.energy_use() * dt;
        run_components(&mut self.internal_components, &mut self.data, dt);
        run_components(&mut self.membrane_components, &mut self.data, dt);
    }
}

/// Iterates through all the [CellComponent]<T>s and runs them. This will update the
/// componnents too.
fn run_components(components: &mut Vec<CellComponent>, state: &mut CellData, dt: f32) {
    // Vector of new components to replace if needed. We need this to avoid mutating the vector of
    // CellComponent while iterating through it.
    let mut new_components: Vec<(CellComponent, usize)> = Vec::with_capacity(components.len() / 4);

    for (counter, component) in components.iter().enumerate() {
        // CellComponent::run will return a new CellComponent if it needs to update itself.
        if let Some(new_component) = (component.run)(state, dt) {
            new_components.push((new_component, counter))
        }
    }

    // Replace the old components with the new ones.
    for (component, index) in new_components {
        components[index] = component;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 1.,
            velocity: Vec2::new(0., 0.),
            internal_components: vec![CellComponent {
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
            data: CellData {
                membrane_strength: 1.,
                membrane_permeability: 1.,
                food: 0.,
                food_storage: 1.,
                food_difficulty: 1.,
                atp: 0.,
                atp_storage: 1.,
            },
        }
    }
}
