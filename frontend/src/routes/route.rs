use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/api-status")]
    ApiStatus,
    #[at("/asset-analysis")]
    AssetAnalysis,
    #[not_found]
    #[at("/404")]
    NotFound,
}
