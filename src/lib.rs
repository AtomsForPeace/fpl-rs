pub mod fpl_error;
pub mod models;

use fpl_error::FplError;
use models::{
    bootstrap_static::{BootstrapStatic, Event, Player, Players, Team},
    classic_league::ClassicLeague,
    fixture::{Fixture, Fixtures},
    gameweek::Gameweek,
    h2h_league::H2HLeague,
    user::User,
};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Fpl {
    bootstrap_static: Option<BootstrapStatic>,
    http_client: Client,
}

impl Fpl {
    pub fn new() -> Fpl {
        let default_headers = HeaderMap::new();
        let http_client = ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .expect("Failed to build Http client");
        Fpl {
            bootstrap_static: None,
            http_client,
        }
    }

    async fn fetch<T>(&self, url: String) -> Result<T, FplError>
    where
        T: DeserializeOwned,
    {
        let error_message = format!("Failed when making request to: {}", url);
        let response = match self.http_client.get(url).send().await {
            Ok(r) => r,
            Err(err) => {
                let error_message = format!("{} with this error: {}", error_message, err);
                return Err(FplError::from(error_message.as_str()));
            }
        };
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<T>().await {
                Ok(parsed) => Ok(parsed),
                Err(err) => {
                    let error_message = format!("{} with this error: {}", error_message, err);
                    Err(FplError::from(error_message.as_str()))
                }
            },
            other_status_code => {
                let error_message = format!(
                    "{} with this status code: {}",
                    error_message, other_status_code
                );
                Err(FplError::from(error_message.as_str()))
            }
        }
    }

    pub async fn get_user(&self, user_id: i64) -> Result<User, FplError> {
        let url = format!("https://fantasy.premierleague.com/api/entry/{}/", user_id);
        return self.fetch(url).await;
    }

    pub async fn get_fixtures(&self) -> Result<Fixtures, FplError> {
        let url = String::from("https://fantasy.premierleague.com/api/fixtures/");
        return self.fetch(url).await;
    }

    pub async fn get_gameweek_fixtures(&self, gameweek_id: i64) -> Result<Fixtures, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/fixtures/?event={}",
            gameweek_id
        );
        return self.fetch(url).await;
    }

    pub async fn get_fixture(&mut self, fixture_id: i64) -> Result<Fixture, FplError> {
        let all_fixtures = self.get_fixtures().await?;
        let gameweek_id = all_fixtures
            .into_iter()
            .filter(|fixture| fixture.id == fixture_id)
            .collect::<Fixtures>()
            .first()
            .expect("Failed when parsing fixtures response.")
            .event
            .expect("Failed when parsing fixtures response.");

        let gameweek_fixtures = self.get_gameweek_fixtures(gameweek_id).await?;

        let fixture = gameweek_fixtures
            .into_iter()
            .filter(|fixture| fixture.id == fixture_id)
            .collect::<Fixtures>();

        match fixture.first() {
            Some(f) => Ok(f.clone()),
            None => return Err(FplError::from("Failed when parsing fixtures response.")),
        }
    }

    pub async fn get_static_gameweek(
        &mut self,
        gameweek_id: i64,
    ) -> Result<Option<Event>, FplError> {
        let all_gameweeks = self.get_static_gameweeks().await?;
        return Ok(all_gameweeks
            .into_iter()
            .filter(|gameweek| gameweek_id == gameweek.id)
            .collect::<Vec<Event>>()
            .first()
            .cloned());
    }

    pub async fn get_live_gameweek(&self, gameweek_id: i64) -> Result<Gameweek, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/event/{}/live",
            gameweek_id
        );
        return self.fetch(url).await;
    }

    pub async fn get_classic_league(&self, league_id: i64) -> Result<ClassicLeague, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/leagues-classic/{}/standings/",
            league_id
        );
        return self.fetch(url).await;
    }

    pub async fn get_h2h_league(&self, league_id: i64) -> Result<H2HLeague, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/leagues-h2h-matches/league/{}/",
            league_id
        );
        return self.fetch(url).await;
    }

    pub async fn get_team(&mut self, team_id: i64) -> Result<Option<Team>, FplError> {
        let bootstrap_static = match &self.bootstrap_static {
            Some(bootstrap_static) => bootstrap_static.clone(),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => bootstrap_static,
                Err(e) => return Err(e),
            },
        };
        Ok(bootstrap_static
            .clone()
            .teams
            .into_iter()
            .filter(|team| team_id == team.id)
            .collect::<Vec<Team>>()
            .first()
            .cloned())
    }

    pub async fn get_teams(&mut self, team_ids: Vec<i64>) -> Result<Vec<Team>, FplError> {
        let bootstrap_static = match &self.bootstrap_static {
            Some(bootstrap_static) => bootstrap_static.clone(),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => bootstrap_static,
                Err(e) => return Err(e),
            },
        };
        match team_ids {
            x if x.is_empty() => Ok(bootstrap_static.teams),
            t_ids => Ok(bootstrap_static
                .clone()
                .teams
                .into_iter()
                .filter(|team| t_ids.contains(&team.id))
                .collect()),
        }
    }

    pub async fn get_all_teams(&mut self) -> Result<Vec<Team>, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().teams),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.teams),
                Err(e) => return Err(e),
            },
        }
    }

    pub async fn get_player(&mut self, player_id: i64) -> Result<Option<Player>, FplError> {
        let bootstrap_static = match &self.bootstrap_static {
            Some(bootstrap_static) => bootstrap_static.clone(),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => bootstrap_static,
                Err(e) => return Err(e),
            },
        };
        Ok(bootstrap_static
            .clone()
            .elements
            .into_iter()
            .filter(|element| player_id == element.id)
            .collect::<Players>()
            .first()
            .cloned())
    }

    pub async fn get_players(&mut self, player_ids: Vec<i64>) -> Result<Players, FplError> {
        let bootstrap_static = match &self.bootstrap_static {
            Some(bootstrap_static) => bootstrap_static.clone(),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => bootstrap_static,
                Err(e) => return Err(e),
            },
        };

        Ok(bootstrap_static
            .clone()
            .elements
            .into_iter()
            .filter(|element| player_ids.contains(&element.id))
            .collect::<Players>())
    }

    pub async fn get_all_players(&mut self) -> Result<Players, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().elements),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.elements),
                Err(e) => return Err(e),
            },
        }
    }

    pub async fn get_static_gameweeks(&mut self) -> Result<Vec<Event>, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().events),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.events),
                Err(e) => return Err(e),
            },
        }
    }

    pub async fn get_bootstrap_static(&mut self) -> Result<BootstrapStatic, FplError> {
        match &self.bootstrap_static {
            Some(b) => return Ok(b.clone()),
            None => {}
        }
        let url = String::from("https://fantasy.premierleague.com/api/bootstrap-static/");
        return self.fetch(url).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user() {
        let fpl = Fpl::new();
        let user_id = 5489342;
        match fpl.get_user(user_id).await {
            Ok(user) => assert_eq!(user.id, user_id),
            Err(e) => panic!("Got this guy: {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_bootstrap_static() {
        let mut fpl = Fpl::new();
        let bootstrap_static = fpl.get_bootstrap_static().await.unwrap();
        assert!(bootstrap_static.teams.len() > 0);
    }

    #[tokio::test]
    async fn test_get_all_teams() {
        let mut fpl = Fpl::new();
        let teams = fpl.get_all_teams().await.unwrap();
        assert!(teams.len() == 20);
    }

    #[tokio::test]
    async fn test_get_teams() {
        let mut fpl = Fpl::new();
        let teams = fpl.get_teams(vec![1, 2]).await.unwrap();
        assert!(teams.len() == 2);
    }

    #[tokio::test]
    async fn test_get_team() {
        let mut fpl = Fpl::new();
        let team = fpl.get_team(2).await.unwrap().unwrap();
        assert!(team.name == "Aston Villa");
    }

    #[tokio::test]
    async fn test_get_fixture() {
        let mut fpl = Fpl::new();
        let fixture = fpl.get_fixture(65).await.unwrap();
        assert!(fixture.team_h == 14);
    }

    #[tokio::test]
    async fn test_get_classic_league() {
        let fpl = Fpl::new();
        let classic_league = fpl.get_classic_league(753276).await.unwrap();
        assert!(classic_league.standings.results.len() == 16);
    }

    #[tokio::test]
    async fn test_get_h2h_league() {
        let fpl = Fpl::new();
        let h2h_league = fpl.get_h2h_league(288399).await.unwrap();
        assert!(!h2h_league.results.is_empty());
    }

    #[tokio::test]
    async fn test_get_live_gameweek() {
        let fpl = Fpl::new();
        let live_gameweek = fpl.get_live_gameweek(2).await.unwrap();
        assert!(live_gameweek.elements.len() == 670);
    }

    #[tokio::test]
    async fn test_get_static_gameweek() {
        let mut fpl = Fpl::new();
        let gameweek_id = 2;
        let static_gameweek = fpl.get_static_gameweek(gameweek_id).await.unwrap().unwrap();
        assert!(static_gameweek.id == gameweek_id);
    }
}
