use std::collections::HashMap;

use sm64ds_tunneling::{fx, fr};
use sm64ds_tunneling::fix::Fix;
use sm64ds_tunneling::player::{Player, Character};

fn tunnel_bfs(character: Character, position_y: Fix, horz_speed: Fix, jump_index: usize) -> Vec<(Vec<(bool, bool)>, i32)> {
    let mut to_search = vec![Player::new(character, position_y, horz_speed, jump_index)];
    // Each Yoshi stores the Yoshi it came from, whether B was held, and whether R was pressed.
    let mut visited = vec![(Player::new(character, position_y, horz_speed, jump_index), None)]
        .into_iter().collect::<HashMap<_, _>>();

    let mut end_players = vec![];
    while let Some(player) = to_search.pop() {
        let offset = (player.position_y() - fx!(-50.0)).val();
        if offset >= -62 && offset < 64 {
            end_players.push((player, offset));
            continue; // position is definitely negative
        }

        // For negative y, insert and stop.
        // Yoshi must reach -50 in one drop.
        if player.position_y() >= fx!(0.0) {
            let mut player_b = player.clone();
            player_b.update(true);
            let mut player_n = player.clone();
            player_n.update(false);
            let mut player_r = player.clone();
            player_r.update_ground_pound_until_below();
                
            for (new_player, held_b, pressed_r) in 
                [(player_b, true, false), (player_n, false, false), (player_r, true, true)]
            {
                visited.entry(new_player.clone()).or_insert_with(|| {
                    to_search.push(new_player);
                    Some((player.clone(), held_b, pressed_r))
                });
            }
        }
    }

    end_players.into_iter().map(|(mut player, offset)| {
        let mut inputs = vec![];
        while let Some((old_player, held_b, pressed_r)) = &visited[&player] {
            player = old_player.clone();
            inputs.push((*held_b, *pressed_r));
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
    let (character, pos, speed, jump_index) = (|| {
        let args = std::env::args().collect::<Vec<_>>();
        let character = args.get(1)?;
        let character = match character.as_str() {
            "mario" => Character::Mario,
            "luigi" => Character::Luigi,
            "wario" => Character::Wario,
            "yoshi" => Character::Yoshi,
            _ => None?
        };
        let pos = parse_fix(&args.get(2)?).filter(|x| *x >= fx!(0.0))?;
        let speed = parse_fix(&args.get(3)?)?;
        if !["1", "2", "3"].contains(&args.get(4)?) {
            return None;
        }
        let jump_index = args[4].parse::<usize>().ok()? - 1;
        Some((character, pos, speed, jump_index))
    })().unwrap_or_else(|| {
        println!(concat!(
            "floor-tunnel <character> <pos_y> <hspeed> <jump_number>\n",
            "\n",
            "character:   \"mario\", \"luigi\", \"wario\", or \"yoshi\"\n",
            "pos_y:       Nonnegative starting offset y from the clipping floor\n",
            "hspeed:      Initial horizontal speed\n",
            "jump_number: 1 if single jump, 2 if double jump, 3 if triple jump\n",
            "\n",
            "pos_y and hspeed can be given in integer, hex, or fxu. So the following are equivalent:\n",
            "floor-tunnel yoshi 204800 40960 2\n",
            "floor-tunnel yoshi 0x32000 0xa000 2\n",
            "floor-tunnel yoshi 50.0fxu 10.0fxu 2\n",
            "\n",
            "Examples:\n",
            "floor-tunnel mario 0 0 1\n",
            "finds all floor tunneling setups where Mario starts on the same floor he tunnels through\n",
            "at a standstill and tunnels with a single jump.\n",
            "\n",
            "floor-tunnel luigi 50.0fxu 20.0fxu 2\n",
            "finds all floor tunneling setups where Luigi starts 50 fxu above the floor he tunnels through,\n",
            "running at 20 fxu/frame, and tunnels with a double jump.\n",
            "The setups start with the second jump, not the first one!\n",
            "\n",
            "Example output:\n",
            "v1, ^2, v3, ^ (-4 ≤ offset < 5)\n",
            "Hold B for 1 frame, then release for 2 frames, then hold for 3 frames, then release until you tunnel.\n",
            "Works if the floor offset is at least -4 and less than 5.\n",
            "\n",
            "v6, gp (-7 ≤ offset < 8)\n",
            "Hold B for 6 frames, then ground pound.\n",
            "Works if the floor offset is at least -7 and less than 8.\n",
        ));
        panic!("Invalid input");
    });

    let setups = tunnel_bfs(character, pos, speed, jump_index);

    if setups.is_empty() {
        println!("No setups found.");
    }

    'outer: for (inputs, offset) in setups {
        let mut prev = true;
        let mut count = 0;
        for (held_b, pressed_r) in inputs {
            if prev == held_b && !pressed_r {
                count += 1;
            } else {
                print!("{}{}, ", if prev {"v"} else {"^"}, count);
                count = 1;
                prev = held_b;

                if pressed_r {
                    println!("gp ({} ≤ offset < {})", -offset, 64 - offset);
                    continue 'outer;
                }
            }
        }
        println!("{} ({} ≤ offset < {})", if prev {"v"} else {"^"}, -offset, 64 - offset);
    }
}