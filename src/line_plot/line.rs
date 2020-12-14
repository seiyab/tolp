use yew::prelude::*;

pub struct Line {
  props: LineProps,
}

#[derive(Properties, Clone)]
pub struct LineProps {
    pub xys: Vec<(f64, f64)>,
    #[prop_or(String::from("black"))]
    pub stroke: String,
}


impl Component for Line {
    type Message = ();
    type Properties = LineProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
      Self{
        props,
      }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
      self.props = props;
      true
    }

    fn view(&self) -> Html {
        // let lines = self.props.xys.iter().zip(self.props.xys[1..].iter());
        let points = self.props.xys.iter()
          .map(|(a, b)| format!("{},{}", a, b))
          .collect::<Vec<String>>()
          .join(" ");
        html! {
          <polyline
            points=points
            stroke=self.props.stroke
            fill="none"
          />
        }
    }
}
