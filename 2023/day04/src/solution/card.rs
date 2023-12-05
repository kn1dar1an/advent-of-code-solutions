use std::io;

pub struct Card {
    pub instances: usize,
    winning_numbers: Vec<usize>,
    scratched_numbers: Vec<usize>,
}

impl Card {
    pub fn from_str(s: String) -> io::Result<Card> {
        let mut winning_numbers: Vec<usize> = vec![];
        let mut scratched_numbers: Vec<usize> = vec![];

        let (_, game) = s.split_at(s.find(": ").unwrap() + 2);

        let (winning, scratched) = game.split_at(game.find("|").unwrap() + 2);

        winning.split(" ").for_each(|s| {
            if let Ok(parsed) = s.parse::<usize>() {
                winning_numbers.push(parsed)
            }
        });
        scratched.split(" ").for_each(|s| {
            if let Ok(parsed) = s.parse::<usize>() {
                scratched_numbers.push(parsed)
            }
        });

        Ok(Self {
            instances: 0,
            winning_numbers,
            scratched_numbers,
        })
    }

    pub fn count_points(&self) -> usize {
        let count = self.count_matches() as u32;

        if count == 0 {
            0
        } else {
            2usize.pow(count - 1)
        }
    }

    pub fn count_matches(&self) -> usize {
        let mut count = 0usize;
        self.scratched_numbers.iter().for_each(|num| {
            if self.winning_numbers.contains(num) {
                count += 1
            }
        });
        count
    }
}
