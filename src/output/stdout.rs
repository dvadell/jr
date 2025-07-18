use crate::types::{WorkerResult,Config};

pub fn run(result: WorkerResult, _config: Config) {
    println!("Value: {:.2} {:?}, message: {}", result.value, result.units, result.message);
}
