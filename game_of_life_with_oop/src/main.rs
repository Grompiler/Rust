use std::collections::HashMap;
use std::{thread, time};

trait CellMethods {
    fn new(x_pos: usize, y_pos: usize) -> Self;
    fn is_alive(&self) -> bool;
    fn give_life(&mut self);
    fn kill(&mut self);
    fn get_coord(&self) -> (usize, usize);
    fn apply_new_state(&mut self);
    fn update_next_state(&mut self, b: bool);
}

trait GridMethods {
    fn new(x_size: usize, y_size: usize) -> Self;
    fn fill_cells(&mut self);
    fn init_indexes(&mut self);
    fn init_life(&mut self, coord_to_set_alive: Vec<(usize, usize)>);
    fn show(&self);
    fn get_size(&self) -> (usize, usize);
    fn next_step(&mut self);
}

#[derive(Debug)]
struct Cell {
    current_state: bool,
    next_state: bool,
    coord: (usize, usize),
}
struct Grid {
    size: (usize, usize),
    cells: Vec<Cell>,
    idxs_cell_and_neighbours: Vec<(usize, HashMap<String, usize>)>,
}

impl CellMethods for Cell {
    fn new(x_pos: usize, y_pos: usize) -> Self {
        Cell {
            current_state: false,
            next_state: false,
            coord: (x_pos, y_pos),
        }
    }
    fn is_alive(&self) -> bool {
        self.current_state
    }
    fn give_life(&mut self) {
        self.current_state = true;
    }
    fn get_coord(&self) -> (usize, usize) {
        (self.coord.0, self.coord.1)
    }
    fn kill(&mut self) {
        self.current_state = false;
    }
    fn apply_new_state(&mut self) {
        self.current_state = self.next_state;
        self.next_state = false;
    }
    fn update_next_state(&mut self, b: bool) {
        self.next_state = b;
    }
}

impl GridMethods for Grid {
    fn init_indexes(&mut self) {
        for (i, _) in self.cells.iter().enumerate() {
            // dbg!(i);
            // println!("{:?}, {:?}", i, e);
            let x_size = self.get_size().0;
            let mut neighbours: HashMap<String, usize> = HashMap::new();

            let border = i % self.get_size().0;

            match border {
                0 => {
                    if let Some(_) = self.cells.get(i + 1) {
                        neighbours.insert(String::from("east"), i + 1);
                    }
                    if let Some(_) = self.cells.get(i + (x_size + 1)) {
                        neighbours.insert(String::from("s_e"), i + (x_size + 1));
                    }
                    if let Some(_) = self.cells.get(i + x_size) {
                        neighbours.insert(String::from("south"), i + x_size);
                    }

                    if i >= x_size {
                        if let Some(_) = self.cells.get(i - (x_size - 1)) {
                            neighbours.insert(String::from("n_e"), i - (x_size - 1));
                        }
                        if let Some(_) = self.cells.get(i - x_size) {
                            neighbours.insert(String::from("north"), i - x_size);
                        }
                    }
                }
                matched_value @ _ if matched_value < x_size - 1 => {
                    if let Some(_) = self.cells.get(i + 1) {
                        neighbours.insert(String::from("east"), i + 1);
                    }
                    if let Some(_) = self.cells.get(i + (x_size + 1)) {
                        neighbours.insert(String::from("s_e"), i + (x_size + 1));
                    }
                    if let Some(_) = self.cells.get(i + x_size) {
                        neighbours.insert(String::from("south"), i + x_size);
                    }
                    if let Some(_) = self.cells.get(i + (x_size - 1)) {
                        neighbours.insert(String::from("s_w"), i + (x_size - 1));
                    }
                    if let Some(_) = self.cells.get(i - 1) {
                        neighbours.insert(String::from("west"), i - 1);
                    }

                    if i >= x_size {
                        if let Some(_) = self.cells.get(i - (x_size + 1)) {
                            neighbours.insert(String::from("n_w"), i - (x_size + 1));
                        }
                        if let Some(_) = self.cells.get(i - x_size) {
                            neighbours.insert(String::from("north"), i - x_size);
                        }
                        if let Some(_) = self.cells.get(i - (x_size - 1)) {
                            neighbours.insert(String::from("n_e"), i - (x_size - 1));
                        }
                    }
                }
                matched_value @ _ if matched_value < x_size => {
                    if let Some(_) = self.cells.get(i - 1) {
                        neighbours.insert(String::from("west"), i - 1);
                    }
                    if let Some(_) = self.cells.get(i + x_size) {
                        neighbours.insert(String::from("south"), i + x_size);
                    }
                    if let Some(_) = self.cells.get(i + (x_size - 1)) {
                        neighbours.insert(String::from("s_w"), i + (x_size - 1));
                    }
                    if i >= x_size {
                        if let Some(_) = self.cells.get(i - (x_size + 1)) {
                            neighbours.insert(String::from("n_w"), i - (x_size + 1));
                        }
                        if let Some(_) = self.cells.get(i - x_size) {
                            neighbours.insert(String::from("north"), i - x_size);
                        }
                    }
                }
                _ => panic!("This should not happen, all cases are handled up here"),
            }

            self.idxs_cell_and_neighbours.push((i, neighbours));
        }
    }

