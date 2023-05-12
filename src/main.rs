use clap::{Parser, Subcommand};
use indexer::Point;
use space::Metric;

use crate::indexer::OpenAI;

mod indexer;
mod openai;
mod server;
mod vectors;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(short, long)]
        directory: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(short, long, default_value_t = 100)]
        size: usize,
    },
    Embed {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        string: String,
    },
    Compare {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        s1: String,
        #[arg(short, long)]
        s2: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    match args.command {
        Commands::Serve {
            directory,
            port,
            size,
        } => server::serve(directory, port, size).await?,
        Commands::Embed { key, string } => {
            let v = openai::embeddings_for(&key, &[string]).await?;
            eprintln!("{:?}", v);
        },
        Commands::Compare { key, s1, s2 } => {
            let v = openai::embeddings_for(&key, &[s1, s2]).await?;
            let p1 = Point::Mem { vec: Box::new(v[0]) };
            let p2 = Point::Mem { vec: Box::new(v[1]) };
            println!("distance: {}", OpenAI.distance(&p1, &p2));
        }
    }

    Ok(())
}
