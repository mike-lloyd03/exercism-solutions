use clock::Clock;

fn main() {
    println!("23:38 | {}", Clock::new(0, -22));
    let c = Clock::new(12, 30);
    println!("{}", c);
    println!("{}", c.add_minutes(15));
}

// fn test_compare_clocks_with_minutes_overflow() {
//     assert_eq!(Clock::new(0, 1), Clock::new(0, 1441));
// }
// fn test_compare_clocks_with_minutes_overflow_by_several_days() {
//     assert_eq!(Clock::new(2, 2), Clock::new(2, 4322));
// }
// fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
//     assert_eq!(Clock::new(18, 7), Clock::new(-54, -11_513))
// }
// fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
//     assert_eq!(Clock::new(6, 15), Clock::new(6, -4305))
// }
// fn test_hours_and_minutes_roll_over_continuously() {
//     assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
// }
// fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
//     assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
// }
// fn test_minutes_roll_over_continuously() {
//     assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
// }
// fn test_negative_hour_and_minutes_both_roll_over_continuously() {
//     assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
// }
// fn test_negative_minutes_roll_over_continuously() {
//     assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
// }
// fn test_zero_hour_and_negative_minutes() {
//     assert_eq!(Clock::new(0, -22).to_string(), "23:38");
// }
