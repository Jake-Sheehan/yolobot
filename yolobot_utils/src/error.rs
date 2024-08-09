use std::error::Error;

pub fn log<T: Error>(error: T) {
    println!("{}", error);
}
