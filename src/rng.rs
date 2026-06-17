pub struct RNG {
    state: u64,
}

impl RNG {
    pub fn new(seed: u64) -> Self {
        RNG { state: seed }
    }
    pub fn next(&mut self) -> u64 {
        let a: u64 = 0x5DEECE66D;
        let c: u64 = 0xB;
        let m: u64 = 1 << 48 - 1;
        self.state = self.state.wrapping_mul(a).wrapping_add(c) & m;
        self.state
    }
    pub fn range(&mut self, min: u64, max: u64) -> u64 {
        let span = max - min;
        min + (self.next() % span)
    }
}
