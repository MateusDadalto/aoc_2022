#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../inputs/example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    let decryption_key = 811589153;

    // let input: Vec<i64> = EXAMPLE.lines().map(|l| l.parse::<i64>().unwrap() * decryption_key).collect();
    let input: Vec<i64> = INPUT.lines().map(|l| l.parse::<i64>().unwrap() * decryption_key).collect();
    let mut shifter: Vec<_> = (0..input.len()).collect();
    let mut zero_idx = 0;

    for _ in 0..10  {
        for (i, &n) in input.iter().enumerate() {
            if n == 0 {
                zero_idx = i;
                continue;
            }
    
            let mixed_idx = shifter.iter().position(|m_i| *m_i == i).unwrap();
    
            shifter.remove(mixed_idx);
            // living and learning, my overcomplicated solution can be instead this simple F_ing method
            // damn you modulus and remainder
            let new_mixed_idx = (mixed_idx as i64 + n).rem_euclid(shifter.len() as i64) as usize;
    
            shifter.insert(new_mixed_idx, i);
        }
    }
    
    let zero_idx = shifter.iter().position(|s_i| *s_i == zero_idx).unwrap();
    println!("0 index {zero_idx}");

    let x = input[shifter[(zero_idx + 1000) % shifter.len()]];
    let y = input[shifter[(zero_idx + 2000) % shifter.len()]];
    let z = input[shifter[(zero_idx + 3000) % shifter.len()]];

    println!(
        "Day 20 part 1: 1000th: {x} | 2000th: {y} | 3000th: {z} | {}",
        x + y + z
    );
}
