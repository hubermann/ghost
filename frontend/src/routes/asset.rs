use yew::prelude::*;
use crate::components::{info_card::InfoCard, api_status_card::ApiStatusCard, system_metrics_card::SystemMetricsCard, multi_temporal_analyzer::MultiTemporalAnalyzer};

// Create alias to maintain compatibility
pub use AssetAnalysisWorking as AssetAnalysis;

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
pub fn AssetAnalysisWorking() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "Asset Analysis" }</h1>
            <div class="content">
                <p class="subtitle">{ "Análisis de Activos de inBestia" }</p>
                <p>{ "Análisis técnico multitemporal para activos financieros" }</p>

                <div class="columns is-multiline mt-5">
                    <div class="column is-full">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "✅ Sistema Operativo" }</h3>
                                <p>{ "El componente Asset Analysis está funcionando correctamente" }</p>
                                <p class="is-size-7 has-text-grey">{ "Routing, WASM y componentes: OK" }</p>
                            </div>
                        </div>
                    </div>
                    <div class="column is-full">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "🎯 Multi-Temporal Analysis" }</h3>
                                <p>{ "MultiTemporalAnalyzer temporalmente deshabilitado para debugging" }</p>
                                <p class="is-size-7 has-text-grey">{ "Componente complejo - verificando si causa hang" }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}