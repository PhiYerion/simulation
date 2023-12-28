use std::sync::Arc;

use super::cell_base::CellData;

/// Iterates through all the [CellComponent]<T>s and runs them. This will update the
/// componnents too.
pub fn run_components(components: &mut Vec<CellComponent>, data: &mut CellData, dt: f32) {
    // Vector of new components to replace if needed. We need this to avoid mutating the vector of
    // CellComponent while iterating through it.
    let mut new_components: Vec<(CellComponent, usize)> = Vec::with_capacity(components.len() / 4);

    for (counter, component) in components.iter().enumerate() {
        // CellComponent::run will return a new CellComponent if it needs to update itself.
        if let Some(new_component) = (component.run)(data, dt) {
            new_components.push((new_component, counter))
        }
    }

    // Replace the old components with the new ones.
    for (component, index) in new_components {
        components[index] = component;
    }
}

pub type CellComponentFn = Arc<dyn Fn(&mut CellData, f32) -> Option<CellComponent>>;
/// A physical component of a [Cell], either in the Membrane or Internal structure.
/// [CellComponent::size] represents the space it takes up in either the membrane or internal
/// structure. [CellComponent::run] is a function that sohuld be called each frame, potentially
/// mutating [CellData].
pub struct CellComponent {
    pub size: f32,
    /// Function that should be called each frame. This function takes in [CellData] and a delta
    /// and returns a [CellComponent] if it needs to update itself. The function will contain the
    /// state of the [CellComponent].
    pub run: CellComponentFn,
}

impl Clone for CellComponent {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            run: self.run.clone(),
        }
    }
}
unsafe impl Send for CellComponent {}
unsafe impl Sync for CellComponent {}
