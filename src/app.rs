use rusty_gun::Node;
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Model {
    value: i64,
    node: Node,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let node = Node::new(&JsValue::NULL);
        Self {
            value: 0,
            node,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SubOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
                <p>{ format!("{:?}", self.node) }</p>
            </div>
        }
    }
}
