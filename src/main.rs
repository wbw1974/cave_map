extern crate rand;

mod generation_rule;
mod simple_step_range;
mod map_generator;
use std::env;
use std::process;
use std::num;

const EXIT_FAILURE: i32 = 0x0100;
const EXIT_SUCCESS: i32 = 0x0000;

/// Cave Map Generator - A cellular automata map generator.
/// Generates maps that look like natural caves.
/// Author: W. Brent Williams
/// Since: 2018-07-03 
/// Ported from C source at [Cellular Automata Method for Generating Random Cave-Like Levels][1]
/// 
/// Template: 
/// `cave_map x_size y_size fill_% (rule_1 rule_2 number_of_times_to_apply)+`
/// 
/// Example:
/// ```bash
/// cave_map 45 45 45 5 1 6
/// W[0](p) = rand[0, 100) < 45
/// 
/// Repeat 6: W'(p) = R[1](p) >= 5 || R[2](p) <= 1
/// (Generated map)
/// ```
/// [1]: http://roguebasin.roguelikedevelopment.org/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels
fn main() {
    let args: Vec<String> = env::args().collect();
    let (x_size, y_size, fill_percent, rules) = process_args(&args);
    let map = map_generator::create_random_cave_map(x_size, y_size, fill_percent, &rules);
    map_generator::print_function(&rules, fill_percent);
    map_generator::print_map(map);
    process::exit(EXIT_SUCCESS);
}

/// Function that takes an array of arguments pulled from the command line by `main()`
/// and, if the arguments match the expected pattern, returns a tuple containing three i32 and
/// a Vec of GenerationRules.
///  
/// Example:
/// ```rust
/// let (x_size, y_size, fill_percent, rules) = process_args(&args);
/// ```
fn process_args(args: &[String]) -> (i32, i32, i32, Vec<generation_rule::GenerationRule>) {
    if args.len() < 6 || (args.len() - 4) % 3 != 0 {
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

/// Helper function for `process_args()` that unwraps the results of a call to
/// `str.parse::<i32>()` and either produces the i32 or writes an error message
/// to output and exits with an `EXIT_FAILURE`.
/// 
/// Example:
/// ```rust
/// let arg1 = process_argument_result(&args[0], args[1].parse::<i32>());
/// ```
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
