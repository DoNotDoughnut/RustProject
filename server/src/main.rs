use anyhow::Result;
use log::info;

const SERVER: &str = "127.0.0.1:12351";

fn main() -> Result<()> {
    simplelog::TermLogger::init(log::LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed)?;
    info!("Starting server...");


    Ok(())
}
