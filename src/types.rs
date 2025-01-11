#[derive(Debug, Clone)]
pub struct Config {
    pub n: u64,
    pub function: String,
    pub args: String
}

#[derive(Debug, Clone)]
pub struct WorkerResult {
    pub value: f64,
    pub message: String,
    pub graph_value: Option<u32>,
    pub graph_type: Option<String>,
    pub graph_name: Option<String>,
    pub graph_short_name: Option<String>
}

impl Default for WorkerResult {
    fn default() -> Self {
        WorkerResult { 
            value: 0.0,
            message: "".to_string(),
            graph_value: Some(0),
            graph_type: Some("g".to_string()),
            graph_name: Some("".to_string()),
            graph_short_name: None
        } 
    }
}