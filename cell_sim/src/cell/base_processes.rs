use super::cell_internals::SignalProtein;

pub struct Sensitivity {
    pub index: usize,
    pub weight: f32,
}

pub struct Weight {
    pub index: f32,
    pub range: f32,
    pub base: f32,
    pub sensitivity: Sensitivity,
}

pub struct WeightList {
    weights: Vec<Weight>,
}
impl WeightList {
    fn new(mut weights: Vec<Weight>) -> Self {
        weights.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
        Self { weights }
    }
    fn append(&mut self, weight: Weight) {
        self.weights.binary_search_by(|a| a.index.partial_cmp(&weight.index).unwrap())
            .map(|index| self.weights.insert(index, weight));
        self.weights.push(weight);
    }
    fn extend(&mut self, mut weights: Vec<Weight>) {
        weights.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
        self.weights.extend(weights);
    }
    fn remove(&mut self, index: usize) {
        self.weights.remove(index);
    }
    fn get(&self) -> &Vec<Weight> {
        &self.weights
    }
}

fn get_split_vals<'a>(
    cell_size: f32,
    signal_proteins: &mut Vec<SignalProtein>,
    weights: &Vec<Weight>,
    container: &'a mut [f32],
) -> &'a mut [f32] {
    let start = weights.first().unwrap().index;
    let end = {
        weights.iter().fold(start, |acc, weight| {
            let weight_end = weight.index + weight.range;
            match acc > weight_end {
                true => acc,
                false => weight_end,
            }
        })
    };

    let frame_width = (end - start) / container.len() as f32;

    // This can be further optimized by finding the start index of the next frame during
    // calculation of last frame. We can also stop the first and last binary search.
    container
        .iter_mut()
        .enumerate()
        .for_each(|(out_index, out_weight)| {
            let frame_start = start + out_index as f32 * frame_width;
            let frame_end = frame_start + frame_width;
            let weight_start_index = weights
                .binary_search_by(|weight| weight.index.partial_cmp(&frame_start).unwrap())
                .unwrap_or_else(|index| index - 1);
            for weight in weights.iter().skip(weight_start_index) {
                if weight.index > frame_end {
                    break;
                }
                let sensitivity_magnitude = match signal_proteins.get(weight.sensitivity.index) {
                    Some(signal_protein) => signal_protein.strength(cell_size) * weight.sensitivity.weight,
                    None => 0.,
                };
                let total_magnitude = f32::tanh(weight.base + sensitivity_magnitude);
                *out_weight = total_magnitude;
            }
        });

    container
}
