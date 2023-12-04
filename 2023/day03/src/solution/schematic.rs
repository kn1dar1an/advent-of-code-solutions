use std::{
    cell::RefCell,
    char,
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{self, BufRead, BufReader},
    rc::Rc,
    usize, vec,
};

use super::NUMBERS;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cell<T> {
    pub coord: Coord,
    pub value: T,
    pub neighbours: Vec<Rc<RefCell<Cell<T>>>>,
}

impl<T> Hash for Cell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}

impl<T> Cell<T> {
    pub fn new(coord: Coord, value: T) -> Self {
        Self {
            coord,
            value,
            neighbours: vec![],
        }
    }
}

#[derive(Debug)]
pub struct SimpleMatrix<T> {
    pub cells: Vec<Rc<Cell<T>>>,
    pub adjacency_map: HashMap<Coord, Vec<Rc<Cell<T>>>>,
}

impl<T> SimpleMatrix<T> {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            adjacency_map: HashMap::new(),
        }
    }

    pub fn add_adjacent(&mut self, rc_cell: Rc<Cell<T>>) {
        let coord = &rc_cell.coord;
        let y_neg_bound = if coord.y == 0 { 0 } else { coord.y - 1 };
        let x_neg_bound = if coord.x == 0 { 0 } else { coord.x - 1 };

        for y in y_neg_bound..=coord.y + 1 {
            for x in x_neg_bound..=coord.x + 1 {
                let adj_coord = Coord { x, y };
                match self.adjacency_map.get_mut(&adj_coord) {
                    Some(adjacents) => adjacents.push(rc_cell.clone()),
                    None => {
                        let _ = self.adjacency_map.insert(adj_coord, vec![rc_cell.clone()]);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Component {
    cells: Vec<Rc<Cell<char>>>,
}

impl Component {
    pub fn get_number(&self) -> usize {
        let mut num_buf = String::new();
        for cell_rc in &self.cells {
            num_buf.push(cell_rc.value);
        }

        num_buf.parse::<usize>().unwrap()
    }
}

pub struct Schematic {
    _matrix: SimpleMatrix<char>,
    _components: Vec<Rc<Component>>,
    symbol_adjacency_map: HashMap<Rc<Cell<char>>, HashSet<Rc<Component>>>,
}

impl Schematic {
    pub fn new(reader: BufReader<File>) -> io::Result<Self> {
        let mut matrix = SimpleMatrix::<char>::new();
        let mut components: Vec<Rc<Component>> = vec![];
        let mut symbol_adjacency_map: HashMap<Rc<Cell<char>>, HashSet<Rc<Component>>> =
            HashMap::new();

        let mut component_cell_buf: Vec<Rc<Cell<char>>> = vec![];
        for (y, str_row) in reader.lines().enumerate() {
            if let Ok(s) = str_row {
                for (x, c) in s.chars().enumerate() {
                    let coord = Coord { x, y };
                    let rc_cell = Rc::new(Cell::new(coord, c));

                    // Create component
                    if NUMBERS.contains(&c) {
                        component_cell_buf.push(rc_cell.clone());
                    } else if !component_cell_buf.is_empty() {
                        let mut component = Component { cells: vec![] };
                        component_cell_buf
                            .iter()
                            .for_each(|rc_cell| component.cells.push(rc_cell.clone()));
                        components.push(Rc::new(component));
                        component_cell_buf.clear();
                    }

                    matrix.add_adjacent(rc_cell.clone());

                    // Keep owned rc_cells in matrix
                    matrix.cells.push(rc_cell);
                }
            }
        }

        // Set adjacent symbols
        for component_ref in &components {
            for rc_cell_ref in &component_ref.cells {
                if let Some(adjacents) = matrix.adjacency_map.get(&rc_cell_ref.coord) {
                    for adjacent_rc_cell_ref in adjacents {
                        if adjacent_rc_cell_ref.value != '.'
                            && !NUMBERS.contains(&adjacent_rc_cell_ref.value)
                        {
                            match symbol_adjacency_map.get_mut(adjacent_rc_cell_ref) {
                                Some(adjacent_components) => {
                                    adjacent_components.insert(component_ref.clone());
                                }
                                None => {
                                    let mut new_set = HashSet::new();
                                    new_set.insert(component_ref.clone());
                                    let _ = symbol_adjacency_map
                                        .insert(adjacent_rc_cell_ref.clone(), new_set);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            _matrix: matrix,
            _components: components,
            symbol_adjacency_map,
        })
    }

    pub fn find_component_sum(&self) -> usize {
        let mut sum = 0usize;

        for (_rc_cell, adjacent_component_refs) in &self.symbol_adjacency_map {
            for component in adjacent_component_refs {
                sum += component.get_number();
            }
        }

        sum
    }

    pub fn find_gear_ratio_sum(&self) -> usize {
        let mut sum = 0usize;

        for (rc_cell, adjacent_component_refs) in &self.symbol_adjacency_map {
            if rc_cell.value == '*' && adjacent_component_refs.len() > 1 {
                let mut ratio = 1usize;
                for component in adjacent_component_refs {
                    ratio *= component.get_number();
                }
                sum += ratio;
            }
        }

        sum
    }
}