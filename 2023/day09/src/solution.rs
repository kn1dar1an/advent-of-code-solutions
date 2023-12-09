use crate::configuration::Configuration;
use std::io::{self, BufRead, ErrorKind};

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut part_1_sum = 0isize;
        let mut part_2_sum = 0isize;
        for s in input_buf.lines().flatten() {
            let readings: Vec<isize> = s
                .split(' ')
                .map(|item| item.parse::<isize>().unwrap())
                .collect();
            part_1_sum += extrapolate(&readings, ExtrDir::Forwards);
            part_2_sum += extrapolate(&readings, ExtrDir::Backwards);
        }

        Ok(format!("part1: {}, part2: {}", part_1_sum, part_2_sum))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

fn generate_diff_history(history: &Vec<isize>) -> Vec<isize> {
    let mut history_diff: Vec<isize> = Vec::new();

    for i in 0..=history.len() - 2 {
        let diff = history[i + 1] - history[i];
        history_diff.push(diff);
    }

    history_diff
}

enum ExtrDir {
    Forwards,
    Backwards,
}

fn extrapolate(history: &Vec<isize>, direction: ExtrDir) -> isize {
    let mut diff_tree: Vec<Vec<isize>> = vec![];

    let mut diffs = generate_diff_history(history);
    diff_tree.push(diffs.clone());
    loop {
        if !all_items_zero(&diffs) {
            diffs = generate_diff_history(&diffs);
            diff_tree.push(diffs.clone());
        } else {
            break;
        }
    }

    let mut result = 0isize;
    for diff_level in diff_tree.iter().rev() {
        match direction {
            ExtrDir::Forwards => {
                if let Some(last) = diff_level.last() {
                    result += last;
                }
            }
            ExtrDir::Backwards => {
                if let Some(first) = diff_level.first() {
                    result = first - result
                }
            }
        }
    }
    match direction {
        ExtrDir::Forwards => history.last().unwrap() + result,
        ExtrDir::Backwards => history.first().unwrap() - result,
    }
}

fn all_items_zero(items: &Vec<isize>) -> bool {
    for item in items {
        if item != &0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::solution::extrapolate;

    #[test]
    fn history_1_fwd() {
        let values = vec![0isize, 3isize, 6isize, 9isize, 12isize, 15isize];

        assert_eq!(
            extrapolate(&values, crate::solution::ExtrDir::Forwards),
            18isize
        );
    }

    #[test]
    fn history_2_fwd() {
        let values = vec![1isize, 3isize, 6isize, 10isize, 15isize, 21isize];

        assert_eq!(
            extrapolate(&values, crate::solution::ExtrDir::Forwards),
            28isize
        );
    }

    #[test]
    fn history_3_fwd() {
        let values = vec![10isize, 13isize, 16isize, 21isize, 30isize, 45isize];

        assert_eq!(
            extrapolate(&values, crate::solution::ExtrDir::Forwards),
            68isize
        );
    }

    #[test]
    fn history_3_bwd() {
        let values = vec![10isize, 13isize, 16isize, 21isize, 30isize, 45isize];

        assert_eq!(
            extrapolate(&values, crate::solution::ExtrDir::Backwards),
            5isize
        );
    }
}
