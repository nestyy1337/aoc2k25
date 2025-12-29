fn main() {
    let x = include_str!("./input.txt");
    let mut start: i32 = 50;
    let mut zero_count: i32 = 0;
    let click = 1;
    x.lines()
        .filter_map(|l| {
            let (direction, count_str) = l.split_at(1);
            let count: i32 = count_str.trim().parse().ok()?;
            Some((direction, count))
        })
        .for_each(|(direction, count)| {
            for _ in 0..count {
                if direction == "R" {
                    start = (start + click) % 100;
                } else if direction == "L" {
                    start = ((start - click) % 100 + 100) % 100;
                }
                if start == 0 {
                    zero_count += 1;
                }
            }
        });
    println!("{}", zero_count);
}
