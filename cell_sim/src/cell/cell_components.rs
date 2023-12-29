use std::sync::Arc;

use super::cell_base::CellData;

/// Iterates through all the [CellComponent]<T>s and runs them. This will update the
/// componnents too.
pub fn run_components(components: &mut Vec<CellComponent>, data: &mut CellData, dt: f32) {
    // Vector of new components to replace if needed. We need this to avoid mutating the vector of
    // CellComponent while iterating through it.
    let mut replaced: Vec<(CellComponent, usize)> = Vec::with_capacity(components.len() / 4);
    let mut new: Vec<CellComponent> = Vec::new();

    for (counter, component) in components.iter().enumerate() {
        // CellComponent::run will return a new CellComponent if it needs to update itself.
        let result = (component.run)(data, dt);
        if let Some(replaced_component) = result.0 {
            replaced.push((replaced_component, counter))
        }
        if let Some(new_components) = result.1 {
            new.extend(new_components);
        }
    }

    // Replace the old components with the new ones.
    for (component, index) in replaced {
        components[index] = component;
    }
    components.extend(new);
}

pub type CellComponentFn =
    Arc<dyn Fn(&mut CellData, f32) -> (Option<CellComponent>, Option<Vec<CellComponent>>)>;
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
