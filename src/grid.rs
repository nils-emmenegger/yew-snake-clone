mod snake_game;

use gloo::events::EventListener;
use gloo_timers::callback::Interval;
use snake_game::Dir::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

pub struct Grid {
    grid: snake_game::Grid,
    _interval: Interval,
    kbd_listener: Option<EventListener>,
}

#[derive(Clone, Copy)]
pub enum Msg {
    Tick,
    Dir(snake_game::Dir),
    DoNothing,
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
            kbd_listener: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Tick => self.grid.tick(),
            Msg::Dir(d) => {
                self.grid.change_dir(d);
                false
            }
            Msg::DoNothing => false,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let document = gloo::utils::document();
            let kbd_callback =
                ctx.link()
                    .callback(move |k: KeyboardEvent| match k.key().as_str() {
                        "w" => Msg::Dir(Up),
                        "a" => Msg::Dir(Left),
                        "s" => Msg::Dir(Down),
                        "d" => Msg::Dir(Right),
                        "ArrowUp" => Msg::Dir(Up),
                        "ArrowLeft" => Msg::Dir(Left),
                        "ArrowDown" => Msg::Dir(Down),
                        "ArrowRight" => Msg::Dir(Right),
                        _ => Msg::DoNothing,
                    });
            let listener = EventListener::new(&document, "keydown", move |event| {
                let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
                kbd_callback.emit(event.clone());
            });
            self.kbd_listener.replace(listener);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sz = ctx.props().grid_size;
        let mut items = Vec::with_capacity(sz * sz);
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
        }
        html! {
            <div style={AttrValue::from(format!("display: inline-grid; grid-template-columns: repeat({}, auto);", sz))}>
            { for items }
            </div>
        }
    }
}
