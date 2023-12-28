use super::base_processes::Polysaccharide;
use super::cell_base::{CellComponent, CellData};

fn get_speed_efficiency(size: f32, proteins: f32) -> (f32, f32) {
    // The idea here is that there will be a process that will require a set amount of proteins to
    // construct and will require a set amount of space. proteins/size is the amount of proteins in
    // the process, assuming one unit of size is one process.
    let ratio = proteins / size;

    let efficiency = ratio / (ratio + 1.);
    let speed = size;

    (speed, efficiency)
}

pub fn reduce_polysaccharides_builder(size: f32, proteins: f32) -> CellComponent {
    let (speed, efficiency) = get_speed_efficiency(size, proteins);

    CellComponent {
        size,
        run: Box::new(move |cell: &mut CellData, dt: f32| {
            if let Some(polysaccharide) = cell.base.polysaccharides.get_mut(0) {
                let amount = dt * speed;
                if amount < polysaccharide.amount {
                    polysaccharide.amount -= amount;
                    cell.base.glucose += amount * polysaccharide.complexity * efficiency;
                } else {
                    cell.base.glucose += polysaccharide.amount * efficiency;
                    cell.base.polysaccharides.remove(0);
                }
            }

            None
        })
    }
}

const AMINO_ACID_FROM_GLYCOLYSIS: f32 = 0.1;
pub fn burn_glucose_builder(size: f32, proteins: f32) -> CellComponent {
    let (speed, efficiency) = get_speed_efficiency(size, proteins);

    CellComponent {
        size,
        run: Box::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = dt * speed;

            if cell.base.glucose < amount {
                amount = cell.base.glucose;
            }

            cell.base.glucose -= amount;
            cell.base.atp += amount * efficiency;
            cell.base.amino_acids += amount * AMINO_ACID_FROM_GLYCOLYSIS;

            None
        })
    }
}

const POLYSACCHARIDE_ATP_COST: f32 = 0.01;
pub fn create_polysaccharides_builder(size: f32, proteins: f32) -> CellComponent {
    let (speed, efficiency) = get_speed_efficiency(size, proteins);

    CellComponent {
        size,
        run: Box::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = dt * speed;

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
        })
    }
}

pub fn create_proteins_builder(size: f32, proteins: f32) -> CellComponent {
    let (speed, efficiency) = get_speed_efficiency(size, proteins);

    CellComponent {
        size,
        run: Box::new(move |cell: &mut CellData, dt: f32| {
            let mut amount = dt * speed;

            if cell.base.amino_acids < amount {
                amount = cell.base.amino_acids;
            }

            cell.base.amino_acids -= amount;
            cell.base.proteins += amount * efficiency;

            None
        })
    }
}
