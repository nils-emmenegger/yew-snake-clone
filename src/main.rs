mod grid;

use grid::Grid;
use yew::prelude::*;

struct App {
    playing: bool,
}

enum Msg {
    PressPlay,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App { playing: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::PressPlay => {
                self.playing = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                if self.playing {
                    <Grid grid_size={10}/>
                } else {
                    <button onclick={link.callback(|_| Msg::PressPlay)}>{ "Play game" }</button>
                }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
