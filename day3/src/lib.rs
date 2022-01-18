fn calc_result(input: &String) -> i32 {
    let width = input.lines().next().unwrap().bytes().count();
    let line_count = input.lines().count();

    let mut counter = vec![0; width];

    input.lines().for_each(|line| {
        line.bytes().enumerate().for_each(|(idx, c)| {
            if c == b'1' {
                counter[idx] = counter[idx] + 1
            }
        })
    });

    let gamma_rate_bin: Vec<u8> = counter
        .iter()
        .map(|&count| if count > line_count / 2 { 1 } else { 0 })
        .collect();

    let pwr_str = gamma_rate_bin
        .iter()
        .map(|&bit| if bit == 0 { '1' } else { '0' })
        .collect::<String>();

    let g = gamma_rate_bin
        .iter()
        .map(|f| f.to_string())
        .collect::<String>();

    let gamma_rate = isize::from_str_radix(&g, 2).unwrap() as i32;
    let pwr = isize::from_str_radix(&pwr_str, 2).unwrap() as i32;

    gamma_rate * pwr
}

#[test]
fn it_works() {
    let input = include_str!("../input.txt");
    assert_eq!(calc_result(&input.to_string()), 3320834);
}
