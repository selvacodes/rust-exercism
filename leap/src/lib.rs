#![feature(conservative_impl_trait)]

pub fn is_leap_year(year: i32) -> bool {
    let divisible_by_4 = divisible_by(4);
    let divisible_by_100 = divisible_by(100);
    let divisible_by_400 = divisible_by(400);
    divisible_by_400(year) || (divisible_by_4(year) && not(divisible_by_100(year)))
}

fn divisible_by(n: i32) -> impl Fn(i32) -> bool {
    move |x| x % n == 0
}

fn not(boolean: bool) -> bool {
    !boolean
}
