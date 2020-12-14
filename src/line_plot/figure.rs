use yew::prelude::*;

pub struct Figure {
  props: FigureProps,
}

#[derive(Properties, Clone)]
pub struct FigureProps {
    pub children: Children,
}

impl Component for Figure {
    type Message = ();
    type Properties = FigureProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
      Self{
        props,
      }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <svg viewBox="0 0 300 100" xmlns="http://www.w3.org/2000/svg">
              { self.props.children.clone() }
            </svg>
        }
    }
}