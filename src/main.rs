use clap::{Parser, Subcommand};


#[derive(Parser)] // requires `derive` feature
#[clap(author, version)]
#[clap(name = "speed")]
#[clap(about = "a swiss army knife of internet speed tests", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias="cf")]
    #[clap(about = "Run speed test using speed.cloudflare.com (shorthand: cf)", long_about = None)]
    Cloudflare,

    #[clap(alias="nf")]
    #[clap(about = "Run speed test using fast.com (shorthand: fs)", long_about = None)]
    Fast,

    #[clap(alias="os")]
    #[clap(about = "Run speed test using speedtest.net (shorthand: st)", long_about = None)]
    Ookla,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Cloudflare => {
            println!("Running speed test using speed.cloudflare.com")
        }

        Commands::Fast => {
            println!("Running speed test using fast.com")
        }

        Commands::Ookla => {
            println!("Running speed test using speedtest.net")
        }
    }
}