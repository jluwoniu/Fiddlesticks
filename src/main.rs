use fiddlesticks::LeagueClient;




#[tokio::main]
async fn main() {
    let league_client = LeagueClient::new().unwrap();
    let summoner = league_client.summoner_v1().current_summoner().await;
    println!("{:?}",summoner);
}