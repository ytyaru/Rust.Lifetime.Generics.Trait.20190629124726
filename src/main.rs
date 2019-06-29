/*
 * Rustのジェネリクス、トレイト、ライフタイムを一度に。
 * CreatedAt: 2019-06-29
 */
fn main() {
    println!("{}", my_fn("A", "BB", "C"));
}
fn my_fn<'a, T>(p1: &'a str, p2: &'a str, p3: T) -> &'a str
    where T: std::fmt::Display
{
    println!("{}", p3);
    if p1.len() > p2.len() { p1 }
    else { p2 }
}
