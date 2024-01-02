use bevy::log;

use super::cell_internals::SignalProtein;

#[derive(Clone, Copy, Debug)]
pub struct Sensitivity {
    pub index: usize,
    pub weight: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Weight {
    pub index: f32,
    pub range: f32,
    pub base: f32,
    pub sensitivity: Sensitivity,
}

#[derive(Clone)]
pub struct WeightList {
    weights: Vec<Weight>,
}

// Vector stuff
impl WeightList {
    pub fn new(mut weights: Vec<Weight>) -> Self {
        weights.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
        if weights.is_empty() {
            panic!("WeightList must have at least one weight");
        }
        Self { weights }
    }
    pub fn append(&mut self, weight: Weight) {
        if let Ok(index) = self
            .weights
            .binary_search_by(|a| a.index.partial_cmp(&weight.index).unwrap())
        {
            self.weights.insert(index, weight)
        }
        self.weights.push(weight);
    }
    pub fn extend(&mut self, mut weights: Vec<Weight>) {
        weights.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
        self.weights.extend(weights);
    }
    pub fn remove(&mut self, index: usize) {
        self.weights.remove(index);
    }
    pub fn get(&self) -> &Vec<Weight> {
        &self.weights
    }
    pub fn get_val_at(
        &self,
        index: usize,
        cell_size: f32,
        signal_proteins: &[SignalProtein],
    ) -> f32 {
        self.weights
            .get(index)
            .map(|weight| {
                let sensitivity_magnitude = match signal_proteins.get(weight.sensitivity.index) {
                    Some(signal_protein) => {
                        signal_protein.strength(cell_size) * weight.sensitivity.weight
                    }
                    None => 0.,
                };

                f32::tanh(weight.base + sensitivity_magnitude)
            })
            .unwrap_or(0.)
    }
}

impl WeightList {
    pub fn get_split_vals(
        &self,
        cell_size: f32,
        signal_proteins: &[SignalProtein],
        amount: usize,
    ) -> Vec<f32> {
        let start = self.weights.first().unwrap().index;
        let end = {
            self.weights.iter().fold(start, |acc, weight| {
                let weight_end = weight.index + weight.range;
                match acc > weight_end {
                    true => acc,
                    false => weight_end,
                }
            })
        };

        let frame_width = (end - start) / amount as f32;

        let mut container = vec![0.; amount];
        // This can be further optimized by finding the start index of the next frame during
        // calculation of last frame. We can also stop the first and last binary search.
        container
            .iter_mut()
            .enumerate()
            .for_each(|(out_index, out_weight)| {
                let frame_start = start + out_index as f32 * frame_width;
                let frame_end = frame_start + frame_width;
                let weight_start_index = self
                    .weights
                    .binary_search_by(|weight| weight.index.partial_cmp(&frame_start).unwrap())
                    .unwrap_or_else(|index| index - 1);
                for weight in self.weights.iter().skip(weight_start_index) {
                    if weight.index > frame_end {
                        break;
                    }
                    let sensitivity_magnitude = match signal_proteins.get(weight.sensitivity.index)
                    {
                        Some(signal_protein) => {
                            signal_protein.strength(cell_size) * weight.sensitivity.weight
                        }
                        None => 0.,
                    };
                    let total_magnitude = f32::tanh(weight.base + sensitivity_magnitude);
                    *out_weight = total_magnitude;
                }
            });

        container
    }
}

impl Default for WeightList {
    fn default() -> Self {
        let weight_size = (rand::random::<f32>() * 1000.) as usize + 100;
        let mut weights = Vec::with_capacity(weight_size);
        for _ in 0..weight_size {
            weights.push(Weight {
                index: rand::random::<f32>() * 100.,
                range: rand::random::<f32>() * 100.,
                base: rand::random::<f32>() * 1000. - 500.,
                sensitivity: Sensitivity {
                    index: (rand::random::<f32>() * 100.) as usize,
                    weight: rand::random::<f32>() * 1000. - 500.,
                },
            });
        }

        Self { weights }
    }
}
