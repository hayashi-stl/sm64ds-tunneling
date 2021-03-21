mod fix;
mod yoshi;
use fix::Fix;
use yoshi::Yoshi;

fn main() {
    for i in 1..100 {
        for j in i..100 {
            let mut yoshi = Yoshi::new(true);
            let mut frame = 0;
            while yoshi.position_y() >= fx!(0.0) {
                yoshi.update(frame < i || frame >= j);
                let val = yoshi.position_y().val();
                frame += 1;

                if val.div_euclid(64).rem_euclid(64) == 0 && [0x3c, 0x7e, 0xc4, 0x10e, 0x15c].contains(&val.div_euclid(4096)) {
                    println!("Let go @ {0:3}, re-press @ {1:3}, ground pound @ {2:3}. Position Y: {3:8x} aka {3}", i, j, frame, yoshi.position_y());
                }
            }
        }
    }
    //println!();

    //let mut yoshi = Yoshi::new(true);
    //let mut frame = 0;
    //while yoshi.position_y() >= fx!(0.0) {
    //    yoshi.update(frame < 3 || frame >= 8);
    //    let val = yoshi.position_y().val();
    //    frame += 1;

    //    println!("Beginning of frame {0:3}. Position Y: {1:8x} aka {1}", frame, yoshi.position_y());
    //}
}
