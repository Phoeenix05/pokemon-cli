mod pokeapi;

use clap::Parser;
use pokeapi::get::get_by_name;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    random: bool,

    #[arg(short, long)]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    if args.name != None {
        let pokemon = get_by_name(args.name.unwrap()).await;
        println!("{:#?}", pokemon)
    }

    Ok(())
}
