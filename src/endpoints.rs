use std::future::Future;

use reqwest::Method;

use crate::{LeagueClient, models::summoner_v1};



impl LeagueClient {
    /// Returns a handle for accessing [AccountV1](crate::endpoints::AccountV1) endpoints.
    /// # Riot Developer API Reference
    /// <a href="https://developer.riotgames.com/apis#account-v1" target="_blank">`account-v1`</a>
    ///
    /// Note: this method is automatically generated.
    #[inline]
    pub fn summoner_v1(&self) -> SummonerV1 {
        SummonerV1 { base: self }
    }
}


pub struct SummonerV1<'a> {
    base: &'a LeagueClient,
}


impl<'a> SummonerV1<'a> {
    /// get current summoner
    
    pub fn current_summoner(&self) -> impl Future<Output = Result<summoner_v1::Summoner, reqwest::Error>> + 'a {
        let request = self.base.request(Method::GET,"/lol-summoner/v1/current-summoner");
        self.base.execute_val::<summoner_v1::Summoner>(request)
    }
}