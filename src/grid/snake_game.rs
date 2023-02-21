use rand::Rng;
use std::collections::VecDeque;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    Snake,
    Apple,
}

pub struct Grid {
    sz: usize,
    grid: Vec<Vec<Cell>>,
    dead: bool,
    pos: VecDeque<(usize, usize)>,
    dir: (i32, i32),
}

#[derive(Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    pub fn with_size(sz: usize) -> Self {
        let mut grid = vec![vec![Cell::Empty; sz]; sz];
        let start_pos = (sz / 2, 0);
        let mut pos = VecDeque::new();
        pos.push_front(start_pos);
        grid[start_pos.0][start_pos.1] = Cell::Snake;
        Grid::place_apple(&mut grid, sz);
        Grid {
            sz,
            grid,
            dead: false,
            pos,
            dir: (0, 1),
        }
    }

    fn place_apple(grid: &mut Vec<Vec<Cell>>, sz: usize) {
        let mut rng = rand::thread_rng();
        loop {
            let i = rng.gen_range(0..sz);
            let j = rng.gen_range(0..sz);
            if grid[i][j] == Cell::Empty {
                grid[i][j] = Cell::Apple;
                break;
            }
        }
    }

    // returns true if something has changed (i.e. non-dead snake)
    pub fn tick(&mut self) -> bool {
        if self.dead {
            return false;
        }
        let nxt = *self.pos.front().unwrap();
        let nxt = (nxt.0 as i32 + self.dir.0, nxt.1 as i32 + self.dir.1);
        let nxt = (
            (nxt.0 + self.sz as i32) as usize % self.sz,
            (nxt.1 + self.sz as i32) as usize % self.sz,
        );
        if self.grid[nxt.0][nxt.1] == Cell::Apple {
            Grid::place_apple(&mut self.grid, self.sz);
        } else {
            let lst = *self.pos.back().unwrap();
            self.pos.pop_back();
            self.grid[lst.0][lst.1] = Cell::Empty;
            if self.grid[nxt.0][nxt.1] == Cell::Snake {
                self.dead = true;
                return false;
            }
        }
        self.pos.push_front(nxt);
        self.grid[nxt.0][nxt.1] = Cell::Snake;
        true
    }

    pub fn change_dir(&mut self, d: Dir) -> () {
        match d {
            Dir::Up => {
                self.dir = (-1, 0);
            }
            Dir::Down => {
                self.dir = (1, 0);
            }
            Dir::Left => {
                self.dir = (0, -1);
            }
            Dir::Right => {
                self.dir = (0, 1);
            }
        };
    }

    pub fn get_cell(&self, i: usize, j: usize) -> Cell {
        self.grid[i][j].clone()
    }
}
