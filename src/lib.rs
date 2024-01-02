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
    user_picks::UserPicks,
};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::de::DeserializeOwned;

/// Fantasy Premier League API Wrapper
///
/// The `Fpl` struct represents a wrapper for interacting with the Fantasy Premier League (FPL) API.
/// It provides methods for retrieving various data such as player details, team information, gameweek details, and more.
#[derive(Debug)]
pub struct Fpl {
    /// An optional field containing static data fetched from the FPL API.
    /// It is set to `None` initially and is populated with data whenever a request requiring static information is made.
    bootstrap_static: Option<BootstrapStatic>,
    /// An instance of an HTTP client used to make requests to the FPL API.
    http_client: Client,
}

impl Fpl {
    /// Creates a new instance of the `Fpl` API wrapper.
    ///
    /// # Returns
    ///
    /// A new instance of the `Fpl` API wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// fn main() {
    ///     // Create a new Fpl instance
    ///     let fpl = Fpl::new();
    ///
    ///     // Use the Fpl instance to make API requests
    ///     // ...
    /// }
    /// ```
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

    /// Asynchronously fetches data from the specified URL and deserializes it into the provided type.
    ///
    /// # Arguments
    ///
    /// * `url` - A `String` representing the URL from which to fetch the data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the deserialized data on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the specified URL.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the specified type.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted URLs or data types.
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

