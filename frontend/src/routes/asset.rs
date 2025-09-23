use yew::prelude::*;
use crate::components::{info_card::InfoCard, api_status_card::ApiStatusCard, system_metrics_card::SystemMetricsCard, asset_analysis_card::AssetAnalysisCard};

#[function_component]
pub fn ApiStatus() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "API Status" }</h1>
            <div class="content">
                <p class="subtitle">{ "Estado de la API de inBestia" }</p>
                <p>{ "Monitoreo en tiempo real del estado de la conexión con la API de inBestia" }</p>

                <div class="columns is-multiline mt-5">
                    <div class="column is-full">
                        <ApiStatusCard />
                    </div>
                    <div class="column is-full">
                        <SystemMetricsCard />
                    </div>
                    <div class="column is-full">
                        <InfoCard />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn AssetAnalysis() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "Asset Analysis" }</h1>
            <div class="content">
                <p class="subtitle">{ "Análisis de Activos" }</p>
                <p>{ "Herramientas de análisis técnico y fundamental" }</p>
                
                <div class="columns is-multiline mt-5">
                    <div class="column is-full">
                        <AssetAnalysisCard />
                    </div>
                </div>
            </div>
        </div>
    }
}
