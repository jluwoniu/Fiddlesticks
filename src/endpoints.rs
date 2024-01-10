use std::future::Future;

use reqwest::Method;
use serde_json::{Map, Value};

use crate::{LeagueClient, models::{summoner_v1, gameflow_v1::{self, GameflowPhase}, lobby_v2::{self, Lobby}}};



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

    #[inline]
    pub fn gameflow_v1(&self) -> GameflowV1 {
        GameflowV1 { base: self }
    }

    #[inline]
    pub fn lobby_v2(&self) -> LobbyV2 {
        LobbyV2 { base: self }
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

pub struct GameflowV1<'a> {
    base: &'a LeagueClient,
}


impl<'a> GameflowV1<'a> {
    /// get current summoner
    
    pub fn gameflow_phase(&self) -> impl Future<Output = Result<GameflowPhase, reqwest::Error>> + 'a {
        let request = self.base.request(Method::GET,"/lol-gameflow/v1/gameflow-phase");
        self.base.execute_val::<GameflowPhase>(request)
    }

    pub fn tick(&self) -> impl Future<Output = Result<String, reqwest::Error>> + 'a {
        let request = self.base.request(Method::POST,"/lol-gameflow/v1/session/request-lobby");
        self.base.execute_val::<String>(request)
    }
}

pub struct LobbyV2<'a> {
    base: &'a LeagueClient,
}


impl<'a> LobbyV2<'a> {
    /// get current summoner
    pub fn get_lobby(&self) -> impl Future<Output = Result<String, reqwest::Error>> + 'a {
        let request = self.base.request(Method::GET,"/lol-lobby/v2/lobby");
        self.base.execute_val::<String>(request)
    }

    pub fn create_lobby(&self, body: &lobby_v2::NewLobby) -> impl Future<Output = Result<Lobby, reqwest::Error>> + 'a {
        let request = self.base.request(Method::POST,"/lol-lobby/v2/lobby");
        let request = request.body(serde_json::ser::to_vec(body).unwrap());
        self.base.execute_val::<Lobby>(request)
    }
}