pub mod print {
    use crate::riot_api::get_from_api::Profile;
    pub fn print_summoner(profile: Profile) {
        println!("**********************************");
        println!("Summoner Name: {}", profile.summoner.name);
        println!("Level: {}", profile.summoner.summoner_level);
        print_rank(&profile);
        print_match_history(&profile);
        println!("**********************************")
    }
    fn print_rank(profile: &Profile) {
        for rank in profile.rank.iter() {
            let total: f64 = (rank.wins + rank.losses).into();
            let wins: f64 = rank.wins.into();
            let wr: f64 = (wins / total) * 100f64;
            println!(
                "{}: {} {} {} LP {:.2}% ({} / {})",
                rank.queue_type, rank.tier, rank.rank, rank.league_points, wr, rank.wins, rank.losses
            );
        }
    }
    fn print_match_history(profile: &Profile) {
        println!("Match History: ");
        for matches in profile.match_history.matches.iter() {
            println!("Lane: {}",
             matches.lane);
        }
    }
}
