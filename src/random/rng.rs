pub trait Rng {
    /// seed the random number generator
    fn seed(seed: u32) -> Self;

    /// get the next random number in the sequence
    fn rand(&mut self) -> u32;

    /// get a u8 value in the range [start, stop)
    fn rand_u8(&mut self, start: u8, stop: u8) -> u8 {
        (self.rand() % ((stop - start) as u32)) as u8 + start
    }

    /// sample randomly from a discrete distribution
    /// the sum of the elements in distribution should be 1
    fn rand_sample_discrete(&mut self, distribution: &Vec<f32>) -> usize {
        let value = self.rand() as f32 / u32::MAX as f32;
        let mut check = 0.0;
        for (index, probability) in distribution.iter().enumerate() {
            check += probability;
            if value <= check {
                return index;
            }
        }

        panic!("Unable to sample distribution");
    }
}
