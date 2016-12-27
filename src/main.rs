extern crate rand;

use rand::{thread_rng, Rng};
use std::io;

#[derive(Debug, PartialEq)]
enum Dreidel {
    Shin,
    Nun,
    Hay,
    Gimel,
    Pass,
}

#[derive(Debug)]
struct DreidelResult {
    player: u16,
    pot: u16,
    result: Dreidel,
}

impl PartialEq for DreidelResult {
    fn eq(&self, other: &DreidelResult) -> bool {
        self.player == other.player && self.pot == other.pot && self.result == other.result
    }
}

#[derive(Debug)]
struct DreidelGame {
    pot: u16,
    players: Vec<Option<u16>>,
}

impl DreidelGame {
    fn in_progress(&self) -> bool {
        self.players.iter().any(|&player| match player {
            Some(_) => true,
            _ => false,
        })
    }

    fn play_turn(&mut self, player_index: usize, random_number: u8) {
        match self.players[player_index] {
            None => println!("{:?}", Dreidel::Pass),
            Some(stake) => {
                let result = run_round(stake, self.pot, random_number);
                if result.player == 0 {
                    self.players[player_index] = None;
                } else {
                    self.players[player_index] = Some(result.player)
                };
                println!("{:?}", result.result);
                self.pot = result.pot;
            }
        }
    }

    fn ante(&mut self) {
        let result = self.players.iter().fold((self.pot, Vec::with_capacity(self.players.len())),
                                              |mut accum, &player| {
            match player {
                Some(1) => {
                    accum.0 += 1;
                    accum.1.push(None);
                    accum
                }
                Some(stake) => {
                    accum.0 += 1;
                    accum.1.push(Some(stake - 1));
                    accum
                }
                _ => {
                    accum.1.push(None);
                    accum
                }
            }
        });
        self.pot = result.0;
        self.players = result.1;
    }
}

#[test]
fn test_in_progress() {
    let game_1 = DreidelGame {
        pot: 30,
        players: vec![None, None],
    };
    assert_eq!(false, game_1.in_progress());
    let game_2 = DreidelGame {
        pot: 30,
        players: vec![Some(3), None],
    };
    assert_eq!(true, game_2.in_progress());
}

#[test]
fn test_play_turn() {
    let mut game = DreidelGame {
        pot: 30,
        players: vec![Some(10), Some(15)],
    };
    game.play_turn(0, 3);
    assert_eq!(game.pot, 0);
    assert_eq!(game.players[0], Some(40));
    game = DreidelGame {
        pot: 30,
        players: vec![Some(1), Some(15)],
    };
    game.play_turn(0, 0);
    assert_eq!(game.players[0], None);
}

#[test]
fn test_ante() {
    let mut game = DreidelGame {
        pot: 30,
        players: vec![Some(10), Some(15)],
    };
    game.ante();
    assert_eq!(32, game.pot);
    assert_eq!(Some(9), game.players[0]);
}

fn main() {
    let game = initialize_game(20);
    println!("{:?}", game);
}

fn initialize_game(starting_balance: u16) -> DreidelGame {
    let mut num_players = String::new();
    clear_screen();
    println!("How many players?");
    io::stdin().read_line(&mut num_players).expect("Failed to read line.");
    let num_players: usize = num_players.trim().parse().expect("Please type a number.");
    DreidelGame {
        pot: 0,
        players: vec![Some(starting_balance); num_players],
    }
}

fn clear_screen() {
    std::process::Command::new("clear").status().unwrap().success();
}

fn run_round(player: u16, pot: u16, random_number: u8) -> DreidelResult {
    match random_number {
        0 => shin(player, pot),
        1 => nun(player, pot),
        2 => hay(player, pot),
        _ => gimel(player, pot),
    }
}

#[test]
fn test_run_round() {
    assert_eq!(shin(55, 71), run_round(55, 71, 0));
    assert_eq!(nun(55, 71), run_round(55, 71, 1));
    assert_eq!(hay(55, 71), run_round(55, 71, 2));
    assert_eq!(gimel(55, 71), run_round(55, 71, 3));
}

fn random_0_to_4() -> u8 {
    thread_rng().gen_range(0, 4)
}

fn shin(player: u16, pot: u16) -> DreidelResult {
    let adjustment_amount = if player == 0 { 0 } else { 1 };
    DreidelResult {
        player: player - adjustment_amount,
        pot: pot + adjustment_amount,
        result: Dreidel::Shin,
    }
}

#[test]
fn test_shin() {
    assert_eq!(DreidelResult {
                   player: 54,
                   pot: 72,
                   result: Dreidel::Shin,
               },
               shin(55, 71));
    assert_eq!(DreidelResult {
                   player: 0,
                   pot: 15,
                   result: Dreidel::Shin,
               },
               shin(0, 15));
}

fn nun(player: u16, pot: u16) -> DreidelResult {
    DreidelResult {
        player: player,
        pot: pot,
        result: Dreidel::Nun,
    }
}

#[test]
fn test_nun() {
    assert_eq!(DreidelResult {
                   player: 0,
                   pot: 0,
                   result: Dreidel::Nun,
               },
               nun(0, 0));
    assert_eq!(DreidelResult {
                   player: 5,
                   pot: 5,
                   result: Dreidel::Nun,
               },
               nun(5, 5));
}

fn hay(player: u16, pot: u16) -> DreidelResult {
    let adjustment_amount = if pot & 1 == 0 { pot / 2 } else { (pot + 1) / 2 };
    DreidelResult {
        player: player + adjustment_amount,
        pot: pot - adjustment_amount,
        result: Dreidel::Hay,
    }
}

#[test]
fn test_hay() {
    assert_eq!(DreidelResult {
                   player: 32,
                   pot: 15,
                   result: Dreidel::Hay,
               },
               hay(17, 30));
    assert_eq!(DreidelResult {
                   player: 19,
                   pot: 9,
                   result: Dreidel::Hay,
               },
               hay(9, 19));
}

fn gimel(player: u16, pot: u16) -> DreidelResult {
    DreidelResult {
        player: player + pot,
        pot: 0,
        result: Dreidel::Gimel,
    }
}

#[test]
fn test_gimel() {
    assert_eq!(DreidelResult {
                   player: 47,
                   pot: 0,
                   result: Dreidel::Gimel,
               },
               gimel(17, 30));
    assert_eq!(DreidelResult {
                   player: 19,
                   pot: 0,
                   result: Dreidel::Gimel,
               },
               gimel(0, 19));
}

#[cfg(test)]
mod tests {
    use super::*;
}
