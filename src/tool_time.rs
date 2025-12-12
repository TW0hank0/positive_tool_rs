use time;

pub fn month_to_number(month: time::Month) -> u8 {
    match month {
        time::Month::April => 4,
        time::Month::August => 8,
        time::Month::December => 12,
        time::Month::February => 2,
        time::Month::January => 1,
        time::Month::July => 7,
        time::Month::June => 6,
        time::Month::March => 3,
        time::Month::May => 5,
        time::Month::November => 11,
        time::Month::October => 10,
        time::Month::September => 9,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time;

    #[test]
    fn test_month_to_number() {
        assert_eq!(1, month_to_number(time::Month::January));
        assert_eq!(12, month_to_number(time::Month::December));
        assert_eq!(6, month_to_number(time::Month::June));
    }
}
