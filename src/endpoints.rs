use std::future::Future;

use reqwest::Method;

use crate::{
    models::{
        champ_select_v1::{NewAction, Session},
        gameflow_v1::GameflowPhase,
        lobby_v2::{self, Lobby},
        summoner_v1, Empty,
    },
    LeagueClient,
};

impl LeagueClient {
    /// Returns a handle for accessing [AccountV1](crate::endpoints::AccountV1) endpoints.
    /// # Riot Developer API Reference
    /// <a href="https://developer.riotgames.com/apis#account-v1" target="_blank">`account-v1`</a>
    ///
    /// Note: this method is automatically generated.
    pub fn summoner_v1(&self) -> SummonerV1 {
        SummonerV1 { base: self }
    }

    pub fn gameflow_v1(&self) -> GameflowV1 {
        GameflowV1 { base: self }
    }

    pub fn lobby_v2(&self) -> LobbyV2 {
        LobbyV2 { base: self }
    }

    pub fn matchmaking_v1(&self) -> MatchmakingV1 {
        MatchmakingV1 { base: self }
    }
    pub fn champ_select_v1(&self) -> ChampSelectV1 {
        ChampSelectV1 { base: self }
    }
}

pub struct SummonerV1<'a> {
    base: &'a LeagueClient,
}

impl<'a> SummonerV1<'a> {
    /// get current summoner

    pub fn current_summoner(
        &self,
    ) -> impl Future<Output = Result<summoner_v1::Summoner, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::GET, "/lol-summoner/v1/current-summoner");
        self.base.execute_val::<summoner_v1::Summoner>(request)
    }
}

pub struct GameflowV1<'a> {
    base: &'a LeagueClient,
}

impl<'a> GameflowV1<'a> {
    /// get current summoner

    pub fn gameflow_phase(
        &self,
    ) -> impl Future<Output = Result<GameflowPhase, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::GET, "/lol-gameflow/v1/gameflow-phase");
        self.base.execute_val::<GameflowPhase>(request)
    }
}

pub struct LobbyV2<'a> {
    base: &'a LeagueClient,
}

impl<'a> LobbyV2<'a> {
    /// get current summoner
    pub fn get_lobby(&self) -> impl Future<Output = Result<String, reqwest::Error>> + 'a {
        let request = self.base.request(Method::GET, "/lol-lobby/v2/lobby");
        self.base.execute_val::<String>(request)
    }

    pub fn create_lobby(
        &self,
        body: &lobby_v2::NewLobby,
    ) -> impl Future<Output = Result<Lobby, reqwest::Error>> + 'a {
        let request = self.base.request(Method::POST, "/lol-lobby/v2/lobby");
        let request = request.body(serde_json::ser::to_vec(body).unwrap());
        self.base.execute_val::<Lobby>(request)
    }

    pub fn search_match(&self) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::POST, "/lol-lobby/v2/lobby/matchmaking/search");
        self.base.execute_val::<Empty>(request)
    }

    pub fn accept_match(&self) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::POST, "/lol-matchmaking/v1/ready-check/accept");
        self.base.execute_val::<Empty>(request)
    }
}

pub struct MatchmakingV1<'a> {
    base: &'a LeagueClient,
}

impl<'a> MatchmakingV1<'a> {
    pub fn search(&self) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::POST, "/lol-lobby/v2/lobby/matchmaking/search");
        self.base.execute_val::<Empty>(request)
    }

    pub fn accept(&self) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::POST, "/lol-matchmaking/v1/ready-check/accept");
        self.base.execute_val::<Empty>(request)
    }

    pub fn decline(&self) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::POST, "/lol-matchmaking/v1/ready-check/decline");
        self.base.execute_val::<Empty>(request)
    }
}

pub struct ChampSelectV1<'a> {
    base: &'a LeagueClient,
}

impl<'a> ChampSelectV1<'a> {
    pub fn session(&self) -> impl Future<Output = Result<Session, reqwest::Error>> + 'a {
        let request = self
            .base
            .request(Method::GET, "/lol-champ-select/v1/session");
        self.base.execute_val::<Session>(request)
    }

    pub fn action(
        &self,
        action: &NewAction,
    ) -> impl Future<Output = Result<Empty, reqwest::Error>> + 'a {
        let request = self.base.request(
            Method::PATCH,
            &format!("/lol-champ-select/v1/session/actions/{}/", action.id),
        );
        let request = request.body(serde_json::to_vec(action).unwrap());
        self.base.execute_val::<Empty>(request)
    }
}
