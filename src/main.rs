mod fix;
mod yoshi;
use std::collections::HashMap;

use fix::Fix;
use yoshi::Yoshi;

fn yoshi_bfs(double_jump: bool) -> Vec<bool> {
    let mut to_search = vec![Yoshi::new(double_jump)];
    // Each Yoshi stores the Yoshi it came from and whether B was held.
    let mut visited = vec![(Yoshi::new(double_jump), None)].into_iter().collect::<HashMap<_, _>>();

    let mut end_yoshi = None;
    while let Some(yoshi) = to_search.pop() {
        if yoshi.position_y().val().div_euclid(64) == -50 * 64 {
            println!("Yoshi position: {0} aka {0:8x}", yoshi.position_y());
            end_yoshi = Some(yoshi);
            break;
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

    if let Some(mut yoshi) = end_yoshi {
        let mut inputs = vec![];
        while let Some((old_yoshi, held_b)) = &visited[&yoshi] {
            yoshi = old_yoshi.clone();
            inputs.push(*held_b);
        }

        inputs.reverse();
        inputs
    } else {
        panic!("Ahh! No setup!")
    }
}

fn main() {
    let inputs = yoshi_bfs(false);
    //println!("Inputs: {:?}", inputs.into_iter().map(|b| if b {1} else {0}).collect::<Vec<_>>());
    let mut prev = true;
    let mut count = 0;
    for input in inputs {
        if prev == input {
            count += 1;
        } else {
            println!("{} for {} frames", if prev {"Hold"} else {"Let go"}, count);
            count = 1;
            prev = input;
        }
    }
    println!("{} for {} frames", if prev {"Hold"} else {"Let go"}, count);
}
