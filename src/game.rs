use dreidel::Dreidel;
use player::Player;
use rand::{thread_rng, Rng, ThreadRng};

#[derive(Debug)]
pub struct Game {
    current_player: Option<usize>, // index into self.players, None if game over
    rng: ThreadRng,
    players: Vec<Player>,
    pot: usize,
}

impl Game {
    pub fn new(players: Vec<String>, starting_stake: usize) -> Game {
        Game {
            current_player: Some(0),
            players: players
                .into_iter()
                .map(|player| {
                    Player {
                        name: player,
                        stake: starting_stake,
                    }
                })
                .collect::<Vec<_>>(),
            pot: 0,
            rng: thread_rng(),
        }
    }

    fn play_turn(&mut self, mut rng: &mut Rng) {
        if self.current_player.is_some() {
            let player = &mut self.players[self.current_player.unwrap()];
            Dreidel::spin(&mut rng, &mut player.stake, &mut self.pot);
        }
    }

    fn advance_player(&mut self) {
        // Bail if we already know there's no current player
        if self.current_player.is_some() {
            // If we go around the circle one time and everyone is broke, we're done. So we have to
            // keep track of the number of players we checked
            let mut iterations = 0;
            while iterations <= self.players.len() {
                // We add one to the current player. But we have to wrap, so do it modulo the
                // number of players in the game. And we have to do it inside an option, hence the
                // `Option::map`.
                self.current_player = self.current_player
                    .map(|player| (player + 1) % self.players.len());
                // We're simulating a do-while loop here. Break as soon as we have a player with a
                // nonzero stake
                if self.players[self.current_player.unwrap()].stake > 0 {
                    break;
                }
                iterations += 1;
            }
            // Once we're out of the `while` loop, we don't know if we stopped because we went all
            // the way around the circle and everyone was broke, or else because we found a
            // non-broke player. So, check.
            if iterations > self.players.len() {
                self.current_player = None;
            }
        }
    }
}

// =================================================================================================
// Tests
// =================================================================================================

#[test]
fn test_new() {
    let game = Game::new(vec![String::from("Ethan"), String::from("Madigan")], 20);
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
    );
    subject.advance_player();
    assert_eq!(None, subject.current_player);
}

#[test]
fn test_everyone_broke_false_positive() {
    let mut subject = Game::new(
        vec![
            String::from("Ethan"),
            String::from("Madigan"),
            String::from("Milo"),
        ],
        0,
    );
    subject.players[0].stake = 1;
    subject.advance_player();
    assert_eq!(Some(0), subject.current_player);
}
