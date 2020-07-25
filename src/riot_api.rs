

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
        pub async fn new_from_name(name: String, api_key: &String) -> Result<Profile, Error> {
            let summoner = Summoner::summoner_from_name(name, &api_key).await?;
            let rank = Rank::from_name(&api_key, &summoner.id).await?;
            let mut match_history = MatchHistory::new(&summoner, &api_key).await?;

            for current_match in match_history.matches.iter_mut() {
                
               println!("{:?}", current_match.match_info.profile_participant_id = current_match.match_info.get_participant_id(&summoner.account_id).unwrap() );
            }

            Ok(Profile {
                summoner: summoner,
                rank: rank,
                match_history: match_history,

            })
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Summoner {
        pub name: String,
        pub summoner_level: i64,
        pub id: String,
        pub account_id: String,
    }
    impl Summoner {
        pub async fn summoner_from_name(name: String, api_key: &String) -> Result<Summoner, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{name}?api_key={api_key}", 
            name = name,
            api_key = api_key);

            let mut response = reqwest::get(&request_url).await?;
            let summoner: Summoner = response.json().await?;
            Ok(summoner)
        }
    }
 
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Rank {
        pub queue_type: String,
        summoner_name: String,
        pub hot_streak: bool,
        pub wins: i32,
        pub veteran: bool,
        pub losses: i32,
        pub rank: String,
        league_id: String,
        inactive: bool,
        fresh_blood: bool,
        pub tier: String,
        summoner_id: String,
        pub league_points: i32,
    }
    impl Rank {
        async fn from_name(api_key: &String, id: &String) -> Result<Vec<Rank>, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/league/v4/entries/by-summoner/{id}?api_key={api_key}", 
            id = id,
            api_key = api_key);

            let mut response = reqwest::get(&request_url).await?;
            let vec_rank: Vec<Rank> = response.json().await?;
            Ok(vec_rank)
        }
    }
 
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct MatchHistory {
        pub matches: Vec<Match>,
        total_games: i32,
        start_index: i32,
        end_index: i32,

    }
    impl MatchHistory {
        pub async fn new(summoner: &Summoner, api_key: &String) -> Result<MatchHistory, Error> {
            let request_url = format!("https://na1.api.riotgames.com/lol/match/v4/matchlists/by-account/{account_id}?endIndex=5&api_key={api_key}", 
            account_id = summoner.account_id,
            api_key = api_key);

            let response = reqwest::get(&request_url).await?;
            let mut match_history: MatchHistory = response.json().await?;
            for current_match in match_history.matches.iter_mut() {
                current_match.match_info = Match::get_match_info(&current_match.game_id, api_key).await?;
            }
            Ok(match_history)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Match{
        #[serde(skip)]
        pub match_info: MatchInfo,
        pub lane: String,
        game_id: i64,
        pub champion: i32,
        platform_id: String,
        season: i32,
        queue: i32,
        role: String,
        timestamp: i64,
    }
    impl Match {
        async fn get_match_info(game_id: &i64, api_key: &String) -> Result<MatchInfo, Error> {
            let request_url: String = format!("https://na1.api.riotgames.com/lol/match/v4/matches/{game_id}?api_key={api_key}",
             game_id = game_id, 
             api_key = api_key);
            
             let response = reqwest::get(&request_url).await?;
             let match_info: MatchInfo = response.json().await?;
            Ok(match_info)
        }
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MatchInfo {
        game_id: i64,
        game_duration: i64,
        queue_id: i32,
        participant_identities: Vec<ParticipantIdentity>,
        participants: Vec<Participant>,
        #[serde(skip)]
        pub profile_participant_id: i32,
        
    }
   impl MatchInfo {
       fn get_participant_id(&self, account_id: &String) -> Result<i32, Error> {
        let mut pid: i32 = 0;
           for participant in self.participant_identities.iter() {
               match &participant.player.account_id == account_id{
                   true => {
                       println!("{}", participant.player.account_id);
                       println!("{}", account_id);
                       //println!("{}", participant.participant_id);
                       pid = participant.participant_id },
                   _ => pid = -1,
               }
           }
           Ok(pid)
       }
       fn get_player_kills(&self, participant_id: i32) -> Result<i32, Error> {
        let mut kills: i32 = 0;
        for participant in self.participants.iter() {
            if participant.participant_id == participant_id {
                kills = participant.stats.kills;
            } else {
                kills = 0;
            }
        }
        Ok(kills)
       }
       fn get_player_deaths(&self, participant_id: i32) -> i32 {
        2
       }
       fn get_player_assists(&self, participant_id: i32) -> i32 {
        2
       }
   }
   #[derive(Serialize, Deserialize, Debug, Default)]
   #[serde(rename_all = "camelCase")]
   struct ParticipantIdentity {
        participant_id: i32,
        player: Player

   }
   #[derive(Serialize, Deserialize, Debug, Default)]
   #[serde(rename_all = "camelCase")]
   struct Player {
        profile_icon: i32,
        account_id: String,
        summoner_id: String,
   }

   #[derive(Serialize, Deserialize, Debug, Default)]
   #[serde(rename_all = "camelCase")]
   struct Participant {
        participant_id: i32,
        champion_id: i32,
        team_id: i32,
        stats: Stats,
        timeline: Timeline,
        spell_1_id: i32,
        spell_2_id: i32,
   }
   #[derive(Serialize, Deserialize, Debug, Default)]
   #[serde(rename_all = "camelCase")]
   struct Stats {
        item_0: i32,
        item_1: i32,
        item_2: i32,
        item_3: i32,
        item_4: i32,
        item_5: i32,
        item_6: i32,
        largest_multi_kill: i32,
        gold_earned: i32,
        champ_level: i32,
        deaths: i32,
        kills: i32,
        assists: i32,
        triple_kills: i32,
        penta_kills: i32,
        total_damage_dealt: i64,
        total_minions_killed: i32,
        win: bool,
        quadra_kills: i32,
   }
   #[derive(Serialize, Deserialize, Debug, Default)]
   #[serde(rename_all = "camelCase")]
   struct Timeline {

   }
}
