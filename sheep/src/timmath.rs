use super::find::get_input;
use colored::*;
use rand::Rng;

fn all_equal(vec: &Vec<usize>) -> bool {
    let first = match vec.first() {
        Some(value) => value,
        None => return true,
    };
    vec.iter().skip(1).all(|val| val == first)
}

fn steps_until_all_equal(value_count: usize) -> usize {
    let mut rng = rand::thread_rng();
    let mut values = Vec::with_capacity(value_count);
    for i in 0..value_count {
        values.push(i);
    }
    let mut steps = 0;
    while !all_equal(&values) {
        let index1 = rng.gen_range(0, value_count);
        let index2 = rng.gen_range(0, value_count);
        if index1 != index2 {
            values[index1] = values[index2];
            steps += 1;
        }
    }
    steps
}

fn get_avg_equilibrium_steps(value_count: usize, trials: usize) -> f64 {
    let total_steps = (0..trials)
        .map(|_| steps_until_all_equal(value_count))
        .sum::<usize>() as f64;
    total_steps / (trials as f64)
}

pub fn main() {
    loop {
        println!("Please give (1) the number of values, and (2) the number of trials.");
        match (get_input().trim().parse::<usize>(), get_input().trim().parse::<usize>()) {
            (Ok(value_count), Ok(trials)) => {
                let average_steps = get_avg_equilibrium_steps(value_count, trials);
                println!("{} steps", average_steps);
            }
            (Err(err), _) => println!("{} Failed to parse number of values: {}", "[your error]".red(), err),
            (_, Err(err)) => println!("{} Failed to parse number of trials: {}", "[your error]".red(), err),
        }
    }
}
