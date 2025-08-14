use quint_evaluator::ir::QuintEx;

///
/// Various helpers (for now)
///
use crate::QuintId;

/// Generates Quint`id` numbers for various IR structs.
/// Uses a simpler iterator for now.
///
/// This is called through through the grammar file.
///
#[derive(Debug, Default)]
pub struct QuintIdGenerator {
    counter: QuintId,
}

impl QuintIdGenerator {
    /// Get the next Quint id
    pub fn get_id(&mut self) -> QuintId {
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

/// Used in grammar to construct QuintApp
pub fn make_quint_app(id: QuintId, opcode: &str, args: Vec<QuintEx>) -> QuintEx {
    QuintEx::QuintApp {
        id,
        opcode: opcode.into(),
        args,
    }
}
