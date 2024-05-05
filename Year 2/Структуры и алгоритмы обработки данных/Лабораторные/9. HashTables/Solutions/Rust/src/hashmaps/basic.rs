pub mod basic {
    pub struct HashNode {
        key: f32,
        value: f32,
        next: Option<Box<HashNode>>,
    }
    pub struct BasicHashMap {
        count: usize,
        capacity: usize,
        table: Box<[Option<Box<HashNode>>]>,
    }

    impl BasicHashMap {
        pub fn new() -> Self {
            Self {
                count: 0,
                capacity: 30,
                table: Box::new([None; 30]),
            }
        }
        fn hasher(&self, value: f32) -> usize {
            (value.to_bits() % self.capacity as u32) as usize
        }
    }
}
