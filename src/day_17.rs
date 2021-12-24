fn step (x: i32, y: i32, vx: i32, vy:i32) -> (i32, i32, i32, i32) {
    return (
        x + vx,
        y + vy,
        if vx > 0 { vx-1 } else if vx < 0 { vx+1 } else { 0 },
        vy - 1
    );
}

fn max_height_for_target (x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> i32 {
    let (target_x_min, target_x_max) = (x_min, x_max);
    let (target_y_min, target_y_max) = (y_min, y_max);

    let mut max_y = 0;

    let needed_vx = ((target_x_min*2) as f64).sqrt() as i32;

    for init_vx in needed_vx..=target_x_max {
        for init_vy in -10..200 {

            let mut x = 0;
            let mut y = 0;
            let mut vx = init_vx;
            let mut vy = init_vy;

            let mut local_max_y = 0;

            while (x <= target_x_max) & (y >= target_y_min) {
                if y > local_max_y {
                    local_max_y = y;
                }

                if (target_x_min..=target_x_max).contains(&x) & (target_y_min..=target_y_max).contains(&y) {
                    if local_max_y > max_y {
                        max_y = local_max_y;
                    }
                    break;
                }

                let updated = step(x, y, vx, vy);
                x = updated.0;
                y = updated.1;
                vx = updated.2;
                vy = updated.3;

            }
        }
    }

    return max_y;
}

#[test]
fn puzzle_1_confers_to_example () {
    assert_eq!(max_height_for_target(20, 30, -10, -5), 45);
}

#[test]
fn puzzle_1_confers_to_puzzle_result () {
    assert_eq!(max_height_for_target(288, 330, -96, -50), 4560);
}

fn initial_velocities_hitting_target (x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Vec<(i32, i32)> {
    let (target_x_min, target_x_max) = (x_min, x_max);
    let (target_y_min, target_y_max) = (y_min, y_max);

    let mut initial_velocities_hitting_target: Vec<(i32, i32)> = Vec::new();

    let needed_vx = ((target_x_min*2) as f64).sqrt() as i32;

    for init_vx in needed_vx..=target_x_max {
        for init_vy in -200..200 {

            let mut x = 0;
            let mut y = 0;
            let mut vx = init_vx;
            let mut vy = init_vy;

            while (x <= target_x_max) & (y >= target_y_min) {

                if (target_x_min..=target_x_max).contains(&x) & (target_y_min..=target_y_max).contains(&y) {
                    initial_velocities_hitting_target.push((init_vx, init_vy));
                    break;
                }

                let updated = step(x, y, vx, vy);
                x = updated.0;
                y = updated.1;
                vx = updated.2;
                vy = updated.3;

            }
        }
    }

    return initial_velocities_hitting_target;
}

#[test]
fn puzzle_e_confers_to_example () {
    assert_eq!(initial_velocities_hitting_target(20, 30, -10, -5).len(), 112);
}

fn main () {
    let (target_x_min, target_x_max) = (288, 330);
    let (target_y_min, target_y_max) = (-96, -50);

    let max_y = max_height_for_target(target_x_min, target_x_max, target_y_min, target_y_max);
    let initial_velocities_hitting_target = initial_velocities_hitting_target(target_x_min, target_x_max, target_y_min, target_y_max);

    // 124750 is incorrect
    println!("Max height {:?}", max_y);
    println!("N initial velocities hitting target {:?}", initial_velocities_hitting_target.len());
}