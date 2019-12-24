mod nice_print;
mod riot_api;
use std::error::Error;
use structopt::StructOpt;

use async_timer::{Timed};
use async_timer::oneshot::{Oneshot, Timer};

use std::time;


#[derive(StructOpt)]
enum Cli {
    //Champion Summoner Tier List
    #[structopt(name = "summoner")]
    Summoner { name: String },
    Champion {
        //champion: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key: String = "RGAPI-c9b07ca2-4b22-41af-a04c-fab683382356".to_string();

    match Cli::from_args() {
        Cli::Summoner { name } => {
            let profile = riot_api::get_from_api::Profile::new_from_name(name, &api_key)?;

            nice_print::print::print_summoner(profile);
        }
        _ => println!("Unsupported Command"),
    }
    Ok(())
}
