use gloo_timers::callback::Interval;
use rand::Rng;
use std::collections::VecDeque;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Cell {
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
    _interval: Interval,
}

#[derive(Clone, Copy)]
pub enum Msg {
    Tick,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub grid_size: usize,
}

impl Grid {
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
}

impl Component for Grid {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let sz = ctx.props().grid_size;
        let mut grid = vec![vec![Cell::Empty; sz]; sz];
        let start_pos = (sz / 2, 0);
        let mut pos = VecDeque::new();
        pos.push_front(start_pos);
        grid[start_pos.0][start_pos.1] = Cell::Snake;
        Grid::place_apple(&mut grid, sz);
        let callback = ctx.link().callback(|_| Msg::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));
        Grid {
            sz,
            grid,
            dead: false,
            pos,
            dir: (0, 1),
            _interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Tick => {
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
            Msg::Up => {
                if self.dir != (1, 0) {
                    self.dir = (-1, 0);
                }
                false
            }
            Msg::Down => {
                if self.dir != (-1, 0) {
                    self.dir = (1, 0);
                }
                false
            }
            Msg::Left => {
                if self.dir != (0, 1) {
                    self.dir = (0, -1);
                }
                false
            }
            Msg::Right => {
                if self.dir != (0, -1) {
                    self.dir = (0, 1);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut items = Vec::with_capacity(self.sz * (self.sz + 1));
        for i in 0..self.sz {
            for j in 0..self.sz {
                items.push(html! {
                <div class={classes!("grid-block",
                                    match self.grid[i][j] {
                                        Cell::Empty => "grid-empty",
                                        Cell::Snake => "grid-snake",
                                        Cell::Apple => "grid-apple",
                                    })}></div> });
            }
            items.push(html! { <br/> });
        }
        let mc = |m| ctx.link().callback(move |_| m);
        html! {
            <>
            { for items }
            <button onclick={mc(Msg::Down)}>{ "Down" }</button>
            <button onclick={mc(Msg::Up)}>{ "Up" }</button>
            <br/>
            <button onclick={mc(Msg::Left)}>{ "Left" }</button>
            <button onclick={mc(Msg::Right)}>{ "Right" }</button>
            </>
        }
    }
}
