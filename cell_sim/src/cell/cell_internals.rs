pub struct CellInternals {
    pub signal_proteins: Vec<SignalProtein>,
    pub atp: f32,
    pub polysaccharides: Vec<Polysaccharide>,
    pub glucose: f32,
    pub proteins: f32,
    pub nucleotides: f32,
    pub amino_acids: f32,
}

impl Default for CellInternals {
    fn default() -> Self {
        Self {
            signal_proteins: Vec::new(),
            polysaccharides: Vec::new(),
            atp: 1.,
            glucose: 0.,
            proteins: 0.,
            nucleotides: 0.,
            amino_acids: 0.,
        }
    }
}

/// [SignalProtein]s are signals passed to [super::cell_base::CellComponent]s. Whether or not these
/// are used is up to the component.
pub struct SignalProtein {
    amount: f32,
}

impl SignalProtein {
    pub fn strength(&self, cell_size: f32) -> f32 {
        self.amount / cell_size
    }
}

/// [Polysaccharide]s are a dense version of glucose, primarily used for long term storage as
/// opoosed to glucose to reduce the size of the cell.
///
/// All saccharides are hydrophilic, thus both [Polysaccharide]s and glucose will increase the size
/// of the cell significantly, but because there is "more saccharides" in this than glucose, the
/// relative effect per unit of energy is less.
pub struct Polysaccharide {
    pub complexity: f32,
    pub amount: f32,
}

const GLUCOSE_SIZE: f32 = 1.;
const PROTEIN_SIZE: f32 = 0.1;
const NUCLEOTIDE_SIZE: f32 = 0.1;
const AMINO_ACID_SIZE: f32 = 0.1;
const SIGNAL_PROTEIN_SIZE: f32 = 0.1;

impl CellInternals {
    pub fn size(&self) -> f32 {
        let mut size = 0.;
        for polysaccharide in &self.polysaccharides {
            size += polysaccharide.amount * GLUCOSE_SIZE;
        }
        for signal_protein in &self.signal_proteins {
            size += signal_protein.amount * SIGNAL_PROTEIN_SIZE;
        }

        size += self.glucose * GLUCOSE_SIZE;
        size += self.proteins * PROTEIN_SIZE;
        size += self.nucleotides * NUCLEOTIDE_SIZE;
        size += self.amino_acids * AMINO_ACID_SIZE;

        size
    }
}
