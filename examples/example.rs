extern crate neat_gru;

use itertools::Itertools;
use neat_gru::game::Game;
use neat_gru::neural_network::nn::NeuralNetwork;
use neat_gru::topology::topology::Topology;
use neat_gru::train::train::Train;
struct Player {
    pub net: NeuralNetwork<f64>,
}

impl Player {
    pub fn new(net: NeuralNetwork<f64>) -> Player {
        Player { net }
    }
    /// Runs all the inputs and calculates the outputs
    fn run(&mut self) -> f64 {
        // Get the inputs
        let inputs = XOR::get_inputs();
        // Calculate a score for every input
        let outputs: Vec<f64> = inputs.iter().map(|i| self.net.compute(i)[0]).collect();
        let mut scores: Vec<f64> = vec![];
        for idx in 0..(inputs.len() - 1) {
            scores.push(compute_score(&inputs[idx], outputs[idx]));
        }
        // And return the sum of the scores
        scores.iter().sum()
    }
}

struct Simulation {
    players: Vec<Player>,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            players: Vec::new(),
        }
    }
}

struct XOR {
    inputs: Vec<Vec<f64>>,
    answers: Vec<Vec<f64>>,
}

impl XOR {
    pub fn new() -> Self {
        XOR {
            inputs: vec![
                vec![0.0, 0.0],
                vec![1.0, 1.0],
                vec![1.0, 0.0],
                vec![0.0, 1.0],
            ],
            answers: vec![vec![0.0], vec![0.0], vec![1.0], vec![1.0]],
        }
    }
    fn get_inputs<'a>() -> &'a [[f64; 2]; 4] {
        &[[0.0, 0.0], [1.0, 1.0], [1.0, 0.0], [0.0, 1.0]]
    }
}

/// Computes the score with given inputs and one output
fn compute_score(inputs: &[f64], output: f64) -> f64 {
    // https://en.wikipedia.org/wiki/XOR_gate
    // Returns 1.0 for a wrong output and 0.0 for a right output. Should be used as a score
    let inputs: Vec<&f64> = inputs.into_iter().collect_vec();
    if (*inputs[0] == 0.0 && *inputs[1] == 1.0) || (*inputs[0] == 1.0 && *inputs[1] == 0.0) {
        if output == 1.0 {
            return 1.0;
        }
    } else {
        if output == 0.0 {
            return 1.0;
        }
    }
    0.0
}

impl Game<f64> for Simulation {
    /// Loss function
    fn run_generation(&mut self) -> Vec<f64> {
        let inputs = XOR::get_inputs();
        self.players.iter_mut().map(|p| p.run()).collect()
    }

    /// Reset networks
    fn reset_players(&mut self, nets: Vec<NeuralNetwork<f64>>) {
        self.players.clear();
        self.players = nets.into_iter().map(Player::new).collect();
    }

    /// Called at the end of training
    fn post_training(&mut self, history: &[Topology<f64>]) {
        // Iter on best topologies and upload the best one
    }
}

const INPUT_COUNT: usize = 2;
const OUTPUT_COUNT: usize = 1;
const NB_GENERATIONS: usize = 100;
const HIDDEN_LAYERS: usize = 2;
const MAX_INDIVIDUALS: usize = 200;
fn run_sim() {
    let mut sim = Simulation::new();

    let mut runner = Train::new(&mut sim);
    runner
        .inputs(INPUT_COUNT)
        .outputs(OUTPUT_COUNT)
        .iterations(NB_GENERATIONS)
        .max_layers(HIDDEN_LAYERS + 2)
        .max_per_layers(HIDDEN_LAYERS)
        .max_individuals(MAX_INDIVIDUALS)
        .delta_threshold(2.) // Delta parameter from NEAT paper
        .formula(0.8, 0.8, 0.3) // c1, c2 and c3 from NEAT paper
        .access_train_object(Box::new(|train| {
            let species_count = train.species_count();
            println!("Species count: {}", species_count);
        })) // Callback called after `reset_players` that gives you access to the train object during training
        .start(); // .start_async().await for async version
}
fn main() {
    run_sim();
}
