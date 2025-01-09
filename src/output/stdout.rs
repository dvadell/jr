use crate::types::WorkerResult;

pub fn run(result: WorkerResult) {
    println!("Value: {:.2}, message: {}", result.value, result.message);
}
