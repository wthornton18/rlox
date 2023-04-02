use std::ops::Index;

use crate::value::Value;

#[derive(Default, Debug, Clone)]
pub struct Constants(Vec<Value>);

impl Constants {
    pub fn push(&mut self, val: Value) -> usize {
        self.0.push(val);
        self.0.len() - 1
    }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Index<usize> for Constants {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
