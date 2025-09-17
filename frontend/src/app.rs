use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::layout::Layout;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout />
        </BrowserRouter>
    }
}
