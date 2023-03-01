use super::direction::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct Movement {
    pub amount: i32,
    pub direction: Direction,
}

impl Movement {
    pub fn from_string(s: String) -> Self {
        let split_str: Vec<&str> = s.split_whitespace().collect::<Vec<_>>();
        let amount_str: &str = split_str.get(1).unwrap();
        let direction_str: &str = split_str.get(0).unwrap();

        let parsed_abs_amount: i32 = amount_str.parse::<i32>().expect("Couldn't parse amount");

        let (amount, direction): (i32, Direction) = if direction_str == "R" || direction_str == "L"
        {
            (
                if direction_str == "R" {
                    parsed_abs_amount
                } else {
                    -parsed_abs_amount
                },
                Direction::Horizontal,
            )
        } else if direction_str == "U" || direction_str == "D" {
            (
                if direction_str == "U" {
                    parsed_abs_amount
                } else {
                    -parsed_abs_amount
                },
                Direction::Vertical,
            )
        } else {
            panic!("Couldn't parse direction");
        };

        Movement {
            amount: amount,
            direction: direction,
        }
    }
}
