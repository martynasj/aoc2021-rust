pub fn calc_increase_count(input: &String) -> i32 {
    let r = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|slice| match slice {
            [a, b] => return b > a,
            _ => false,
        })
        .count();

    return r as i32;
}

#[test]
fn calculates_result() {
    let input = include_str!("./data.txt");
    assert_eq!(calc_increase_count(&input.to_string()), 1451)
}
