mod fix;
mod yoshi;
use std::collections::HashMap;

use fix::Fix;
use yoshi::Yoshi;

fn yoshi_bfs(horz_speed: Fix, double_jump: bool) -> Vec<(Vec<bool>, i32)> {
    let mut to_search = vec![Yoshi::new(horz_speed, double_jump)];
    // Each Yoshi stores the Yoshi it came from and whether B was held.
    let mut visited = vec![(Yoshi::new(horz_speed, double_jump), None)].into_iter().collect::<HashMap<_, _>>();

    let mut end_yoshis = vec![];
    while let Some(yoshi) = to_search.pop() {
        let offset = (yoshi.position_y() - fx!(-50.0)).val();
        if offset >= -62 && offset <= 64 {
            end_yoshis.push((yoshi, offset));
            continue; // position is definitely negative
        }

        // For negative y, insert and stop.
        // Yoshi must reach -50 in one drop.
        if yoshi.position_y() >= fx!(0.0) {
            let mut yoshi_b = yoshi.clone();
            yoshi_b.update(true);
            let mut yoshi_n = yoshi.clone();
            yoshi_n.update(false);
                
            for (new_yoshi, held_b) in vec![(yoshi_b, true), (yoshi_n, false)] {
                visited.entry(new_yoshi.clone()).or_insert_with(|| {
                    to_search.push(new_yoshi);
                    Some((yoshi.clone(), held_b))
                });
            }
        }
    }

    end_yoshis.into_iter().map(|(mut yoshi, offset)| {
        let mut inputs = vec![];
        while let Some((old_yoshi, held_b)) = &visited[&yoshi] {
            yoshi = old_yoshi.clone();
            inputs.push(*held_b);
        }

        inputs.reverse();
        (inputs, offset)
    }).collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let speed_str = &args[1];
    let speed = if speed_str.ends_with("fxu") {
        fx!(speed_str[..speed_str.len() - 3].parse().expect("Expected valid number before fxu"))
    } else if speed_str.starts_with("0x") {
        fr!(i64::from_str_radix(&speed_str[2..], 16).expect("Expected valid hexadecimal integer") as i32)
    } else {
        fr!(speed_str.parse().expect("Expected valid integer"))
    };
    let double_jump = args[2] == "2";

    let setups = yoshi_bfs(speed, double_jump);

    if setups.is_empty() {
        println!("No setups found.");
    }

    for (inputs, offset) in setups {
        let mut prev = true;
        let mut count = 0;
        for input in inputs {
            if prev == input {
                count += 1;
            } else {
                print!("{}{}, ", if prev {"v"} else {"^"}, count);
                count = 1;
                prev = input;
            }
        }
        println!("{} ({} ≤ offset < {})", if prev {"v"} else {"^"}, -offset, 64 - offset);
    }
}
