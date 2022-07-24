fn sum_even_fibonacci_numbers(limit: u64) -> u64 {
    let mut sum = 0;

    let mut last_fibonacci_number = 0;
    let mut current_fibonacci_number = 2;

    while current_fibonacci_number < limit {
        sum += current_fibonacci_number;

        // Calculate next fibonacci number by the formula F(n) = 4 * F(n-3) * F(n-6).
        // The formula is calculated by expanding the fibonacci formula F(n) = F(n-1) + F(n-2)
        // until only constants dividable by three are left.
        // This formula can be used because every third fibonacci number is even.
        let temp_value = current_fibonacci_number;
        current_fibonacci_number = 4 * current_fibonacci_number + last_fibonacci_number;
        last_fibonacci_number = temp_value;
    }

    sum
}

#[test]
fn test_sum_even_fibonacci_numbers() {
    assert_eq!(sum_even_fibonacci_numbers(10), 10);
    assert_eq!(sum_even_fibonacci_numbers(100), 44);
    assert_eq!(sum_even_fibonacci_numbers(200), 188);
    assert_eq!(sum_even_fibonacci_numbers(1000), 798);
    assert_eq!(sum_even_fibonacci_numbers(3000), 3382);
}

fn main() {
    let limit = 4_000_000;
    println!(
        "The sum of all even-valued fibonacci terms below {} is: {}",
        limit,
        sum_even_fibonacci_numbers(limit)
    );
}
