use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub every: Option<u32>,

    #[arg(long)]
    pub once: bool,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(last = true)]
    pub remaining_args: Vec<OsString>,
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub n: u64,
    pub once: bool,
    pub function: String,
    pub args: String,
    pub short_name: String
}

#[derive(Debug, Clone)]
pub struct WorkerResult {
    pub value: f64,
    pub units: Option<String>,
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
            units: None,
            message: "".to_string(),
            graph_value: Some(0),
            graph_type: Some("g".to_string()),
            graph_name: Some("".to_string()),
            graph_short_name: None
        } 
    }
}
