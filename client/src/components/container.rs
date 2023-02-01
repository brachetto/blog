use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::app::Route;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    // 用于跳转到不同的路由
    let navigator = use_navigator().unwrap();

    let set_title = Callback::from(move |content: String| {
        // 设置网页的标题
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .set_title(&format!("{content} - Blog"));
    });

    let jump = move |route| Callback::from(move |_| navigator.push(&route));

    html! {
        <>
            <nav>
                <a onclick={jump(Route::Home)} class="brand">
                    <span>{ "Blog" }</span>
                </a>
                // <div class="menu">
                //     <a href="#" class="button icon-puzzle">{ "About" }</a>
                // </div>
            </nav>

            <ContextProvider<Callback<String>> context={set_title}>
                { for props.children.iter() }
            </ContextProvider<Callback<String>>>

        </>
    }
}
