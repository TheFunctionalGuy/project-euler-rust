// Naive way to calculate sum of multiples
#[allow(dead_code)]
fn sum_of_multiples(number: u64, limit: u64) -> u64 {
    let mut sum = 0;
    let mut current_multiple = number;

    while current_multiple < limit {
        sum += current_multiple;
        current_multiple += number;
    }

    sum
}

#[test]
fn test_sum_of_multiples() {
    assert_eq!(sum_of_multiples(3, 10), 18);
    assert_eq!(sum_of_multiples(5, 10), 5);
    assert_eq!(sum_of_multiples(3, 10) + sum_of_multiples(5, 10), 23);
}

// The gaussian formula to calculate the sum of numbers from 0 to n
fn gauss(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[test]
fn test_gauss() {
    assert_eq!(gauss(100), 5050);
    assert_eq!(gauss((10 - 1) / 3) * 3, 18);
    assert_eq!(gauss((10 - 1) / 5) * 5, 5);
}

// The faster way to calculate the sum of multiples
fn sum_of_multiples_two_numbers(a: u64, b: u64, limit: u64) -> u64 {
    // Calculate multiples of a and b below given limit
    let multiples_of_a = gauss((limit - 1) / a) * a;
    let multiples_of_b = gauss((limit - 1) / b) * b;

    // Now calculate numbers that have been summed twice
    let multiples_of_a_times_b = gauss((limit - 1) / (a * b)) * (a * b);

    multiples_of_a + multiples_of_b - multiples_of_a_times_b
}

#[test]
fn test_sum_of_multiples_two_numbers() {
    assert_eq!(sum_of_multiples_two_numbers(3, 5, 10), 23);
    assert_eq!(sum_of_multiples_two_numbers(3, 5, 100), 2318);
    assert_eq!(sum_of_multiples_two_numbers(1, 10, 101), 5050);
}

fn main() {
    println!(
        "Sum of all the multiples of 3 or 5 below 1000 is: {}",
        sum_of_multiples_two_numbers(3, 5, 1000)
    );
}
