/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */


// @lc code=start
use std::cmp::min;
#[derive(Debug)]
struct MinStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            self.stack.push((val, min(self.stack.last().unwrap().1, val)))
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
// @lc code=end
fn main() {
    let mut stack = MinStack::new();
    stack.push(1);
    stack.pop();
}
