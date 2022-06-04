use console::Term;

use evolution::Evolution;
use snake::direction::Direction;
use snake::game::Game;

mod evolution;
mod snake;

const GAME_GRID_SIZE: isize = 10;

fn main() {
    let stdout = Term::buffered_stdout();
    let mut rng = rand::thread_rng();
    let mut evolution = Evolution::random(&mut rng, GAME_GRID_SIZE);

    'train_loop: loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        evolution.train(&mut rng);

        let best_snake = evolution.best_individual();
        println!(
            "Generation {} complete. Best fitness: {} (Score: {:.2}%)",
            evolution.age(),
            best_snake.fitness(),
            (best_snake.game.score() as f32 / (GAME_GRID_SIZE.pow(2) - 2) as f32) * 100.0
        );

        println!("Options:\n\t'c' -> Continue\n\t's' -> Save\n\t'q' -> Quit");
        if let Ok(character) = stdout.read_char() {
            match character {
                'c' => (),
                's' => evolution.save(),
                'q' => break 'train_loop,
                _ => break 'train_loop,
            }
        }
    }
}

fn _play() {
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
