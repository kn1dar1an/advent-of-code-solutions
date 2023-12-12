use std::{io::{BufRead, self, ErrorKind}, collections::{HashMap, HashSet}, vec, ops::RangeInclusive};
use crate::configuration::Configuration;

pub fn run(config: Configuration) -> io::Result<String> {
    if let Some(input_buf) = config.input_file_buffer {
        let mut start_coord = None;
        let mut tile_map: HashMap<Coord, Tile> = HashMap::new();
        for (y, s) in input_buf.lines().flatten().enumerate() {
            for (x, c) in s.as_bytes().iter().enumerate() {
                let coord = Coord { y: y as isize, x: x as isize };
                let tile = Tile { coord, tile: *c as char, distance_from_s: 0 };
                if *c as char == 'S' { start_coord = Some(coord); }
                tile_map.entry(coord).or_insert(tile);
            }
        }
        let mut loop_tiles = HashSet::new();
        if let Some(start_tile) = tile_map.get(&start_coord.unwrap()) {
            loop_tiles.insert(*start_tile);
        }
        recurse_tile_distances(0, vec![], vec![start_coord.unwrap()], &mut tile_map, &mut loop_tiles);

        let max_distance_tile = loop_tiles.iter().max().unwrap();

        let inner_tiles_count = count_inner_tiles_with_horizontal_rays(&tile_map, &loop_tiles);

        Ok(format!("part1: {}, part2: {}", max_distance_tile.distance_from_s, inner_tiles_count))
    } else {
        Err(io::Error::new(ErrorKind::Other, "Input file required"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    y: isize,
    x: isize,
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Hash)]
struct Tile {
    coord: Coord,
    tile: char,
    distance_from_s: usize,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_from_s.cmp(&other.distance_from_s)
    }
}

fn recurse_tile_distances(hop: usize, prev_coords: Vec<Coord>, coords: Vec<Coord>, tile_map: &mut HashMap<Coord, Tile>, loop_tiles: &mut HashSet<Tile>) {
    let mut next_coords = vec![];
    for coord in &coords {
        let tile = tile_map.get(&coord).unwrap();
        let next: Vec<Coord> = get_next_available_coords(&tile, &tile_map);
        
        if let Some(tile) = tile_map.get_mut(&coord) {
            if tile.distance_from_s == 0 { 
                tile.distance_from_s = hop; 
                next.iter().for_each(|next| if !prev_coords.contains(next) { next_coords.push(*next) });
                loop_tiles.insert(tile.clone());
            }
        }
    }
    if next_coords.is_empty() {
        return;
    }
    recurse_tile_distances(hop + 1, coords, next_coords, tile_map, loop_tiles);
}

fn count_inner_tiles_with_horizontal_rays(tile_map: &HashMap<Coord, Tile>, loop_tiles: &HashSet<Tile>) -> usize {
    let mut count = 0usize;
    let (y_range, x_range) = get_loop_coord_ranges(loop_tiles);

    // Loop horizontal ranges accross pipe loop
    for y in y_range {
        let mut vertical_pipe_count = 0usize;
        let mut last_angle_tile = ' ';
        for x in x_range.clone() {
            let current_coord = Coord { y, x };
            if let Some(tile) = tile_map.get(&current_coord) {
                if loop_tiles.contains(tile) {
                    match tile.tile {
                        '|' => {
                            vertical_pipe_count += 1;
                            last_angle_tile = ' ';
                        },
                        'L' | 'J' => {
                            if [' ', 'L', 'J'].contains(&last_angle_tile) { 
                                vertical_pipe_count += 1;
                                last_angle_tile = tile.tile;
                            }
                        }
                        '7' | 'F' => {
                            if [' ', '7', 'F'].contains(&last_angle_tile) { 
                                vertical_pipe_count += 1;
                                last_angle_tile = tile.tile;
                            }
                        },
                        _ => {},
                    }
                } else if vertical_pipe_count % 2 != 0 {
                    count += 1;
                }
            } 
        }
    }


    count
}

fn get_loop_coord_ranges(loop_tiles: &HashSet<Tile>) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let min_y = loop_tiles.iter().min_by(|a, b| a.coord.y.cmp(&b.coord.y)).unwrap().coord.y;
    let max_y = loop_tiles.iter().max_by(|a, b| a.coord.y.cmp(&b.coord.y)).unwrap().coord.y;
    let min_x = loop_tiles.iter().min_by(|a, b| a.coord.x.cmp(&b.coord.x)).unwrap().coord.x;
    let max_x = loop_tiles.iter().max_by(|a, b| a.coord.x.cmp(&b.coord.x)).unwrap().coord.x;


    (min_y..=max_y, min_x..=max_x)
}

fn get_next_available_coords(tile: &Tile, tile_map: &HashMap<Coord, Tile>) -> Vec<Coord> {
    let mut coords = vec![];

    let available_headings = get_headings(tile);

    for heading in available_headings {
        let next_coord = get_heading_coord(tile.coord, &heading);
        if let Some(next_tile) = tile_map.get(&next_coord) {
            if get_connections(tile, heading).contains(&next_tile.tile) {
                coords.push(next_coord);
            }
        }
    }

    coords
}

enum Heading {
    N, S, E, W
}

fn get_heading_coord(coord: Coord, heading: &Heading) -> Coord {
    match heading {
        Heading::N => Coord { y:coord.y - 1, x:coord.x },
        Heading::S => Coord { y: coord.y + 1, x: coord.x },
        Heading::E => Coord { y: coord.y, x: coord.x + 1 },
        Heading::W => Coord { y: coord.y, x: coord.x - 1 },
    }
}

fn get_headings(tile: &Tile) -> Vec<Heading> {
    match tile.tile {
        'S' => vec![Heading::N, Heading::S, Heading::E, Heading::W],
        '|' => vec![Heading::N, Heading::S],
        '-' => vec![Heading::E, Heading::W],
        'F' => vec![Heading::S, Heading::E],
        'L' => vec![Heading::N, Heading::E],
        '7' => vec![Heading::S, Heading::W],
        'J' => vec![Heading::N, Heading::W],
        _ => vec![],
    }
}

fn get_connections(tile: &Tile, heading: Heading) -> Vec<char> {
    match heading {
        Heading::N => {
            match tile.tile {
                'S' | '|' | 'L' | 'J' => vec!['|', 'F', '7'],
                _ => vec![]
            }
        },
        Heading::S => {
            match tile.tile {
                'S' | '|' | 'F' | '7' => vec!['|', 'L', 'J'],
                _ => vec![]
            }
        },
        Heading::E => {
            match tile.tile {
                'S' | '-' | 'F' | 'L' => vec!['-', 'J', '7'],
                _ => vec![]
            }
        },
        Heading::W => {
            match tile.tile {
                'S' | '-' | 'J' | '7' => vec!['-', 'F', 'L'],
                _ => vec![]
            }
        },
    }
}
