use clap::Parser;

// Parser takes the cli arguments and parses them to Args type
#[derive(Clone, Debug, Parser)]
// For help text
#[command(about, version)]
pub struct Args {
    /// The path for config.toml file!()
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    /// The path of the environment variable file
    #[arg(short, long, default_value = ".env")]
    pub dotenv: String,

    /// The path of the log file
    #[arg(short, long, default_value = "log_config.yml")]
    pub log_config: String,
}
