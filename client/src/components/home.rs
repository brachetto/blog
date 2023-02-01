use yew::prelude::*;

use crate::components::card::Card;

#[function_component(Home)]
pub fn home() -> Html {
    // 通过 Callback 更改网页标题
    use_context::<Callback<String>>()
        .unwrap()
        .emit("Home".into());

    html! {
        <Card title={"Welcome!"}>
            <p>{ "Just a blog..." }</p>
        </Card>
    }
}
