use crate::QuintId;

#[derive(Debug, Default)]
pub struct QuintIdGenerator {
    counter: QuintId,
}

impl QuintIdGenerator {
    pub fn get_id(&mut self) -> u64 {
        self.next().unwrap()
    }
}

impl Iterator for QuintIdGenerator {
    type Item = QuintId;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        Some(self.counter)
    }
}
