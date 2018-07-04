use generation_rule;
use rand;
use rand::Rng;

const TILE_FLOOR: bool = true;
const TILE_WALL: bool = false;

/// Accessible function that creates a random cave map of a given rectangular size,
/// fill percentage, and Vector of GenerationRules.
/// 
/// Author: W. Brent Williams
/// Since: 2018-07-03
///
/// Example:
/// ```rust
/// let map = map_generator::create_random_cave_map(x_size, y_size, fill_percent, &rules);
/// ```
pub fn create_random_cave_map(
    x_size: i32,
    y_size: i32,
    fill_percent: i32,
    rules: &Vec<generation_rule::GenerationRule>,
) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let (mut grid_1, mut grid_2) = inititalize_map(x_size, y_size, fill_percent, &mut rng);
    // walk through generation rules

    for i in 0..rules.len() {
        let rule = &rules[i];
        for _j in 0..rule.times_to_apply {
            generation(&mut grid_1, &mut grid_2, rule, x_size, y_size);
        }
    }

    return grid_1;
}

/// Helper function that uses a random number generator to return true or false
/// based on a given fill percentage.
/// 
/// Example:
/// ```rust
/// grid_1[yi][xi] = random_select(rng, fill_percent);
/// ```
fn random_select<R: Rng>(rng: &mut R, fill_percent: i32) -> bool {
    if rng.gen::<u32>() % 100 < fill_percent as u32 {
        return TILE_WALL;
    } else {
        return TILE_FLOOR;
    }
}

/// Helper function that initializes the map with random data.
/// 
/// Example:
/// ```rust
/// let (mut grid_1, mut grid_2) = inititalize_map(x_size, y_size, fill_percent, &mut rng);
/// ```
fn inititalize_map<R: Rng>(
    x_size: i32,
    y_size: i32,
    fill_percent: i32,
    rng: &mut R,
) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut grid_1 = Vec::with_capacity(y_size as usize);
    let mut grid_2 = Vec::with_capacity(y_size as usize);
    for _i in 0..y_size {
        let mut item_1 = Vec::with_capacity(x_size as usize);
        let mut item_2 = Vec::with_capacity(x_size as usize);
        for _j in 0..x_size {
            item_1.push(TILE_WALL);
            item_2.push(TILE_WALL);
        }
        grid_1.push(item_1);
        grid_2.push(item_2);
    }

    for yi in 1..y_size as usize - 1 {
        for xi in 1..x_size as usize - 1 {
            grid_1[yi][xi] = random_select(rng, fill_percent);
        }
    }

    return (grid_1, grid_2);
}

/// Helper function that applies a cellular automata rule to the initial map
/// to generate the map.
/// 
/// Example:
/// ```rust
/// generation(&mut grid_1, &mut grid_2, rule, x_size, y_size);
/// ```
fn generation(
    grid_1: &mut Vec<Vec<bool>>,
    grid_2: &mut Vec<Vec<bool>>,
    rule: &generation_rule::GenerationRule,
    x_size: i32,
    y_size: i32,
) {
    for yi in 1..y_size - 1 {
        for xi in 1..x_size - 1 {
            let mut adj_count_r1 = 0;
            let mut adj_count_r2 = 0;

            for ii in -1..2 {
                for jj in -1..2 {
                    if grid_1[(yi + ii) as usize][(xi + jj) as usize] == true {
                        adj_count_r1 += 1;
                    }
                }
            }
            for ii in yi - 2..yi + 3 {
                for jj in xi - 2..xi + 3 {
                    if (ii - yi).abs() == 2 && (jj - xi).abs() == 2 {
                        continue;
                    }
                    if ii < 0 || jj < 0 || ii >= y_size || jj >= x_size {
                        continue;
                    }
                    if grid_1[ii as usize][jj as usize] == true {
                        adj_count_r2 += 1;
                    }
                }
            }
            if adj_count_r1 >= rule.rule_1 || adj_count_r2 <= rule.rule_2 {
                grid_2[yi as usize][xi as usize] = TILE_WALL;
            } else {
                grid_2[yi as usize][xi as usize] = TILE_FLOOR;
            }
        }
    }
    for yi in 1..y_size - 1 {
        for xi in 1..x_size - 1 {
            grid_1[yi as usize][xi as usize] = grid_2[yi as usize][xi as usize];
        }
    }
}

/// Accessible function that prints out the mathematical definition of the rules used to
/// generate a map.
/// 
/// Example:
/// ```rust
/// map_generator::print_function(&rules, fill_percent);
/// ```
pub fn print_function(rules: &Vec<generation_rule::GenerationRule>, fill_percent: i32) {
    let mut line_1 = String::from("W[0](p) = rand[0, 100) < ");
    line_1.push_str(&(fill_percent.to_string()));
    line_1.push_str("\n");

    let mut line_2 = String::new();
    for ii in 0..rules.len() {
        line_2.push_str("Repeat ");
        line_2.push_str(&(rules[ii].times_to_apply.to_string()));
        line_2.push_str(": W'(p) = R[1](p) >= ");
        line_2.push_str(&(rules[ii].rule_1.to_string()));
        if rules[ii].rule_2 >= 0 {
            line_2.push_str(" || R[2](p) <= ");
            line_2.push_str(&(rules[ii].rule_2.to_string()));
            line_2.push_str("\n");
        } else {
            line_2.push_str("\n");
        }
    }
    println!("{}", line_1);
    println!("{}", line_2);
}

/// Accessible function that prints out a generated map to STDIO using `#` for a wall
/// and `.` for a floor.
/// 
/// Example:
/// ```rust
/// map_generator::print_map(map);
/// ```
pub fn print_map(map: Vec<Vec<bool>>) {
    let mut line_1 = String::new();
    let size_x = map.len();
    let size_y = map[0].len();

    for yi in 0..size_y {
        for xi in 0..size_x {
            if map[xi][yi] == TILE_WALL {
                line_1.push_str("#");
            } else {
                line_1.push_str(".");
            }
        }
        line_1.push_str("\n");
    }

    println!("{}", line_1);
}
