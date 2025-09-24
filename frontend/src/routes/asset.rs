use yew::prelude::*;
use crate::components::asset_analysis_card::AssetAnalysisCard;

// Create alias to maintain compatibility
pub use AssetAnalysisWorking as AssetAnalysis;


#[function_component]
pub fn AssetAnalysisWorking() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "Asset Analysis" }</h1>
            <div class="content">
                <p class="subtitle">{ "Análisis de Activos de inBestia" }</p>
                <p>{ "Análisis técnico multitemporal para activos financieros" }</p>

                <div class="columns is-multiline mt-5">
                    <div class="column is-full">
                        <AssetAnalysisCard />
                    </div>
                </div>
            </div>
        </div>
    }
}