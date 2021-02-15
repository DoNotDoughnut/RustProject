use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    simplelog::TermLogger::init(log::LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed)?;
    info!("Starting server...");


    Ok(())
}