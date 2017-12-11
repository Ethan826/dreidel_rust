use IOProvider;
use dreidel::Dreidel;
use game::Game;
use player::Player;
use std::io::stdin;

pub struct ConsoleIO;

const SONG: &'static str = "Oh, dreidel, dreidel, dreidel\n\
                            I made you out of clay\n\
                            And when you’re dry and ready\n\
                            Oh Dreidel we shall play";

impl IOProvider for ConsoleIO {
    fn set_up_game(self) -> Game<ConsoleIO> {
        let mut names: Vec<String> = Vec::new();

        println!("{}\n\n", SONG);
        println!(
            "Enter the name of each player separated by a new line;\n\
             enter an empty line when you're done"
        );

        let mut input = String::new();

        loop {
            stdin().read_line(&mut input).expect("Failed to read line");
            if input == "\n" {
                break;
            }
            names.push(String::from(input.trim()));
            input = String::from("");
        }

        input = String::from("");
        println!("Enter the starting stake (how much each player gets).");
        stdin().read_line(&mut input).expect("Failed to read line");
        let stake: usize = input.trim().parse().expect("Please type a number!");
        Game::<ConsoleIO>::new(names, stake, self)
    }

    fn announce_winner(&self, name: &str) {
        println!("{} wins!", name);
    }

    fn announce_ante(&self, player: &Player, pot: usize) {
        println!(
            "{} antes. {} now has {} and the pot has {}.",
            player.name, player.name, player.stake, pot
        );
    }

    fn announce_no_qualified_player(&self) {
        println!("There are no players with money. Game over.");
    }

    fn announce_turn(&self, roll: &Dreidel, player: &Player, pot: usize) {
        println!(
            "{} rolled a {:?}. {} now has {} and the pot has {}.\n",
            player.name, roll, player.name, player.stake, pot
        );
    }
}
