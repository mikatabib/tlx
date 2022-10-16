use std::io;

fn main() {
    let tab64: [u32; 64] = [
        63,  0, 58,  1, 59, 47, 53,  2,
        60, 39, 48, 27, 54, 33, 42,  3,
        61, 51, 37, 40, 49, 18, 28, 20,
        55, 30, 34, 11, 43, 14, 22,  4,
        62, 57, 46, 52, 38, 26, 32, 41,
        50, 36, 17, 19, 29, 10, 13, 21,
        56, 45, 25, 31, 35, 16,  9, 12,
        44, 24, 15,  8, 23,  7,  6,  5
    ];

    let mut buf = String::new();
    let _ = io::stdin()
                .read_line(&mut buf)
                .unwrap();

    let b: u64 = buf.trim()
                    .parse()
                    .unwrap();

    let mut buf = String::new();
    let _ = io::stdin()
                .read_line(&mut buf)
                .unwrap();

    let k: u64 = buf.trim()
                    .parse()
                    .unwrap();

    let mut rem = k-b;
    while rem > 0 {
        let m = 2_u64.pow(log2_64(rem, &tab64));
        println!("{}", m);
        rem -= m;
    }
}

// https://stackoverflow.com/questions/11376288/fast-computing-of-log2-for-64-bit-integers
fn log2_64 (mut value: u64, tab64: &[u32; 64]) -> u32 {
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value |= value >> 32;

    let v: u64 = ((value - (value >> 1)) * 0x07EDD5E59A4E28C2) >> 58;
    tab64[v as usize]
}
