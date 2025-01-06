use crate::types::Result;

pub fn run(result: Result) {
    println!("Value: {:.2}, message: {}", result.value, result.message);
}