    /// Asynchronously retrieves information about a Fantasy Premier League user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - An `i64` representing the unique identifier of the FPL user.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with user information on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `User` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let user_id = 12345;
    ///
    ///     match fpl.get_user(user_id).await {
    ///         Ok(user) => {
    ///             // Process the user information
    ///             println!("{:?}", user);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `user_id` should be a valid identifier of an existing FPL user.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_user(&self, user_id: i64) -> Result<User, FplError> {
        let url = format!("https://fantasy.premierleague.com/api/entry/{}/", user_id);
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves information about Fantasy Premier League fixtures.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with fixtures information on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Fixtures` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///
    ///     match fpl.get_fixtures().await {
    ///         Ok(fixtures) => {
    ///             // Process the user information
    ///             println!("{:?}", fixtures);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_fixtures(&self) -> Result<Fixtures, FplError> {
        let url = String::from("https://fantasy.premierleague.com/api/fixtures/");
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves information about a Fantasy Premier League gameweek fixtures.
    ///
    /// # Arguments
    ///
    /// * `gameweek_id` - An `i64` representing the a gameweek (from 1 to 38)
    ///
    /// # Returns
    ///
    /// Returns a `Result` with fixture information on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `User` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let gameweek_id = 1;
    ///
    ///     match fpl.get_gameweek_fixtures(gameweek_id).await {
    ///         Ok(gameweek_fixtures) => {
    ///             // Process the gameweek fixtures
    ///             println!("{:?}", gameweek_fixtures);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `gameweek_id` should be a valid identifier of a gameweek.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_gameweek_fixtures(&self, gameweek_id: i64) -> Result<Fixtures, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/fixtures/?event={}",
            gameweek_id
        );
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves information about a Fantasy Premier League fixture.
    ///
    /// # Arguments
    ///
    /// * `fixture_id` - An `i64` representing the unique identifier of the FPL fixture.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with fixture information on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Fixture` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let fixture_id = 12;
    ///
    ///     match fpl.get_fixture(fixture_id).await {
    ///         Ok(fixture) => {
    ///             // Process the fixture information
    ///             println!("{:?}", fixture);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `fixture_id` should be a valid identifier of an FPL fixture.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves information about a Fantasy Premier League gameweek.
    ///
    /// # Arguments
    ///
    /// * `gameweek_id` - An `i64` representing the gameweek.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with gameweek information on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Event` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let gameweek_id = 12345;
    ///
    ///     match fpl.get_static_gameweek(gameweek_id).await {
    ///         Ok(Some(gameweek)) => {
    ///                 // Process the gameweek information
    ///                 println!("{:?}", gameweek);
    ///         },
    ///             Ok(None) => {
    ///                 eprintln!("Got nothing!");
    ///             }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         },
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `gameweek_id` should be a valid gameweek.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves live data for a specific Fantasy Premier League gameweek.
    ///
    /// # Arguments
    ///
    /// * `gameweek_id` - An `i64` representing the unique identifier of the FPL gameweek.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with live gameweek data on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Gameweek` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let gameweek_id = 5;
    ///
    ///     match fpl.get_live_gameweek(gameweek_id).await {
    ///         Ok(gameweek) => {
    ///             // Process the live gameweek data
    ///             println!("{:?}", gameweek);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `gameweek_id` should be a valid identifier of an existing FPL gameweek.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted gameweek IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_live_gameweek(&self, gameweek_id: i64) -> Result<Gameweek, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/event/{}/live",
            gameweek_id
        );
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves standings data for a Fantasy Premier League classic league.
    ///
    /// # Arguments
    ///
    /// * `league_id` - An `i64` representing the unique identifier of the FPL classic league.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with classic league standings on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `ClassicLeague` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let league_id = 98765;
    ///
    ///     match fpl.get_classic_league(league_id).await {
    ///         Ok(league) => {
    ///             // Process the classic league standings
    ///             println!("{:?}", league);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `league_id` should be a valid identifier of an existing FPL classic league.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted league IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_classic_league(&self, league_id: i64) -> Result<ClassicLeague, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/leagues-classic/{}/standings/",
            league_id
        );
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves standings data for a Fantasy Premier League head to head league.
    ///
    /// # Arguments
    ///
    /// * `league_id` - An `i64` representing the unique identifier of the FPL head to head league.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with head to head league standings on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `H2HLeague` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let league_id = 98765;
    ///
    ///     match fpl.get_h2h_league(league_id).await {
    ///         Ok(league) => {
    ///             // Process the head to head league standings
    ///             println!("{:?}", league);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `league_id` should be a valid identifier of an existing FPL head to head league.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted league IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_h2h_league(&self, league_id: i64) -> Result<H2HLeague, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/leagues-h2h-matches/league/{}/",
            league_id
        );
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves the picks made by a Fantasy Premier League user for a specific gameweek.
    ///
    /// # Arguments
    ///
    /// * `user_id` - An `i64` representing the unique identifier of the FPL user.
    /// * `gameweek_id` - An `i64` representing the unique identifier of the FPL gameweek.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the user's picks for the specified gameweek on success,
    /// or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `UserPicks` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let fpl = Fpl::new();
    ///     let user_id = 12345;
    ///     let gameweek_id = 5;
    ///
    ///     match fpl.get_user_picks(user_id, gameweek_id).await {
    ///         Ok(user_picks) => {
    ///             // Process the user's picks for the specified gameweek
    ///             println!("{:?}", user_picks);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `user_id` and `gameweek_id` should be valid identifiers.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted user or gameweek IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_user_picks(
        &self,
        user_id: i64,
        gameweek_id: i64,
    ) -> Result<UserPicks, FplError> {
        let url = format!(
            "https://fantasy.premierleague.com/api/entry/{}/event/{}/picks/",
            user_id, gameweek_id
        );
        return self.fetch(url).await;
    }

    /// Asynchronously retrieves information about a Fantasy Premier League team.
    ///
    /// # Arguments
    ///
    /// * `team_id` - An `i64` representing the unique identifier of the FPL team.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with team information on success, or an `FplError` on failure.
    ///
    /// If the team with the specified ID is not found, it returns `Ok(None)`.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Team` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let team_id = 6789;
    ///
    ///     match fpl.get_team(team_id).await {
    ///         Ok(Some(team)) => {
    ///             // Process the team information
    ///             println!("{:?}", team);
    ///         }
    ///         Ok(None) => {
    ///             // Handle the case when the team is not found
    ///             println!("Team not found");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `team_id` should be a valid identifier of an existing FPL team.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted team IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves information about multiple Fantasy Premier League teams.
    ///
    /// # Arguments
    ///
    /// * `team_ids` - A `Vec<i64>` containing unique identifiers of the FPL teams.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with team information for the specified team IDs on success,
    /// or an `FplError` on failure.
    ///
    /// If the provided `team_ids` is empty, it returns information about all FPL teams.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Team` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let team_ids = vec![123, 456, 789];
    ///
    ///     match fpl.get_teams(team_ids).await {
    ///         Ok(teams) => {
    ///             // Process the team information
    ///             println!("{:?}", teams);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `team_ids` should be valid identifiers of existing FPL teams.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted team IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves information about all Fantasy Premier League teams.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with information about all FPL teams on success,
    /// or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Team` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///
    ///     match fpl.get_all_teams().await {
    ///         Ok(teams) => {
    ///             // Process information about all FPL teams
    ///             println!("{:?}", teams);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_all_teams(&mut self) -> Result<Vec<Team>, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().teams),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.teams),
                Err(e) => return Err(e),
            },
        }
    }

    /// Asynchronously retrieves information about a Fantasy Premier League player.
    ///
    /// # Arguments
    ///
    /// * `player_id` - An `i64` representing the unique identifier of the FPL player.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with player information on success, or an `FplError` on failure.
    ///
    /// If the player with the specified ID is not found, it returns `Ok(None)`.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Player` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let player_id = 12345;
    ///
    ///     match fpl.get_player(player_id).await {
    ///         Ok(Some(player)) => {
    ///             // Process player information
    ///             println!("{:?}", player);
    ///         }
    ///         Ok(None) => {
    ///             // Handle the case when the player is not found
    ///             println!("Player not found");
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `player_id` should be a valid identifier of an existing FPL player.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted player IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves information about multiple Fantasy Premier League players.
    ///
    /// # Arguments
    ///
    /// * `player_ids` - A `Vec<i64>` containing unique identifiers of the FPL players.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with player information for the specified player IDs on success,
    /// or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Players` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///     let player_ids = vec![12345, 111];
    ///
    ///     match fpl.get_players(player_ids).await {
    ///         Ok(players) => {
    ///             // Process players information
    ///             println!("{:?}", players);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    /// The provided `player_ids` should be a list of valid identifiers of existing FPL players.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when passing untrusted player IDs or relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
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

    /// Asynchronously retrieves information about all Fantasy Premier League players.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with information about all FPL players on success,
    /// or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Players` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///
    ///     match fpl.get_all_players().await {
    ///         Ok(players) => {
    ///             // Process information about all FPL players
    ///             println!("{:?}", players);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_all_players(&mut self) -> Result<Players, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().elements),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.elements),
                Err(e) => return Err(e),
            },
        }
    }

    /// Asynchronously retrieves information about static gameweeks in the Fantasy Premier League.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with information about static gameweeks on success,
    /// or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `Vec<Event>` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///
    ///     match fpl.get_static_gameweeks().await {
    ///         Ok(gameweeks) => {
    ///             // Process information about static gameweeks
    ///             println!("{:?}", gameweeks);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_static_gameweeks(&mut self) -> Result<Vec<Event>, FplError> {
        match &self.bootstrap_static {
            Some(bootstrap_static) => Ok(bootstrap_static.clone().events),
            None => match self.get_bootstrap_static().await {
                Ok(bootstrap_static) => Ok(bootstrap_static.events),
                Err(e) => return Err(e),
            },
        }
    }

    /// Asynchronously retrieves static data from the Fantasy Premier League API.
    ///
    /// This function is typically used to fetch data that does not change frequently,
    /// such as player details, team information, and gameweek details.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with static data on success, or an `FplError` on failure.
    ///
    /// # Errors
    ///
    /// This function may return an `FplError` in the following cases:
    /// - If there is a failure when making the request to the FPL API.
    /// - If the HTTP response status code is not OK (200).
    /// - If there is an error deserializing the JSON response into the `BootstrapStatic` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fpl_rs::Fpl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut fpl = Fpl::new();
    ///
    ///     match fpl.get_bootstrap_static().await {
    ///         Ok(bootstrap_static) => {
    ///             // Process static data from the FPL API
    ///             println!("{:?}", bootstrap_static);
    ///         }
    ///         Err(err) => {
    ///             // Handle the error
    ///             eprintln!("Error: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function utilizes the `fetch` method internally to make a request to the FPL API.
    ///
    /// # Panics
    ///
    /// This function may panic if there is an internal error during HTTP request processing.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` due to its reliance on external data (HTTP responses).
    /// Use caution when relying on FPL API data.
    ///
    /// # See Also
    ///
    /// - [`fetch`](struct.Fpl.html#method.fetch)
    /// - [Fantasy Premier League API Documentation](https://fantasy.premierleague.com/api)
    pub async fn get_bootstrap_static(&mut self) -> Result<BootstrapStatic, FplError> {
        match &self.bootstrap_static {
            Some(b) => return Ok(b.clone()),
            None => {}
        }
        let url = String::from("https://fantasy.premierleague.com/api/bootstrap-static/");
        let bootstrap_static: BootstrapStatic = self.fetch(url).await?;
        self.bootstrap_static = Some(bootstrap_static.clone());
        return Ok(bootstrap_static);
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

    #[tokio::test]
    async fn test_get_user_picks() {
        let fpl = Fpl::new();
        let user_id = 5489342;
        let gameweek_id = 14;
        let user_picks = fpl.get_user_picks(user_id, gameweek_id).await.unwrap();
        assert!(user_picks.picks.len() == 15);
    }
}
