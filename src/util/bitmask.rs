pub struct BitMask {
    bits: Vec<u64>,
    pub _length: usize, 
}

impl BitMask {
    pub fn new(size: usize) -> Self {
        let num_chunks = (size + 63) / 64;
        Self {
            bits: vec![0u64; num_chunks],
            _length: size,
        }
    }

    pub fn set(&mut self, index: usize) {
        let chunk = index / 64;
        let pos = index % 64;
        self.bits[chunk] |= 1 << pos;
    }

    pub fn clear(&mut self, index: usize) {
        let chunk = index / 64;
        let pos = index % 64;
        self.bits[chunk] &= !(1 << pos);
    }

    pub fn get(&self, index: usize) -> bool {
        let chunk = index / 64;
        let pos = index % 64;
        (self.bits[chunk] & (1 << pos)) != 0
    }

    pub fn count_active(&self) -> usize {
        self.bits.iter().map(|b| b.count_ones() as usize).sum()
    }
}