#[derive(Debug)]
pub struct Config {
    pub n: u64,
    pub function: String,
    pub args: String
}

#[derive(Debug, Clone)]
pub struct WorkerResult {
    pub value: f64,
    pub message: String,
}
