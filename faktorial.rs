use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("input failed");

    let n: i32 = inp.trim()
                    .parse()
                    .expect("parse failed");

    let k = (1.6094_f32 * (n as f32)) as i32;

    let mut zeroes = 0;
    let mut pow = 5;

    for _ in 0..k {
        let cur = n / pow;
        if cur == 0 {break}
        zeroes += cur;
        pow *= 5;
    }
    println!("{}", zeroes);
}
