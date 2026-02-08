use clap::Parser;
use react_agent::agent::Orchestrator;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let orchestrator = Orchestrator::new(20);
    orchestrator.run(args.prompt).await?;

    Ok(())
}
