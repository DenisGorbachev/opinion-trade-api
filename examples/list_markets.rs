use clap::Parser;
use errgonomic::exit_result;
use opinion_trade_api::{MarketPageRequest, RestClient};
use std::pin::pin;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
struct Command {
    #[arg(long, short, env = "OPINION_API_KEY")]
    key: String,
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CliRunError> {
        let Self {
            key,
        } = self;
        let client = RestClient::new(key);
        let mut request = MarketPageRequest::default();
        let _market_pages = pin!(client.market_pages(&mut request));
        // TODO: Output a JSON line per market via to_string
        Ok(ExitCode::SUCCESS)
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let command = Command::parse();
    let result = command.run().await;
    exit_result(result)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}

#[derive(Error, Debug)]
pub enum CliRunError {}
