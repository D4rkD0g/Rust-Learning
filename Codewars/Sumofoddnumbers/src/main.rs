fn row_sum_odd_numbers(n:i64) -> i64 {
  // your solution here
  n * n * n
}

#[test]
fn returns_expected() {
  assert_eq!(row_sum_odd_numbers(1), 1);
  assert_eq!(row_sum_odd_numbers(42), 74088);
}