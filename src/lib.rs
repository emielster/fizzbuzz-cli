pub fn fizzbuzz(number: u32) -> String {
    let divisible_by_three = number % 3 == 0;
    let divisible_by_five = number % 5 == 0;

    if divisible_by_three && divisible_by_five {
        return "FizzBuzz".to_string();
    } else if divisible_by_three {
        return "Fizz".to_string();
    } else if divisible_by_five {
        return "Buzz".to_string();
    } else {
        return number.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz_works() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(6), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn buzz_works() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn fizzbuzz_works() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(60), "FizzBuzz");
    }

    #[test]
    fn normal_numbers_work() {
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(7), "7");
        assert_eq!(fizzbuzz(11), "11");
    }
}
