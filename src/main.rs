use fiddlesticks::{models::lobby_v2::NewLobby, LeagueClient};

#[tokio::main]
async fn main() {
    let league_client = LeagueClient::new().unwrap();
    let summoner = league_client.summoner_v1().current_summoner().await;
    let gameflowphase = league_client.gameflow_v1().gameflow_phase().await;
    let new_lobby = NewLobby { queue_id: 830 };
    let lobby = league_client.lobby_v2().create_lobby(&new_lobby).await;
    println!("{:?}", summoner);
    println!("{:?}", gameflowphase);
    println!("{:?}", lobby);
}
