pub struct Region {
    pub positions: Vec<usize>,
    pub color: i8,
    pub first_position: usize,
}

impl Region {
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn score(&self) -> u32 {
        if self.positions.len() < 2 {
            return 0;
        }

        u32::pow((self.positions.len() - 2) as u32, 2)
    }
}
