use clap::Parser;

use crate::tracing::TracingConfig;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(flatten)]
    pub tracing_config: TracingConfig,
}
