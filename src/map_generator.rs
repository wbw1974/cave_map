use generation_rule;
use rand;
use rand::Rng;

pub fn create_random_cave_map(
    x_size: i32,
    y_size: i32,
    fill_percent: i32,
    rules: Vec<generation_rule::GenerationRule>,
) -> (Vec<Vec<bool>>, i32, i32) {
    // create random number generator
    let mut rng = rand::thread_rng();
    let (grid_1, grid_2) = inititalize_map(x_size, y_size, fill_percent, &mut rng);
    // walk through generation rules

    return (grid_1, 0, 0);
}

fn random_select<R: Rng>(rng: &mut R, fill_percent: i32) -> bool {
    if rng.gen::<i32>() % 100 < fill_percent {
        return true;
    } else {
        return false;
    }
}

fn inititalize_map<R: Rng>(x_size: i32, y_size: i32, fill_percent: i32, rng: &mut R) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut grid_1 = Vec::new();
    let mut grid_2 = Vec::new();
    for _i in 0..y_size {
        let mut item_1 = Vec::new();
        grid_1.push(item_1);
        let mut item_2 = Vec::new();
        grid_2.push(item_2);
    }

	for yi in 1..y_size as usize - 1 {
		for xi in 1..x_size as usize - 1 {
            grid_1[yi][xi] = random_select(rng, fill_percent);
        }
    }

    return (grid_1, grid_2);
}