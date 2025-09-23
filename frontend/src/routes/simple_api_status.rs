use yew::prelude::*;
use crate::components::{
    info_card::InfoCard,
    simple_health_status_card::SimpleHealthStatusCard,
    simple_system_metrics_card::SimpleSystemMetricsCard,
    simple_providers_status_card::SimpleProvidersStatusCard,
};

#[function_component]
pub fn SimpleApiStatus() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "API Status (Direct Connection)" }</h1>
            <div class="content">
                <p class="subtitle">{ "Estado de la API de inBestia - Conexión Directa" }</p>
                <p>{ "Monitoreo en tiempo real del estado de la conexión DIRECTA con la API de inBestia (sin middleware)" }</p>

                <div class="columns is-multiline mt-5">
                    // Health Status (Direct)
                    <div class="column is-full">
                        <SimpleHealthStatusCard />
                    </div>

                    // System Metrics (Direct)
                    <div class="column is-full">
                        <SimpleSystemMetricsCard />
                    </div>

                    // Providers Status (Direct)
                    <div class="column is-full">
                        <SimpleProvidersStatusCard />
                    </div>

                    // API Info (reutiliza el componente existente que ya funciona)
                    <div class="column is-full">
                        <InfoCard />
                    </div>
                </div>

                // Información de arquitectura
                <div class="notification is-info is-light mt-5">
                    <h4 class="title is-6">{ "ℹ️ Arquitectura Simplificada" }</h4>
                    <p>{ "Esta página utiliza conexión directa:" }</p>
                    <p><strong>{ "Ghost Frontend → InBestia API" }</strong></p>
                    <ul>
                        <li>{ "Sin middleware ni capas intermedias" }</li>
                        <li>{ "Tipos de datos que coinciden exactamente con la API" }</li>
                        <li>{ "Menos latencia y mayor simplicidad" }</li>
                        <li>{ "Debugging y mantenimiento simplificado" }</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}