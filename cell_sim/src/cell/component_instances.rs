use std::sync::Arc;

use super::cell_base::{Cell, CellComponentType, CellData};
use super::cell_components::CellComponent;
use super::cell_internals::{Polysaccharide, SignalProtein};
use bevy::prelude::*;

fn get_speed_efficiency(size: f32, proteins: f32) -> (f32, f32) {
    // The idea here is that there will be a process that will require a set amount of proteins to
    // construct and will require a set amount of space. proteins/size is the amount of proteins in
    // the process, assuming one unit of size is one process.
    let ratio = proteins / size;

    let efficiency = ratio / (ratio + 1.);
    let speed = size;

    (speed, efficiency)
}

fn get_amount(dt: f32, speed: f32, signal_proteins: &[SignalProtein], size: f32, sensitivities: &[(usize, f32)]) -> f32 {
    let mut sensitivity_modifier: f32 = 1.;
    sensitivities.iter().for_each(|(index, weight)| {
        sensitivity_modifier += signal_proteins[*index].strength(size) * weight;
    });
    sensitivity_modifier = f32::tanh(sensitivity_modifier);

    dt * speed * sensitivity_modifier
}

pub fn flagella_builder(props: ComponentBuilderProps) -> CellComponentType {
    let (speed, efficiency) = get_speed_efficiency(props.size, props.proteins);

    CellComponentType::Membrane(CellComponent {
        size: props.size,
        run: Arc::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = get_amount(dt, speed, &cell.base.signal_proteins, cell.size, &props.sensitivities);

            if cell.base.atp < amount * amount {
                amount = cell.base.atp / amount;
            }

            cell.base.atp -= amount * amount * cell.base.size() / 200.;

            let direction = rand::random::<f32>();
            let negative = rand::random::<bool>();
            if negative {
                amount = -amount;
            }
            cell.velocity += Vec2 {
                x: amount * efficiency * direction,
                y: amount * efficiency * (1. - direction),
            };

            None
        }),
    })
}

pub fn reduce_polysaccharides_builder(props: ComponentBuilderProps) -> CellComponentType {
    let (speed, efficiency) = get_speed_efficiency(props.size, props.proteins);

    CellComponentType::Internal(CellComponent {
        size: props.size,
        run: Arc::new(move |cell: &mut CellData, dt: f32| {
            if let Some(polysaccharide) = cell.base.polysaccharides.get_mut(0) {
                let amount = get_amount(dt, speed, &cell.base.signal_proteins, cell.size, &props.sensitivities);
                if amount < polysaccharide.amount {
                    polysaccharide.amount -= amount;
                    cell.base.glucose += amount * polysaccharide.complexity * efficiency;
                } else {
                    cell.base.glucose += polysaccharide.amount * efficiency;
                    cell.base.polysaccharides.remove(0);
                }
            }

            None
        }),
    })
}

const AMINO_ACID_FROM_GLYCOLYSIS: f32 = 0.1;
pub fn burn_glucose_builder(props: ComponentBuilderProps) -> CellComponentType {
    let (speed, efficiency) = get_speed_efficiency(props.size, props.proteins);

    CellComponentType::Internal(CellComponent {
        size: props.size,
        run: Arc::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = get_amount(dt, speed, &cell.base.signal_proteins, cell.size, &props.sensitivities);

            if cell.base.glucose < amount {
                amount = cell.base.glucose;
            }

            cell.base.glucose -= amount;
            cell.base.atp += amount * efficiency;
            cell.base.amino_acids += amount * AMINO_ACID_FROM_GLYCOLYSIS;

            None
        }),
    })
}

const POLYSACCHARIDE_ATP_COST: f32 = 0.01;
pub fn create_polysaccharides_builder(props: ComponentBuilderProps) -> CellComponentType {
    let (speed, efficiency) = get_speed_efficiency(props.size, props.proteins);

    CellComponentType::Internal(CellComponent {
        size: props.size,
        run: Arc::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = get_amount(dt, speed, &cell.base.signal_proteins, cell.size, &props.sensitivities);

            if cell.base.glucose < amount {
                amount = cell.base.glucose;
            }
            if cell.base.atp < amount * POLYSACCHARIDE_ATP_COST {
                amount = cell.base.atp / POLYSACCHARIDE_ATP_COST;
            }

            cell.base.atp -= amount * POLYSACCHARIDE_ATP_COST;
            cell.base.glucose -= amount;
            cell.base.polysaccharides.push(Polysaccharide {
                complexity: 2.,
                amount: amount * efficiency,
            });

            None
        }),
    })
}

pub fn create_proteins_builder(props: ComponentBuilderProps) -> CellComponentType {
    let (speed, efficiency) = get_speed_efficiency(props.size, props.proteins);

    CellComponentType::Internal(CellComponent {
        size: props.size,
        run: Arc::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = get_amount(dt, speed, &cell.base.signal_proteins, cell.size, &props.sensitivities);

            if cell.base.amino_acids < amount {
                amount = cell.base.amino_acids;
            }

            cell.base.amino_acids -= amount;
            cell.base.proteins += amount * efficiency;

            None
        }),
    })
}

pub fn register_component_builders() -> Vec<fn(ComponentBuilderProps) -> CellComponentType> {
    vec![
        flagella_builder,
        burn_glucose_builder,
        create_polysaccharides_builder,
        create_proteins_builder,
        reduce_polysaccharides_builder,
    ]
}

#[derive(Clone)]
pub struct ComponentBuilderProps {
    pub size: f32,
    pub proteins: f32,
    pub sensitivities: Vec<(usize, f32)>,
}

#[allow(clippy::upper_case_acronyms)]
pub type RNA = Vec<Option<ComponentBuilderProps>>;

pub fn create_cell(rna: RNA) -> Cell {
    let components = register_component_builders();
    let mut cell = Cell::default();
    for (i, strand) in rna.iter().enumerate() {
        if let Some(props) = strand {
            cell.inject_component((components[i])(props.clone()));
        }
    }

    cell
}
