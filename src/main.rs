mod pokeapi;

use clap::Parser;
use pokeapi::get::get_pokemon;
use rand::Rng;

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

    if args.random {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(1..1010);

        match get_pokemon(id.to_string()).await {
            Ok(res) => println!("{:#?}", res),
            Err(err) => eprintln!("\x1b[1;31m[ERROR]\x1b[0m An error occurred \n{}", err),
        }

        return Ok(());
    }

    if args.id != None {
        match get_pokemon(args.id.unwrap()).await {
            Ok(res) => println!("{:#?}", res),
            Err(err) => eprintln!("\x1b[1;31m[ERROR]\x1b[0m An error occurred \n{}", err),
        }

        return Ok(());
    }

    Ok(())
}
