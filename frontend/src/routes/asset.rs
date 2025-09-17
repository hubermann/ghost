use yew::prelude::*;
use crate::components::{info_card::InfoCard, api_status_card::ApiStatusCard, system_metrics_card::SystemMetricsCard};

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
                <p class="subtitle">{ "Portfolio Analysis" }</p>
                <p>{ "Analyze your financial assets and portfolio performance" }</p>
                
                <div class="columns is-multiline mt-5">
                    <div class="column is-half">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "Portfolio Overview" }</h3>
                                <p>{ "Coming soon: View your complete portfolio" }</p>
                            </div>
                        </div>
                    </div>
                    <div class="column is-half">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "Performance Metrics" }</h3>
                                <p>{ "Coming soon: Track your investment performance" }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
