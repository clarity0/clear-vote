#[derive(Debug)]
pub struct Preference {
    pub permutation: Vec<u8>,
    pub number: u64,
}

impl Preference {
    pub fn winner_count(&self) -> (u8, u64) {
        (self.permutation[0], self.number)
    }
}