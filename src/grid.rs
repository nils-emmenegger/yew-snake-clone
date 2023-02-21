mod snake_game;

use gloo_timers::callback::Interval;
use yew::prelude::*;

pub struct Grid {
    grid: snake_game::Grid,
    _interval: Interval,
}

#[derive(Clone, Copy)]
pub enum Msg {
    Tick,
    Dir(snake_game::Dir),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub grid_size: usize,
}

impl Component for Grid {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| Msg::Tick);
        let _interval = Interval::new(200, move || callback.emit(()));
        Grid {
            grid: snake_game::Grid::with_size(ctx.props().grid_size),
            _interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Tick => self.grid.tick(),
            Msg::Dir(d) => {
                self.grid.change_dir(d);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sz = ctx.props().grid_size;
        let mut items = Vec::with_capacity(sz * (sz + 1));
        for i in 0..sz {
            for j in 0..sz {
                items.push(html! {
                <div class={classes!("grid-block",
                                    match self.grid.get_cell(i, j) {
                                        snake_game::Cell::Empty => "grid-empty",
                                        snake_game::Cell::Snake => "grid-snake",
                                        snake_game::Cell::Apple => "grid-apple",
                                    })}></div> });
            }
            items.push(html! { <br/> });
        }
        let mc = |m| ctx.link().callback(move |_| m);
        html! {
            <>
            { for items }
            <button onclick={mc(Msg::Dir(snake_game::Dir::Down))}>{ "Down" }</button>
            <button onclick={mc(Msg::Dir(snake_game::Dir::Up))}>{ "Up" }</button>
            <br/>
            <button onclick={mc(Msg::Dir(snake_game::Dir::Left))}>{ "Left" }</button>
            <button onclick={mc(Msg::Dir(snake_game::Dir::Right))}>{ "Right" }</button>
            </>
        }
    }
}
