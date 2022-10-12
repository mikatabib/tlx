use std::io;
use std::str::FromStr;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("input failed");
    let nums: Vec<i32> = inp.split_whitespace()
                            .filter_map(|i| i32::from_str(i).ok())
                            .collect();

    let (n, d) = (nums[0], nums[1]);

    let mut idx: i32 = -1;
    for it in 0..n {
        let mut inp = String::new();

        io::stdin()
            .read_line(&mut inp)
            .expect("fail to read line");

        let v: i32 = inp.trim()
                    .parse()
                    .expect("fail");

        if idx == -1 && v == d {
            idx = it;
        }
    }

    println!("{}", idx);
}
