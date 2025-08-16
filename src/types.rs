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

    #[arg(short, long)]
    pub worker: Option<String>,

    #[arg(long)]
    pub min_value: Option<f64>,

    #[arg(long)]
    pub max_value: Option<f64>,

    #[arg(last = true)]
    pub remaining_args: Vec<OsString>,

    #[arg(short, long)]
    pub version: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Metric {
    // From Config
    pub n: u64,
    pub once: bool,
    pub function: String,
    pub group: String,
    pub args: String,
    pub short_name: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,

    // From WorkerResult
    pub value: Option<f64>,
    pub units: Option<String>,
    pub message: Option<String>,
    pub graph_value: Option<i64>,
    pub graph_type: Option<String>,
    pub graph_name: Option<String>,
    pub graph_short_name: Option<String>,
}