//! This library contains all of business-logic for the project and will be used by both the front
//! and back ends.

#![deny(
    unused,
    irrefutable_let_patterns,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]
#![warn(rust_2018_idioms)]

use std::collections::{HashMap, HashSet, hash_map::Iter};

use serde::{Serialize, Deserialize};
use typed_id::TypedId;
use uuid::Uuid;


/// The Id type for tournaments
pub type TournamentId = TypedId<Uuid, Tournament>;
/// The Id type for players
pub type PlayerId = TypedId<Uuid, Player>;
/// The Id type for games
pub type GameId = TypedId<Uuid, Game>;

/// A simple struct for managing players and games in a tournament
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tournament {
    players: HashMap<PlayerId, Player>,
    games: HashMap<GameId, Game>, }

/// A simple struct for representing players in a tournament
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Player {
    /// The name of the player
    pub name: String,
}

/// A simple struct for representing games in a tournament
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Game {
    /// All players in the game
    pub players: HashSet<PlayerId>,
    /// The winner, if one exists
    pub winner: Option<PlayerId>,
}

/// A simple type alias for the data that is returned when performing a tournament action
pub type TournamentResult = Result<ActionData, ActionError>;

/// This enum encodes all the way we can mutable the tournament. This is an example of the
/// message-passing pattern
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TournamentAction {
    /// Used to register a player in a tournament
    RegisterPlayer(String),
    /// Used to create a game in a tournament
    CreateGame(HashSet<PlayerId>),
    /// Used to record the winner of a game
    RecordWinner(GameId, PlayerId),
}

/// This enum encodes all the ways that an action can go wrong
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ActionError {
    /// The specified player is not registered for the tournament
    PlayerNotFound,
    /// The specified game is not in the tournament
    GameNotFound,
    /// The specified player is not in the specified round
    PlayerNotInGame,
}

/// This enum encodes all the data that can be returned when performing an acction
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ActionData {
    /// The new player's id
    RegisterPlayer(PlayerId),
    /// The new game's id
    CreateGame(GameId),
    /// No data is returned
    Nothing,
}

impl Tournament {
    /// Creates a new tournament
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            games: HashMap::new(),
        }
    }

    /// Gets a reference to a player
    pub fn get_player(&self, id: &PlayerId) -> Option<&Player> {
        self.players.get(id)
    }

    /// Gets an iterator over all the players
    pub fn get_players<'a>(&'a self) -> Iter<'a, PlayerId, Player> {
        self.players.iter()
    }

    /// Gets a reference to a game
    pub fn get_game(&self, id: &GameId) -> Option<&Game> {
        self.games.get(id)
    }

    /// Gets an iterator over all games
    pub fn get_games<'a>(&'a self) -> Iter<'a, GameId, Game> {
        self.games.iter()
    }

    /// Gets the win record of all players (games without a winner are excluded)
    pub fn get_standings(&self) -> HashMap<PlayerId, u8> {
        let mut digest = HashMap::with_capacity(self.players.len());
        for winner in self.games.values().filter_map(|game| game.winner) {
            match digest.get_mut(&winner) {
                Some(wins) => {
                    *wins += 1;
                }
                None => {
                    digest.insert(winner, 1);
                }
            }
        }
        digest
    }

    /// Performs an action on the tournament
    pub fn perform_action(&mut self, action: TournamentAction) -> TournamentResult {
        match action {
            TournamentAction::RegisterPlayer(name) => {
                let id = Uuid::new_v4().into();
                let plyr = Player { name };
                self.players.insert(id, plyr);
                Ok(ActionData::RegisterPlayer(id))
            }
            TournamentAction::CreateGame(players) => {
                let id = Uuid::new_v4().into();
                let game = Game {
                    players,
                    winner: None,
                };
                self.games.insert(id, game);
                Ok(ActionData::CreateGame(id))
            }
            TournamentAction::RecordWinner(game, plyr) => {
                if !self.players.contains_key(&plyr) {
                    return Err(ActionError::PlayerNotFound);
                }
                self.games
                    .get_mut(&game)
                    .ok_or(ActionError::GameNotFound)
                    .and_then(|game| {
                        if game.players.contains(&plyr) {
                            game.winner = Some(plyr);
                            Ok(game)
                        } else {
                            Err(ActionError::PlayerNotInGame)
                        }
                    })?;
                Ok(ActionData::Nothing)
            }
        }
    }
}

impl Default for Tournament {
    fn default() -> Self {
        Self::new()
    }
}
