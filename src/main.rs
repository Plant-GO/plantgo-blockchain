use std::env::args;

use clap::Parser;
use plantgo_blockchain::{blockchain::block::Blockchain, types::args::Args};

fn main() {
    let arguments = Args::parse();
    let _ = dotenv::dotenv();

    match log4rs::init_file(&arguments.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for Plant Go!"),
        Err(e) => log::error!("Logger couldn't be initialized for Plant Go: {}", e),
    }
    log::info!("Plant Go Initialized!");
    let mut blockchain = Blockchain::new();
    blockchain.add_new_block(arguments);
}
