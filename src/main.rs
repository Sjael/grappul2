use yew::prelude::*;
use stylist::yew::{styled_component,Global};
use stylist::{style, Style};

const STYLE_FILE :&str = include_str!("main.css");

enum Msg {
    AddOne,
}


#[function_component(App)]
pub fn app() -> Html {
    let god_list = vec!["agni", "merlin"];
    html! {
        <div class={classes!("container")}>
            {
                god_list.into_iter().map(|name| {
                    html!{
                        <div>
                            <img src={format!("img/{}.png", name)} alt={name}/>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
fn main() {
    yew::start_app::<App>();
}
