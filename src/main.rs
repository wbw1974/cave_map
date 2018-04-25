// Task 1: Read in parameters. Make much clearer that C source.
mod generation_rule;
mod simple_step_range;
mod map_generator;
use std::env;
use std::process;
use std::num;

const EXIT_FAILURE: i32 = 0x0100;
const EXIT_SUCCESS: i32 = 0x0000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (x_size, y_size, fill_percent, rules) = process_args(&args);

    println!("x_size: {:?}", x_size);
    println!("y_size: {:?}", y_size);
    println!("fill_percent: {:?}", fill_percent);

    for (i, rule) in rules.iter().enumerate() {
        println!("rule {:?}: ({:?})", i + 1, rule);
    }

    let (map, x, y) = map_generator::create_random_cave_map(x_size, y_size, fill_percent, rules);

    // TODO: Expand below into a real output printer.
    println!("output: {:?}, {:?}, {:?}", map, x, y);

    process::exit(EXIT_SUCCESS);
}

fn process_args(args: &[String]) -> (i32, i32, i32, Vec<generation_rule::GenerationRule>) {
    if args.len() < 6 || (args.len() - 1 - 3) % 3 != 0 {
        println!(
            "Usage: {:?} x_size y_size fill_% (rule_1 rule_2 number_of_times_to_apply)+",
            &args[0]
        );
        process::exit(EXIT_FAILURE);
    }

    let arg1 = process_argument_result(&args[0], args[1].parse::<i32>());
    let arg2 = process_argument_result(&args[0], args[2].parse::<i32>());
    let arg3 = process_argument_result(&args[0], args[3].parse::<i32>());

    let mut rules = Vec::<generation_rule::GenerationRule>::new();

    for z in simple_step_range::SimpleStepRange::new(4 as usize, args.len(), 3 as usize) {
        let a1 = process_argument_result(&args[0], args[z].parse::<i32>());
        let a2 = process_argument_result(&args[0], args[z + 1].parse::<i32>());
        let a3 = process_argument_result(&args[0], args[z + 2].parse::<i32>());
        let rule = generation_rule::GenerationRule::new(a1, a2, a3);
        rules.push(rule);
    }

    return (arg1, arg2, arg3, rules);
}

fn process_argument_result(program: &str, result: Result<i32, num::ParseIntError>) -> i32 {
    match result {
        Ok(n) => return n,
        Err(e) => {
            println!(
                "Error found while parsing an integer: {:?}. 
                Was not able to parse the arguments passed to {:?}.",
                e, program
            );
            process::exit(EXIT_FAILURE);
        }
    }
}
