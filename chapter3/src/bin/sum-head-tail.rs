fn sum(numbers: &[i32]) -> i32 {
    if numbers.is_empty() {
        // BASE CASE
        0
    } else {
        // RECURSIVE CASE
        let head = numbers[0];
        let tail = &numbers[1..];
        head + sum(tail)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[5, 2, 4, 8]), 19);
        assert_eq!(sum(&[1, 10, 100, 1000]), 1111);
    }
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("The sum of {nums:?} is {}", sum(&nums));

    let nums = [5, 2, 4, 8];
    println!("The sum of {nums:?} is {}", sum(&nums));

    let nums = [1, 10, 100, 1000];
    println!("The sum of {nums:?} is {}", sum(&nums));
}
