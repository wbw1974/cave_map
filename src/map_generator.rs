use generation_rule;
use rand;
use rand::Rng;

pub fn create_random_cave_map(
    x_size: i32,
    y_size: i32,
    fill_percent: i32,
    rules: Vec<generation_rule::GenerationRule>,
) -> (Vec<Vec<bool>>, i32, i32) {
    let mut rng = rand::thread_rng();
    let (mut grid_1, mut grid_2) = inititalize_map(x_size, y_size, fill_percent, &mut rng);
    // walk through generation rules

    for i in 0..rules.len() {
        let rule = &rules[i];
        for _j in 0..rule.times_to_apply {
            generation(&mut grid_1, &mut grid_2, rule, x_size, y_size);
        }
    }

    return (grid_1, 0, 0);
}

fn random_select<R: Rng>(rng: &mut R, fill_percent: i32) -> bool {
    if rng.gen::<u32>() % 100 < fill_percent as u32 {
        return true;
    } else {
        return false;
    }
}

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
            item_1.push(false);
            item_2.push(false);
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

            for ii in -1..1 {
                for jj in -1..1 {
                    let idx_1 = (yi + ii) as usize;
                    let idx_2 = (xi + jj) as usize;
                    if grid_1[idx_1][idx_2] == true {
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
                    let idx_1 = ii as usize;
                    let idx_2 = jj as usize;
                    if grid_1[idx_1][idx_2] == true {
                        adj_count_r2 += 1;
                    }
                }
                if adj_count_r1 >= rule.rule_1 || adj_count_r2 <= rule.rule_2 {
                    grid_2[yi as usize][xi as usize] = true;
                } else {
                    grid_2[yi as usize][xi as usize] = false;
                }
            }
        }
    }
}
