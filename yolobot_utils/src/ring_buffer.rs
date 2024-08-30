#![allow(unused)]

pub struct RingBuffer<const N: usize> {
    write: usize,
    read: usize,
    capacity: usize,
    count: usize,
    buffer: [String; N],
}

impl<const N: usize> RingBuffer<const N: usize> {
    fn new() -> Self {
        return Self {
            write: 0,
            read: 0,
            count: 0,
            capacity: N,
            buffer: [String::from(""), N],
        };
    }

    fn push(&self) {}

    fn pop(&self) {}

    fn count(&self) {}

    fn capacity(&self) {}
}

// just go learn const generics god damn it
