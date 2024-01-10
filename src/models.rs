pub mod summoner_v1 {
    /// Summoner data object.
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Summoner {
        #[serde(rename = "accountId")]
        pub account_id: isize,
        #[serde(rename = "displayName")]
        pub display_name: String,
        /// 国服gameName为空
        #[serde(rename = "gameName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub game_name: Option<String>,
        #[serde(rename = "internalName")]
        pub internal_name: String,
        #[serde(rename = "nameChangeFlag")]
        pub name_change_flag: bool,
        #[serde(rename = "percentCompleteForNextLevel")]
        pub percent_complete_for_next_level: i32,
        pub privacy: String,
        #[serde(rename = "profileIconId")]
        pub profile_icon_id: i32,
        #[serde(rename = "puuid")]
        pub puuid: String,
        #[serde(rename = "rerollPoints")]
        pub reroll_points: RerollPoints,
        #[serde(rename = "summonerId")]
        pub summoner_id: isize,
        #[serde(rename = "summonerLevel")]
        pub summoner_level: i32,
        #[serde(rename = "tagLine")]
        pub tag_line: String,
        #[serde(rename = "unnamed")]
        pub unnamed: bool,
        #[serde(rename = "xpSinceLastLevel")]
        pub xp_since_last_level: i32,
        #[serde(rename = "xpUntilNextLevel")]
        pub xp_until_next_level: i32,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct RerollPoints {
        #[serde(rename = "currentPoints")]
        pub current_points: i32,
        #[serde(rename = "maxRolls")]
        pub max_rolls: i32,
        #[serde(rename = "numberOfRolls")]
        pub number_of_rolls: i32,
        #[serde(rename = "pointsCostToRoll")]
        pub points_cost_to_roll: i32,
        #[serde(rename = "pointsToReroll")]
        pub points_to_reroll: i32,
    }
}

pub mod gameflow_v1 {
    
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub enum GameflowPhase {
        None,
        Lobby,
        Matchmaking,
        ReadyCheck,
        ChampSelect,
        InProgress,
        PreEndOfGame,
        EndOfGame,
        Reconnect,
    }
}

pub mod lobby_v2 {
    #[derive(Clone, Debug)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct NewLobby {
        #[serde(rename = "queueId")]
        pub queue_id: i32,
    }

    #[derive(Clone, Debug)]
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Lobby {
        #[serde(rename = "canStartActivity")]
        pub can_start_activity: bool,
    }
}
