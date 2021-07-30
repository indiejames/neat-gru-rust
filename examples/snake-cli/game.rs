use neat_gru::neural_network::nn::NeuralNetwork;
use crossterm::{ExecutableCommand, cursor::Hide, terminal::{Clear, ClearType, SetSize, enable_raw_mode, size}};
use std::io::Stdout;

use crate::{apple::Apple, defs::RESOLUTION, snake::Snake, utils::{distance_to_apple_x, distance_to_apple_y}};

#[derive(Debug)]
pub struct Game {
    snakes: Vec<Snake>,
    apple: Apple,
    scores: Vec<f64>,
}

impl Game {
    pub fn get_scores(&self) -> Vec<f64> {
        self.scores.clone()
    }

    pub fn new(neural_networks: Vec<NeuralNetwork<f64>>) -> Game {
        let mut snakes = vec![];
        for net in &neural_networks {
            snakes.push(Snake::new(net.clone()));
        }
        let mut scores = vec![];
        neural_networks.iter().for_each(|_| scores.push(0.0));
        Game {
            snakes,
            apple: Apple::generate_apple(),
            scores,
        }
    }

    pub fn prepare_ui(&mut self){
    }

    /// Runs the game and if it finishes returns the game at the end
    pub fn run_game(&mut self) {
        self.prepare_ui();
        while !self.game_over() {
            self.update();
        }
    }

    /// Make the snakes make their decision
    pub fn make_decision(&mut self) {
        let mut inputs: [f64; 5] = [0., 0., 0., 0., 0.];
        let cloned_apple = self.apple.clone();
        // Let each snake make a decision
        self.snakes.iter_mut().for_each(|s| {
            // First inputs are the distance to the apple from -1 to 1
            inputs[0] = distance_to_apple_x(s, cloned_apple);
            inputs[1] = distance_to_apple_y(s, cloned_apple);
            s.make_decision(&inputs)
        });
    }


    /// Updates the game. Should be called every tick
    pub fn update(&mut self) {
        let apple_coordinate = self.apple.get_coordinate();
        self.remove_if_dead();
        // And then update it
        let replace_apple = self
            .snakes
            .iter_mut()
            .map(|s| -> bool { s.update(apple_coordinate) })
            .any(|b| b);
        if replace_apple {
            self.apple = Apple::generate_apple();
        }
        // Let each snake make a decision
        self.make_decision();
    }

    fn render(&self) {}

    fn game_over(&self) -> bool {
        self.snakes.is_empty()
    }

    fn remove_if_dead(&mut self) {
        self.snakes.retain(|s| !s.is_colliding());
    }

    fn get_max_score(&self) -> usize {
        self.snakes.iter().map(|s| s.size()).max().unwrap()
    }
}
