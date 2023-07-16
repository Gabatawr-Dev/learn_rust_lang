#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); &rows * &cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        #[rustfmt::skip]
        let offsets = vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1), /*cell*/ ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];

        let mut neighbours = Vec::new();

        for (ox, oy) in offsets {
            let nx = row as isize + ox;
            let ny = col as isize + oy;
            
            if nx >= 0 && ny >= 0 && nx < self.rows as isize && ny < self.cols as isize {
                neighbours.push((nx as usize, ny as usize));
            }
        }
        
        neighbours
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut new_grid = self.grid.clone();
        
        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                let alive_neighbours = self.grid
                    .neighbours(row, col)
                    .iter()
                    .filter(|(r, c)| self.grid.get(*r, *c) == &Cell::Alive)
                    .count();
                
                match (self.grid.get(row, col), alive_neighbours) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => (),
                    (Cell::Dead, 3) => new_grid.set(Cell::Alive, row, col),
                    _ => new_grid.set(Cell::Dead, row, col),
                };
            }
        }
        
        self.grid = new_grid;
    }
}
