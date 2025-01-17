use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
pub struct Args {
    /// A number to specify the 'every' interval.
    #[arg(short, long)]
    pub every: Option<u32>,

    /// A boolean flag for 'once'.
    #[arg(long)]
    pub once: bool,

    /// A name string.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Remaining arguments
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