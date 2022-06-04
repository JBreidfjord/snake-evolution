use console::Term;

use snake::direction::Direction;
use snake::game::Game;

mod snake;

fn main() {
    let stdout = Term::buffered_stdout();
    let mut game = Game::new(10);

    'game_loop: loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", game.display());

        let direction = if let Ok(character) = stdout.read_char() {
            match character {
                'w' => Direction::Up,
                'a' => Direction::Left,
                's' => Direction::Down,
                'd' => Direction::Right,
                _ => break 'game_loop,
            }
        } else {
            panic!("Failed to read input")
        };

        game.move_snake(direction);

        if game.finished() {
            break 'game_loop;
        }
    }
}
