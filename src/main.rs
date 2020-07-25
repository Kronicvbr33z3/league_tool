mod nice_print;
mod riot_api;
use std::error::Error;
use structopt::StructOpt;

//List of Commands
#[derive(StructOpt)]
enum Cli {
    //Champion Summoner Tier List
    #[structopt(name = "summoner")]
    Summoner { name: String },
    Champion {
        //champion: String,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //API key Required for Riot API
    let api_key: String = "RGAPI-88da60fc-f70c-446c-9bde-9217be3db585".to_string();

    match Cli::from_args() {
        Cli::Summoner { name } => {
            let profile = riot_api::get_from_api::Profile::new_from_name(name, &api_key).await?;
            nice_print::print::print_summoner(profile);
        }
        _ => println!("Unsupported Command"),
    }
    Ok(())
}
