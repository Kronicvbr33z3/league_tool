pub mod get_from_api {
    extern crate reqwest;
    use reqwest::Error;
    use serde::{Deserialize, Serialize};

    pub struct Profile {
        pub summoner: Summoner,
        pub rank: Vec<Rank>,
        pub match_history: MatchHistory,
    }
    impl Profile {
        pub fn new_from_name(name: String, api_key: &String) -> Result<Profile, Error> {
            let summoner = Summoner::summoner_from_name(name, &api_key)?;
            let rank = Rank::from_name(&api_key, &summoner.id)?;
            let match_history = MatchHistory::new(&summoner, &api_key)?;
            Ok(Profile {
                summoner: summoner,
                rank: rank,
                match_history: match_history,

            })
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Summoner {
        pub name: String,
        #[serde(alias = "summonerLevel")]
        pub summoner_level: i64,
        pub id: String,
        #[serde(alias = "accountId")]
        pub account_id: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rank {
        #[serde(alias = "queueType")]
        pub queue_type: String,
        #[serde(alias = "summonerName")]
        summoner_name: String,
        #[serde(alias = "hotStreak")]
        pub hot_streak: bool,
        pub wins: i32,
        pub veteran: bool,
        pub losses: i32,
        pub rank: String,
        #[serde(alias = "leagueId")]
        league_id: String,
        inactive: bool,
        #[serde(alias = "freshBlood")]
        fresh_blood: bool,
        pub tier: String,
        #[serde(alias = "summonerId")]
        summoner_id: String,
        #[serde(alias = "leaguePoints")]
        pub league_points: i32,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MatchHistory {
        pub matches: Vec<Match>,
        #[serde(alias="totalGames")]
        total_games: i32,
        #[serde(alias="startIndex")]
        start_index: i32,
        #[serde(alias="endIndex")]
        end_index: i32,

    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Match{
        pub lane: String,
        #[serde(alias="gameId")]
        game_id: i64,
        pub champion: i32,
        #[serde(alias="platformId")]
        platform_id: String,
        season: i32,
        queue: i32,
        role: String,
        timestamp: i64,
    }
    impl Rank {
        fn from_name(api_key: &String, id: &String) -> Result<Vec<Rank>, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/league/v4/entries/by-summoner/{id}?api_key={api_key}", 
            id = id,
            api_key = api_key);

            let mut response = reqwest::get(&request_url)?;
            let vec_rank: Vec<Rank> = response.json()?;
            Ok(vec_rank)
        }
    }
    impl Summoner {
        pub fn summoner_from_name(name: String, api_key: &String) -> Result<Summoner, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{name}?api_key={api_key}", 
            name = name,
            api_key = api_key);

            let mut response = reqwest::get(&request_url)?;
            let summoner: Summoner = response.json()?;
            Ok(summoner)
        }
    }
    impl MatchHistory {
        pub fn new(summoner: &Summoner, api_key: &String) -> Result<MatchHistory, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/match/v4/matchlists/by-account/{account_id}?endIndex=5&api_key={api_key}", 
            account_id = summoner.account_id,
            api_key = api_key);

            let mut response = reqwest::get(&request_url)?;
            let match_history: MatchHistory = response.json()?;
            Ok(match_history)
        }
    }
}
