#![allow(dead_code)]

mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;

fn main() {
    println!("Hello, world!");
    // question1::fizzbuzz();
    // question3::question3();
    // question4::question4();
    // question5::question5();
    // question6::question6();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(1 + 1, 2);
    }
}
