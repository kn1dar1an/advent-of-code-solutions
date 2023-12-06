use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Range,
};

enum InputPhase {
    Seeds,
    SeedToSoilMap,
    SoilToFertilizerMap,
    FertilizerToWaterMap,
    WaterToLightMap,
    LightToTemperatureMap,
    TemperatureToHumidityMap,
    HumidityToLocationMap,
}

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub sts: AlmanacMap,
    pub stf: AlmanacMap,
    pub ftw: AlmanacMap,
    pub wtl: AlmanacMap,
    pub ltt: AlmanacMap,
    pub tth: AlmanacMap,
    pub htl: AlmanacMap,
}

impl Almanac {
    fn new() -> Self {
        let sts = AlmanacMap::new();
        let stf = AlmanacMap::new();
        let ftw = AlmanacMap::new();
        let wtl = AlmanacMap::new();
        let ltt = AlmanacMap::new();
        let tth = AlmanacMap::new();
        let htl = AlmanacMap::new();
        Self {
            seeds: vec![],
            sts,
            stf,
            ftw,
            wtl,
            ltt,
            tth,
            htl,
        }
    }

    pub fn new_from_buf(reader: BufReader<File>) -> io::Result<Self> {
        let mut almanac = Almanac::new();
        let mut input_phase = InputPhase::Seeds;

        for str_row in reader.lines() {
            if let Ok(row) = str_row {
                if !row.is_empty() {
                    if row.contains("seed-to-soil") {
                        input_phase = InputPhase::SeedToSoilMap;
                    } else if row.contains("soil-to-fertilizer") {
                        input_phase = InputPhase::SoilToFertilizerMap;
                    } else if row.contains("fertilizer-to-water") {
                        input_phase = InputPhase::FertilizerToWaterMap;
                    } else if row.contains("water-to-light") {
                        input_phase = InputPhase::WaterToLightMap;
                    } else if row.contains("light-to-temperature") {
                        input_phase = InputPhase::LightToTemperatureMap;
                    } else if row.contains("temperature-to-humidity") {
                        input_phase = InputPhase::TemperatureToHumidityMap;
                    } else if row.contains("humidity-to-location") {
                        input_phase = InputPhase::HumidityToLocationMap;
                    } else {
                        handle_mapping(&input_phase, row, &mut almanac);
                    }
                }
            }
        }

        Ok(almanac)
    }

    pub fn get_lowest_location(&self) -> u64 {
        let mut lowest = u64::MAX;

        for seed in &self.seeds {
            // Could be done with a vec of mappings instead
            let loc = self.htl.get(
                self.tth.get(
                    self.ltt.get(
                        self.wtl
                            .get(self.ftw.get(self.stf.get(self.sts.get(*seed)))),
                    ),
                ),
            );
            if loc < lowest {
                lowest = loc;
            }
        }

        lowest
    }

    pub fn get_lowest_location_with_seed_range(&self) -> u64 {
        let mut ranges = Vec::<Range<u64>>::new();

        for chunk in self.seeds.chunks(2) {
            ranges.push(chunk[0]..chunk[0] + chunk[1]);
        }

        // Could be done with a vec of mappings instead
        let location_ranges = self.htl.get_ranges(
            self.tth.get_ranges(
                self.ltt.get_ranges(
                    self.wtl.get_ranges(
                        self.ftw
                            .get_ranges(self.stf.get_ranges(self.sts.get_ranges(ranges))),
                    ),
                ),
            ),
        );

        location_ranges
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

fn handle_mapping(input_phase: &InputPhase, row: String, almanac: &mut Almanac) {
    match *input_phase {
        InputPhase::Seeds => {
            let (_, seeds_str) = row.split_at(row.find(" ").unwrap() + 1);
            seeds_str
                .split(" ")
                .for_each(|seed_str| almanac.seeds.push(seed_str.parse::<u64>().unwrap()));
        }
        InputPhase::SeedToSoilMap => {
            almanac.sts.add_mapping(&row);
        }
        InputPhase::SoilToFertilizerMap => {
            almanac.stf.add_mapping(&row);
        }
        InputPhase::FertilizerToWaterMap => {
            almanac.ftw.add_mapping(&row);
        }
        InputPhase::WaterToLightMap => {
            almanac.wtl.add_mapping(&row);
        }
        InputPhase::LightToTemperatureMap => {
            almanac.ltt.add_mapping(&row);
        }
        InputPhase::TemperatureToHumidityMap => {
            almanac.tth.add_mapping(&row);
        }
        InputPhase::HumidityToLocationMap => {
            almanac.htl.add_mapping(&row);
        }
    }
}

type DstStart = u64;
type SrcStart = u64;
type Length = u64;
type SrcRange = Range<u64>;
type DstRange = Range<u64>;
pub struct AlmanacMap {
    maps: Vec<(SrcRange, DstStart)>,
    range_maps: Vec<(SrcRange, DstRange)>,
}
impl AlmanacMap {
    pub fn new() -> Self {
        Self {
            maps: Vec::new(),
            range_maps: Vec::new(),
        }
    }

    pub fn add_mapping(&mut self, str: &str) {
        let map_fields = str.split(" ").collect::<Vec<_>>();

        let dst_start = map_fields[0].parse::<DstStart>().unwrap();
        let src_start = map_fields[1].parse::<SrcStart>().unwrap();
        let length = map_fields[2].parse::<Length>().unwrap();

        let src_range = src_start..src_start + length - 1;
        let dst_range = dst_start..dst_start + length - 1;
        self.maps.push((src_range.clone(), dst_start));
        self.range_maps.push((src_range, dst_range));
    }

    pub fn get(&self, source: u64) -> u64 {
        for (src_range, dst_start) in &self.maps {
            if src_range.contains(&source) {
                let diff = source - src_range.start;
                return dst_start + diff;
            }
        }

        return source;
    }

    pub fn get_ranges(&self, input_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut out_ranges = vec![];

        for (mapped_src_range, mapped_dst_range) in &self.range_maps {
            let mut intersections = vec![];
            for input_range in &input_ranges {
                if let Some(intersection) = intersect(mapped_src_range, input_range) {
                    let out_dst_start = 
                        mapped_dst_range.start + intersection.start - mapped_src_range.start;

                    let out_dst_end =
                        mapped_dst_range.start + intersection.end - mapped_src_range.start;

                    out_ranges.push(out_dst_start..out_dst_end);
                    intersections.push(intersection);
                }
            }
            out_ranges.extend(intersection_differences(mapped_src_range, intersections));
        }


        out_ranges
    }
}

fn intersect(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = u64::max(a.start, b.start);
    let end = u64::min(a.end, b.end);

    if start <= end {
        Some(start..end)
    } else {
        None
    }
}

fn intersection_differences(range: &Range<u64>, mut intersections: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut out_ranges = Vec::new();

    intersections.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    let mut range_cursor = range.start;

    for intersection in &intersections {
        if intersection.start > range_cursor {
            out_ranges.push(range_cursor..intersection.start - 1);
        }

        if intersection.end > range_cursor {
            range_cursor = intersection.end;
        }
    }

    // Check for final tail
    if range.end > range_cursor {
        out_ranges.push(range_cursor..range.end);
    }

    out_ranges
}
