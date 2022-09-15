use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    counter: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
      let link = ctx.link();
        html!(
          <div class="container">
            <p>{self.counter}</p>
            <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
          </div>
        )
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
