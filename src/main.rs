mod pokeapi;

use clap::Parser;
use pokeapi::get::get_pokemon;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    random: bool,

    #[arg(long)]
    id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    if args.id != None {
        match get_pokemon(args.id.unwrap()).await {
            Ok(res) => println!("{:#?}", res),
            Err(err) => eprintln!("\x1b[1;31m[ERROR]\x1b[0m An error occurred \n{}", err),
        }
    }

    Ok(())
}
