use clap::Parser;
use dotenv::from_path;
use plantgo_blockchain::{
    blockchain::block::Blockchain, chat, crypto::crypto_func, types::args::Args,
};

fn main() {
    let arguments = Args::parse();
    let _ = dotenv::dotenv();

    match log4rs::init_file(&arguments.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for Plant Go!"),
        Err(e) => log::error!("Logger couldn't be initialized for Plant Go: {}", e),
    }
    log::info!("Plant Go Initialized!");

    if let Err(err) = from_path(&arguments.dotenv) {
        log::error!("Failed to log .env file: {}", err);
    }

    chat::main();
    crypto_func();
    let mut blockchain = Blockchain::new();
    blockchain.init();
    blockchain.add_new_block();
}