    fn get_size(&self) -> (usize, usize) {
        (self.size.0, self.size.1)
    }
    fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            size: (x_size, y_size),
            cells: Vec::new(),
            idxs_cell_and_neighbours: Vec::new(),
        }
    }

    fn fill_cells(&mut self) {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                self.cells.push(Cell::new(i, j));
            }
        }
    }

    fn init_life(&mut self, coord_to_set_alive: Vec<(usize, usize)>) {
        for e in coord_to_set_alive {
            match self.cells.iter_mut().find(|x| x.get_coord() == e) {
                Some(x) => x.give_life(),
                None => (),
            }
        }
    }

    fn show(&self) {
        let x_size = self.size.0 - 1;
        let mut actual_x_pos = 0;
        for e in &self.cells {
            match e.is_alive() {
                true => print!(" @"),
                false => print!(" -"),
            }
            if actual_x_pos == x_size {
                actual_x_pos = 0;
                println!("");
            } else {
                actual_x_pos += 1;
            }
        }
        thread::sleep(time::Duration::from_millis(100));
        print!("\x1B[2J");
    }

    fn next_step(&mut self) {
        for (a, b) in &self.idxs_cell_and_neighbours {
            let mut inc = 0;
            for e in b.values() {
                if self.cells[*e as usize].is_alive() {
                    inc += 1;
                }
                if inc == 3 || inc == 2 && self.cells[*a as usize].is_alive() {
                    self.cells[*a as usize].update_next_state(true);
                } else {
                    self.cells[*a as usize].update_next_state(false);
                }
            }
        }
        for e in self.cells.iter_mut() {
            e.apply_new_state();
        }
    }
}

fn main() {
    let x = 36;
    let y = 36;
    // let idty_5 = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)];
    // let line = vec![(1, 0), (1, 1), (1, 2)];
    // let fun = vec![
    // (4, 15),
    // (5, 15),
    // (6, 14),
    // (6, 16),
    // (7, 15),
    // (8, 15),
    // (9, 15),
    // (10, 15),
    // (11, 14),
    // (11, 16),
    // (12, 15),
    // (13, 15),
    // (14, 15),
    // ];

    // grid size for fun_2: 29x29
    // let fun_2 = vec![
    // (9, 13),
    // (9, 15),
    // (10, 11),
    // (10, 12),
    // (10, 13),
    // (10, 15),
    // (10, 16),
    // (10, 17),
    // (11, 10),
    // (11, 14),
    // (11, 18),
    // (12, 10),
    // (12, 12),
    // (12, 16),
    // (12, 18),
    // (13, 11),
    // (13, 12),
    // (13, 14),
    // (13, 16),
    // (13, 17),
    // (15, 11),
    // (15, 12),
    // (15, 14),
    // (15, 16),
    // (15, 17),
    // (16, 10),
    // (16, 12),
    // (16, 16),
    // (16, 18),
    // (17, 10),
    // (17, 14),
    // (17, 18),
    // (18, 11),
    // (18, 12),
    // (18, 13),
    // (18, 15),
    // (18, 16),
    // (18, 17),
    // (19, 13),
    // (19, 15),
    // ];
    let fire_gun = vec![
        (8, 16),
        (8, 17),
        (9, 16),
        (9, 17),
        (10, 16),
        (10, 17),
        (8, 0),
        (8, 1),
        (9, 0),
        (9, 1),
        (7, 11),
        (7, 12),
        (6, 12),
        (8, 10),
        (8, 11),
        (9, 9),
        (9, 10),
        (9, 11),
        (10, 10),
        (10, 11),
        (11, 11),
        (11, 12),
        (12, 12),
        (7, 19),
        (7, 22),
        (6, 20),
        (6, 22),
        (5, 21),
        (5, 23),
        (4, 23),
        (8, 20),
        (8, 22),
        (9, 21),
        (9, 23),
        (10, 23),
        (6, 34),
        (6, 35),
        (7, 34),
        (7, 35),
    ];
    let mut grid = Grid::new(x, y);
    grid.fill_cells();
    grid.init_life(fire_gun);
    grid.init_indexes();
    // for e in &grid.idxs_cell_and_neighbours {
    // println!("{:?}", e);
    // }

    loop {
        grid.show();
        grid.next_step();
    }
}
