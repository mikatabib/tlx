use std::io;
use std::str::FromStr;

fn reverse(mut n: i32) -> i32{
    let digit = 1 + ((n as f32).log10()) as i32;
    let mut rev = 0;

    for it in (0..digit).rev() {
        let pow = (10_f32.powf(it as f32)) as i32;
        rev += n % 10 * pow;
        n /= 10;
    }
    return rev
}

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read input");

    let n = i32::from_str(&inp.trim()).unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("failed to read input");

        let v = i32::from_str(&inp.trim()).unwrap();
        nums.push(v);
    }

    for it in nums { 
        println!("{}", reverse(it));
    }
}
