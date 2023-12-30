use crate::cell::weights::{Sensitivity, Weight};

use super::cell_internals::SignalProtein;
use super::component_instances::{register_component_builders, ComponentBuilderProps, RNA};
use super::weights::WeightList;

pub fn build_rna(
    mut weightlist: WeightList,
    cell_size: f32,
    signal_proteins: &Vec<SignalProtein>,
) -> RNA {
    let mut rna = RNA::new();
    let components = register_component_builders();

    // Read the first part of weightlist to find windows
    let weights_start = weightlist.get_val_at(0, cell_size, signal_proteins) as usize - 2;
    weightlist.remove(0);
    let weights_amount = weightlist.get_val_at(0, cell_size, signal_proteins) as usize;
    weightlist.remove(0);

    let raw_weightlist = weightlist.get();

    let (args_raw_weightlist, sensitivities_raw_weightlist) =
        raw_weightlist.split_at(weights_start);

    let args_weightlist = WeightList::new(args_raw_weightlist.to_vec());
    let sensitivities_weightlist = WeightList::new(sensitivities_raw_weightlist.to_vec());

    const ARGS_CHUNK_SIZE: usize = 3;
    let args_size = components.len() * ARGS_CHUNK_SIZE;
    let binding = args_weightlist
        .get_split_vals(cell_size, signal_proteins, args_size);
    let args = binding
        .chunks(ARGS_CHUNK_SIZE);

    const WEIGHT_SIZE: usize = 5;
    let weightlist_arg_size = WEIGHT_SIZE * weights_amount;
    let total_weightlist_arg_size = components.len() * weightlist_arg_size;

    // components.len() chunks of length `weightlist_arg_size`
    let binding = sensitivities_weightlist
        .get_split_vals(cell_size, signal_proteins, total_weightlist_arg_size);
    let sensitivities = binding
        .chunks(weightlist_arg_size);

    for (arg_chunk, sensitivity_chunk) in args.zip(sensitivities) {
        let [activation, size, proteins] = *arg_chunk else { todo!() };
        let collected_sensitivities: Vec<Weight> = sensitivity_chunk
            .chunks(WEIGHT_SIZE)
            .map(|c| (Weight {
                index: *c.first().unwrap_or(&0.),
                base: *c.get(1).unwrap_or(&0.),
                range: *c.get(2).unwrap_or(&0.),
                sensitivity: Sensitivity {
                    index: *c.get(3).unwrap_or(&0.) as usize,
                    weight: *c.get(4).unwrap_or(&0.),
                }}))
            .collect();

        let result = match activation > 0. {
            true => Some(ComponentBuilderProps {
                size,
                proteins,
                weightlist: WeightList::new(collected_sensitivities),
            }),
            false => None
        };

        rna.push(result);
    }

    rna
}
