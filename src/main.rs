extern crate dreidel;

use dreidel::io::ConsoleIO;
use dreidel::IOProvider;

fn main() {
    let io = ConsoleIO {};
    let mut game = io.set_up_game();
    game.play_game();
}
