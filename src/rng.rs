pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng {
            state: seed ^ 0x9E3779B97F4A7C15,
        }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        (self.next_u64() % n as u64) as u32
    }
    pub fn chance(&mut self, num: u32, den: u32) -> bool {
        if den == 0 {
            return false;
        }
        self.below(den) < num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }
    #[test]
    fn below_is_in_range() {
        let mut r = Rng::new(1);
        for _ in 0..1000 {
            assert!(r.below(6) < 6);
        }
        assert_eq!(r.below(0), 0);
    }
}
