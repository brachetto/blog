use crate::components::{container::Container, home::Home, not_found::NotFound};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// 主路由
#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    html! {
        <Container>
        {
            match route {
                Route::Home => html! { <Home/> },
                Route::NotFound => html! { <NotFound/> },
            }
        }
        </Container>
    }
}
