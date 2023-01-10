pub trait Rng {
    /// seed the random number generator
    fn seed(seed: u32) -> Self;

    /// get the next random number in the sequence
    fn rand(&mut self) -> u32;

    /// get a u8 value in the range [start, stop)
    fn rand_u8(&mut self, start: u8, stop: u8) -> u8 {
        (self.rand() % ((stop - start) as u32)) as u8 + start
    }

    /// get a u16 value in the range [start, stop)
    fn rand_u16(&mut self, start: u16, stop: u16) -> u16 {
        (self.rand() % ((stop - start) as u32)) as u16 + start
    }

    /// get a u32 value in the range [start, stop)
    fn rand_u32(&mut self, start: u32, stop: u32) -> u32 {
        (self.rand() % ((stop - start) as u32)) as u32 + start
    }

    /// get a i8 value in the range [start, stop)
    fn rand_i8(&mut self, start: i8, stop: i8) -> i8 {
        // need to cast down to i16 b/c the position could be larger than i8
        // -- can hold
        let rand_pos =
            (self.rand() % (stop as i16 - start as i16) as u32) as i16;
        (rand_pos + start as i16) as i8
    }

    /// get a i16 value in the range [start, stop)
    fn rand_i16(&mut self, start: i16, stop: i16) -> i16 {
        // need to cast down to i32 b/c the position could be larger than i16
        // -- can hold
        let rand_pos =
            (self.rand() % (stop as i32 - start as i32) as u32) as i32;
        (rand_pos + start as i32) as i16
    }

    /// get a i32 value in the range [start, stop)
    fn rand_i32(&mut self, start: i32, stop: i32) -> i32 {
        // need to cast down to i16 b/c the position could be larger than i8
        // -- can hold
        let rand_pos =
            (self.rand() % (stop as i64 - start as i64) as u32) as i64;
        (rand_pos + start as i64) as i32
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
