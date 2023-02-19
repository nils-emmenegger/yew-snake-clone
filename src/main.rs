mod grid;

use grid::Grid;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let playing = use_state(|| false);
    let onclick = {
        let playing = playing.clone();
        Callback::from(move |_| playing.set(true))
    };
    html! {
        <div>
            if *playing {
                <Grid grid_size={10}/>
            } else {
                <button {onclick}>{ "Play game" }</button>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
