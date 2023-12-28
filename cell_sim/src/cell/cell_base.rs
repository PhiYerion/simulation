use super::cell_components::{CellComponent, run_components};

use bevy::prelude::*;

use super::cell_internals::CellInternals;

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
    /// Components that make up the internal structure of the cell
    pub internal_components: Vec<CellComponent>,
    /// Components that make up the membrane of the cell
    pub membrane_components: Vec<CellComponent>,
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
    /// Accelleration the cell can move at
    pub speed: f32,
    pub base: CellInternals,
    pub velocity: Vec2,
    pub new_cells: Vec<Cell>,
}

impl Cell {
    pub fn size(&self) -> f32 {
        let mut size = self.data.speed / 4. + self.data.base.size();
        for component in &self.internal_components {
            size += component.size;
        }
        for component in &self.membrane_components {
            size += component.size;
        }

        size
    }

    /// Update the cell. This will run all the [InternalComponent]s and [MembraneComponent]s.
    pub fn update(&mut self, dt: f32) {
        run_components(&mut self.internal_components, &mut self.data, dt);
        run_components(&mut self.membrane_components, &mut self.data, dt);
    }

    pub fn inject_component(&mut self, component: CellComponentType) {
        match component {
            CellComponentType::Internal(component) => {
                self.internal_components.push(component);
            }
            CellComponentType::Membrane(component) => {
                self.membrane_components.push(component);
            }
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            internal_components: vec![],
            membrane_components: vec![],
            data: CellData {
                speed: 1.,
                base: CellInternals::default(),
                velocity: Vec2::new(0., 0.),
                new_cells: Vec::new(),
            },
        }
    }
}
