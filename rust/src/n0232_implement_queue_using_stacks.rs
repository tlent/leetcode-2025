#[derive(Default)]
pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(v) = self.input.pop() {
                self.output.push(v);
            }
        }
        self.output.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(v) = self.input.pop() {
                self.output.push(v);
            }
        }
        *self.output.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}
