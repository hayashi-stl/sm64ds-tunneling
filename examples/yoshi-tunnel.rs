use std::collections::HashMap;

use sm64ds_tunneling::{fx, fr};
use sm64ds_tunneling::fix::Fix;
use sm64ds_tunneling::yoshi::Yoshi;

fn yoshi_bfs(position_y: Fix, horz_speed: Fix, double_jump: bool) -> Vec<(Vec<bool>, i32)> {
    let mut to_search = vec![Yoshi::new(position_y, horz_speed, double_jump)];
    // Each Yoshi stores the Yoshi it came from and whether B was held.
    let mut visited = vec![(Yoshi::new(position_y, horz_speed, double_jump), None)].into_iter().collect::<HashMap<_, _>>();

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

fn parse_fix(string: &str) -> Option<Fix> {
    Some(if string.ends_with("fxu") {
        fx!(string[..string.len() - 3].parse().ok()?)
    } else if string.starts_with("0x") {
        fr!(i64::from_str_radix(&string[2..], 16).ok()? as i32)
    } else {
        fr!(string.parse().ok()?)
    })
}

fn main() {
    let (pos, speed, double_jump) = (|| {
        let args = std::env::args().collect::<Vec<_>>();
        let pos = parse_fix(&args.get(1)?).filter(|x| *x >= fx!(0.0))?;
        let speed = parse_fix(&args.get(2)?)?;
        if !["1", "2"].contains(&args.get(3)?) {
            return None;
        }
        let double_jump = args[3] == "2";
        Some((pos, speed, double_jump))
    })().unwrap_or_else(|| {
        println!(concat!(
            "yoshi-tunnel <pos_y> <hspeed> <jump_number>\n",
            "\n",
            "pos_y:       Nonnegative starting offset y from the clipping floor\n",
            "hspeed:      Initial horizontal speed\n",
            "jump_number: 1 if single jump, 2 if double jump\n",
        ));
        panic!("Invalid input");
    });

    let setups = yoshi_bfs(pos, speed, double_jump);

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