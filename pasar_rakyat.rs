use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("input fail");

    let n: i32 = inp.trim()
                .parse()
                .expect("parse failed");

    let mut merchants = Vec::new();
    for _ in 0..n {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("input fail");

        let v: i32 = inp.trim()
                    .parse()
                    .expect("parse failed");
        merchants.push(v)
    }

    let mut com = merchants[0];
    for it in &mut merchants[1..] {
        let num = com * (*it);
        com = num / gcd(com, *it);
    }
    println!("{com}");
}

fn gcd(mut n: i32, mut m: i32) -> i32 {
    while m != 0 {
        if m < n {
            m ^= n;
            n ^= m;
            m ^= n;
        }
        m = m % n;
    }
    n
}
