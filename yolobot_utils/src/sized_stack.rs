#![allow(unused)]
use std::collections::VecDeque;

pub struct SizedStack<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    length: usize,
}

impl<T> SizedStack<T> {
    pub fn new(capacity: usize) -> Self {
        return Self {
            buffer: VecDeque::with_capacity(capacity + 1),
            capacity,
            length: 0,
        };
    }

    pub fn push(&mut self, value: T) {
        self.buffer.push_back(value);
        if self.length >= self.capacity {
            self.buffer.pop_front();
        }
        if self.length < self.capacity {
            self.length += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            self.length -= 1;
        }
        return self.buffer.pop_back();
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn buffer(&self) -> &VecDeque<T> {
        return &self.buffer;
    }
}
