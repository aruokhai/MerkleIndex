use cli::command::*;
use cli::healthindex::*;
/// The main entry point of the application.

#[tokio::main]
fn main() {

    if let Err(err) = run().await {
        eprintln!("{err}");
    }

}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let new_cli: Cli = Cli::parse();
    let api = OnlineClient::<SubstrateConfig>::from_url(url).await?;
    match &new_cli.command {
        Commands::RegisterProvider => register_provider(new_cli.url.clone(), new_cli.user.clone())?,
    }
    cli_match();

}