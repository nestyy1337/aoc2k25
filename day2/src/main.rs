fn main() {
    let regex = fancy_regex::Regex::new(r"^(.+)\1+$").unwrap();
    let input = include_str!("./input.txt");
    let pattern = |num: usize| {
        let s = num.to_string();
        if regex.is_match(&s).unwrap() {
            return true;
        }
        false
    };
    let mut sum: usize = 0;
    input
        .split_terminator(',')
        .filter_map(|range| {
            let (s, e) = range.split_once('-').unwrap();
            let start: usize = s.parse().ok()?;
            let end: usize = e.parse().ok()?;
            Some((start, end))
        })
        .for_each(|(start, end)| {
            for num in start..=end {
                if pattern(num) {
                    sum += num as usize;
                }
            }
        });
    println!("{}", sum);
}
