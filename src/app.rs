use yew::prelude::*;

use crate::line_plot::Figure;
use crate::line_plot::Line;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>
                { "Hello world!" }
                </p>
                <Figure>
                    <Line xys=vec![(10.0, 20.0), (20.0, 10.0), (30.0, 30.0)] />
                </Figure>
            </div>
        }
    }
}