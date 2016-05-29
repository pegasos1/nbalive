pub enum StatType {
    PlayByPlay,
    GameHeader,
    EastConfStandings,
    WestConfStandings,
    TeamRoster,
}


#[derive(Debug)]
pub enum Stat {
    PlayByPlay {
        game_id: Option<i64>,
        eventnum: Option<u64>,
        eventmsgtype: Option<u64>,
        eventmsgactiontype: Option<u64>,
        period: Option<u64>,
        wctimestring: Option<String>,
        pctimestring: Option<String>,
        homedescription: Option<String>,
        neutraldescription: Option<String>,
        visitordescription: Option<String>,
        score: Option<u64>,
        scoremargin: Option<u64>,
    },

    GameHeader {
        gamedate_est: Option<String>,
        game_sequence: Option<u64>,
        game_id: Option<String>,
        game_status_id: Option<u64>,
        game_status_text: Option<String>,
        gamecode: Option<String>,
        home_team_id: Option<String>,
        visitor_team_id: Option<String>,
        season: Option<u64>,
        live_period: Option<u64>,
        live_pc_time: Option<u64>,
        natl_tv_broadcaster_abbreviation: Option<String>,
        live_period_time_bcast: Option<String>,
        wh_status: Option<u64>,
    },

    EastConfStandings {
        team_id: Option<String>,
        league_id: Option<String>,
        season_id: Option<String>,
        standings_date: Option<String>,
        conference: Option<String>,
        team: Option<String>,
        g: Option<u64>,
        wins: Option<u64>,
        losses: Option<u64>,
        w_pct: Option<f64>,
        home_record: Option<String>,
        road_record: Option<String>,
    },

    WestConfStandings {
        team_id: Option<String>,
        league_id: Option<String>,
        season_id: Option<String>,
        standings_date: Option<String>,
        conference: Option<String>,
        team: Option<String>,
        g: Option<u64>,
        wins: Option<u64>,
        losses: Option<u64>,
        w_pct: Option<f64>,
        home_record: Option<String>,
        road_record: Option<String>,
    },

    TeamRoster {
        team_id: Option<String>,
        season: Option<String>,
        league_id: Option<String>,
        player: Option<String>,
        num: Option<String>,
        position: Option<String>,
        height: Option<String>,
        weight: Option<String>,
        birth_date: Option<String>,
        age: Option<u64>,
        exp: Option<String>,
        school: Option<String>,
        player_id: Option<String>,
    },
}
