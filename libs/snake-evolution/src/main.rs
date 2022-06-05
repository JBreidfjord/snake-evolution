use console::Term;

use evolution::Evolution;
use snake::direction::Direction;
use snake::game::Game;

mod evolution;
mod snake;

const TRAINING_STEP_SIZE: usize = 1000;
const GAME_GRID_SIZE: isize = 10;

fn main() {
    let stdout = Term::buffered_stdout();
    let mut rng = rand::thread_rng();
    let mut evolution = Evolution::random(&mut rng, GAME_GRID_SIZE);
    let mut generation = 0;

    'train_loop: loop {
        generation += 1;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        evolution.train();

        let best_snake = evolution.best_individual();
        println!(
            "Generation {} complete. Best fitness: {} (Score: {:.2}%)",
            generation,
            best_snake.fitness(),
            (best_snake.game.score() as f32 / (GAME_GRID_SIZE.pow(2) - 2) as f32) * 100.0
        );

        if generation % TRAINING_STEP_SIZE == 0 {
            println!("Options:\n\t'c' -> Continue\n\t'b' -> Replay Best\n\t'w' -> Replay Worst\n\t's' -> Save Population\n\t'q' -> Quit");
            if let Ok(character) = stdout.read_char() {
                match character {
                    'c' => (),
                    'b' => evolution.replay("best", &stdout),
                    'w' => evolution.replay("worst", &stdout),
                    's' => evolution.save(),
                    'q' => break 'train_loop,
                    _ => break 'train_loop,
                }
            }
        }
        evolution.process_evolution(&mut rng);
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
