pub mod error;
pub mod sized_stack;

#[cfg(test)]
mod tests {
    use std::u64;

    use sized_stack::SizedStack;

    use super::*;

    #[test]
    fn sized_stack_works() {
        let mut stack: SizedStack<u64> = sized_stack::SizedStack::new(3);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.buffer(), &[1, 2, 3]);
        stack.push(4);
        assert_eq!(stack.length(), 3);
        assert_eq!(stack.buffer(), &[2, 3, 4]);
        stack.push(5);
        assert_eq!(stack.length(), 3);
        assert_eq!(stack.buffer(), &[3, 4, 5]);
        let v = stack.pop();
        assert_eq!(stack.length(), 2);
        assert_eq!(v, Some(5));
        assert_eq!(stack.buffer(), &[3, 4]);
    }
}
