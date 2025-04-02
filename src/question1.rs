// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。

pub fn fizzbuzz() {
    let n: i32 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(5);

    for i in 1..=n {
        println!("{}", output(i));
    }
}

fn output(index: i32) -> String {
    if index % 3 == 0 && index % 5 == 0 {
        format!("{} FizzBuzz", index)
    } else if index % 3 == 0 {
        format!("{} Fizz", index)
    } else if index % 5 == 0 {
        format!("{} Buzz", index)
    } else {
        format!("{}", index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        assert!(output(2) == "2".to_string());
        assert!(output(10) == "10 Buzz".to_string());
        assert!(output(15) == "15 FizzBuzz".to_string());
    }

    #[test]
    fn test_fizz() {
        assert_eq!(output(3), "3 Fizz".to_string());
        assert_eq!(output(6), "6 Fizz".to_string());
    }

    #[test]
    fn test_buzz() {
        assert_eq!(output(5), "5 Buzz".to_string());
        assert_eq!(output(20), "20 Buzz".to_string());
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(output(15), "15 FizzBuzz".to_string());
        assert_eq!(output(30), "30 FizzBuzz".to_string());
    }

    #[test]
    fn test_non_fizzbuzz() {
        assert_eq!(output(1), "1".to_string());
        assert_eq!(output(7), "7".to_string());
        assert_eq!(output(11), "11".to_string());
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(output(0), "0 FizzBuzz".to_string()); // 0 is divisible by both 3 and 5
        assert_eq!(output(-3), "-3 Fizz".to_string());
        assert_eq!(output(-5), "-5 Buzz".to_string());
        assert_eq!(output(-15), "-15 FizzBuzz".to_string());
    }
}