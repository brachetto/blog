use yew::prelude::*;

use crate::components::card::Card;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    // 通过 Callback 更改网页标题
    use_context::<Callback<String>>()
        .unwrap()
        .emit("找不到该页面".into());

    html! {
        <Card title={"找不到该页面"}>
            <p>{ "尝试换个地址？" }</p>
        </Card>
    }
}
