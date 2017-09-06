pub fn nth(n: i32) -> Result<i32, String> {
    let y = (1..).filter(|&x| is_prime(x)).take(n as usize).last();
    match y {
        Some(n) => Ok(n),
        None => Err("Invalid Input".to_string()),
    }
}

fn is_prime(n: i32) -> bool {
    let is_not_prime = (2..n / 2 + 1)
        .take_while(|x| n % x != 0)
        .collect::<Vec<i32>>();

    (is_not_prime.len() as i32) == (n / 2 - 1)
}
