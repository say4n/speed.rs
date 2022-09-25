use clap::{Parser, Subcommand};
mod providers;

#[derive(Parser)] // requires `derive` feature
#[clap(author, version)]
#[clap(name = "speed")]
#[clap(about = "a swiss army knife of internet speed tests", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "cf")]
    #[clap(about = "Run speed test using speed.cloudflare.com (shorthand: cf)", long_about = None)]
    Cloudflare,

    #[clap(alias = "nf")]
    #[clap(about = "Run speed test using fast.com (shorthand: fs)", long_about = None)]
    Fast,

    #[clap(alias = "os")]
    #[clap(about = "Run speed test using speedtest.net (shorthand: st)", long_about = None)]
    Ookla,
}

fn main() {
    let cli = Cli::parse();

    print!("Running speed test using ");
    match &cli.command {
        Commands::Cloudflare => {
            println!("speed.cloudflare.com\n");

            providers::cloudflare::get_server_info();
            providers::cloudflare::measure_latency(None);
        }

        Commands::Fast => {
            println!("fast.com\n");
            unimplemented!()
        }

        Commands::Ookla => {
            println!("speedtest.net\n");
            unimplemented!()
        }
    }
}
