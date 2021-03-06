use dreidel::Dreidel;
use player::Player;
use rand::{thread_rng, ThreadRng};
use IOProvider;

#[derive(Debug)]
pub struct Game<T>
where
    T: IOProvider,
{
    current_player: Option<usize>, // index into self.players, None if game over
    io_provider: Box<T>,
    players: Vec<Player>,
    pot: usize,
    rng: ThreadRng,
}

impl<T> Game<T>
where
    T: IOProvider,
{
    pub fn new(players: Vec<String>, starting_stake: usize, io_provider: T) -> Game<T> {
        Game {
            current_player: Some(0),
            players: players
                .into_iter()
                .map(|player| Player {
                    name: player,
                    stake: starting_stake,
                })
                .collect::<Vec<_>>(),
            io_provider: Box::new(io_provider),
            pot: 0,
            rng: thread_rng(),
        }
    }

    pub fn play_game(&mut self) {
        loop {
            // 1. See if anyone has won. If so, call the announce_winner method and break
            match self.get_winner() {
                Some(winner) => {
                    self.io_provider.announce_winner(&self.players[winner].name);
                    break;
                }
                _ => (),
            }
            // 2. See if no one can play. If so, call the no money method and break
            if !self.has_qualified_player() {
                self.io_provider.announce_no_qualified_player();
                break;
            }
            // 3. Call advance player, then play turn with that player
            self.advance_player();
            self.play_turn();
        }
    }

    fn get_winner(&self) -> Option<usize> {
        let live_players = &self.players
            .iter()
            .enumerate()
            .filter(|&(_, player)| player.stake > 0)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        if live_players.len() == 1 {
            Some(live_players[0])
        } else {
            None
        }
    }

    fn has_qualified_player(&self) -> bool {
        self.players.iter().any(|player| player.stake > 0)
    }

    fn play_turn(&mut self) {
        if self.current_player.is_some() {
            for player in &mut self.players {
                if player.stake > 0 {
                    player.stake -= 1;
                    self.pot += 1;
                    self.io_provider.announce_ante(player, self.pot);
                }
            }

            let player = &mut self.players[self.current_player.unwrap()];
            let roll = Dreidel::spin(&mut self.rng, &mut player.stake, &mut self.pot);
            self.io_provider.announce_turn(&roll, player, self.pot);
        }
    }

    fn advance_player(&mut self) {
        // Bail if we already know there's no current player
        if self.current_player.is_some() {
            for _ in 0..self.players.len() {
                // We add one to the current player. But we have to wrap, so do it modulo the
                // number of players in the game. And we have to do it inside an option, hence the
                // `Option::map`.
                self.current_player = self.current_player
                    .map(|player| (player + 1) % self.players.len());
                // Return as soon as we have a player with a nonzero stake
                if self.players[self.current_player.unwrap()].stake > 0 {
                    return;
                }
            }
            // If we have gone around the circle and nobody has any money...
            self.current_player = None;
        }
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[cfg(test)]
struct MockIO;

#[cfg(test)]
impl IOProvider for MockIO {
    fn set_up_game(self) -> Game<MockIO> {
        Game {
            current_player: Some(0),
            io_provider: Box::new(self),
            players: Vec::new(),
            pot: 0,
            rng: thread_rng(),
        }
    }

    fn announce_winner(&self, _: &str) {}

    fn announce_no_qualified_player(&self) {}

    fn announce_turn() {}
}

#[test]
fn test_new() {
    let game = Game::new(
        vec![String::from("Ethan"), String::from("Madigan")],
        20,
        MockIO {},
    );
    assert_eq!(
        game.players,
        vec![
            Player {
                name: String::from("Ethan"),
                stake: 20,
            },
            Player {
                name: String::from("Madigan"),
                stake: 20,
            },
        ]
    );
    assert_eq!(game.pot, 0);
}

#[test]
fn test_advance_player() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        10,
        MockIO {},
    );
    subject.advance_player();
    assert_eq!(Some(1), subject.current_player);
}

#[test]
fn test_advance_player_broke_player() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        10,
        MockIO {},
    );
    subject.players[1].stake = 0;
    subject.advance_player();
    assert_eq!(Some(2), subject.current_player);
}

#[test]
fn test_advance_broke_player_wrapping() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        10,
        MockIO {},
    );
    subject.current_player = Some(1);
    subject.players[2].stake = 0;
    subject.advance_player();
    assert_eq!(Some(0), subject.current_player);
}

#[test]
fn test_everyone_broke() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        0,
        MockIO {},
    );
    subject.advance_player();
    assert_eq!(None, subject.current_player);
}

#[test]
fn test_everyone_broke_false_positive() {
    let mut subject = Game::new(
        vec![String::from("Ethan"), String::from("Madigan")],
        0,
        MockIO {},
    );
    subject.players[0].stake = 1;
    subject.advance_player();
    assert_eq!(Some(0), subject.current_player);
}

#[test]
fn test_play_game_winner() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        0,
        MockIO {},
    );
    subject.players[0].stake = 1;
    subject.play_game();
}
