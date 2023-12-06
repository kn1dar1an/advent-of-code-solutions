use crate::configuration::Configuration;
use std::io;

pub fn run(_config: Configuration) -> io::Result<String> {
    let _test_races = vec![
        RaceData { ms: 7, mm: 9 },
        RaceData { ms: 15, mm: 40 },
        RaceData { ms: 30, mm: 200 },
    ];

    let part_1_races = vec![
        RaceData { ms: 46, mm: 208 },
        RaceData { ms: 85, mm: 1412 },
        RaceData { ms: 75, mm: 1257 },
        RaceData { ms: 82, mm: 1410 },
    ];

    let part_2_race = RaceData {
        ms: 46857582,
        mm: 208141212571410,
    };

    let mut margin_of_error = 1usize;
    for race in part_1_races.iter() {
        let count = get_count(race);
        margin_of_error *= count;
    }

    let part2_count = get_count(&part_2_race);

    Ok(format!(
        "part1: {}, part2: {}",
        margin_of_error, part2_count
    ))
}

struct RaceData {
    ms: usize,
    mm: usize,
}

fn get_count(race: &RaceData) -> usize {
    let mut button_time = 0usize;
    let count = loop {
        let distance = button_time * (race.ms - button_time);
        if distance > race.mm {
            break race.ms + 1 - (button_time * 2); // + 1 to include 0ms / 7ms button time
        } else {
            button_time += 1;
        }
    };

    count
}
