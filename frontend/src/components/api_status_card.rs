use yew::prelude::*;
use crate::services::api::check_api_health;
use crate::domain::types::ApiHealth;

#[function_component]
pub fn ApiStatusCard() -> Html {
    let health_state = use_state(|| None::<ApiHealth>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Verificar automáticamente al cargar el componente
    {
        let health_state = health_state.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                match check_api_health().await {
                    Ok(health) => {
                        health_state.set(Some(health));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            
            || {}
        });
    }

    html! {
        <div class="card">
            <div class="card-content">
                <div class="is-flex is-justify-content-space-between is-align-items-center mb-4">
                    <h3 class="title is-5">{ "Estado de la API" }</h3>
                    <div class="is-size-7 has-text-grey">
                        { "Última verificación: " }{ chrono::Utc::now().format("%H:%M:%S") }
                    </div>
                </div>

                if let Some(error) = (*error).as_ref() {
                    <div class="notification is-danger">
                        <strong>{ "Error al verificar la API:" }</strong>
                        <p>{ error }</p>
                    </div>
                } else if *loading {
                    <div class="has-text-centered">
                        <div class="spinner"></div>
                        <p class="mt-2">{ "Verificando conexión..." }</p>
                    </div>
                } else if let Some(health) = (*health_state).as_ref() {
                    <div class="content">
                        // Estado general
                        <div class="field is-grouped is-grouped-multiline">
                            <div class="control">
                                <div class="tags has-addons">
                                    <span class="tag">{ "Gateway" }</span>
                                    <span class={format!("tag is-{}", if health.gateway == "healthy" { "success" } else { "danger" })}>
                                        { &health.gateway }
                                    </span>
                                </div>
                            </div>
                            <div class="control">
                                <div class="tags has-addons">
                                    <span class="tag">{ "inBestia API" }</span>
                                    <span class={format!("tag is-{}", if health.inbestia_api == "available" { "success" } else { "danger" })}>
                                        { &health.inbestia_api }
                                    </span>
                                </div>
                            </div>
                        </div>

                        // Estado general
                        <div class="notification is-light">
                            <div class="is-flex is-align-items-center">
                                <span class={format!("icon mr-2 {}", if health.status == "healthy" { "has-text-success" } else { "has-text-danger" })}>
                                    {if health.status == "healthy" { "✅" } else { "❌" }}
                                </span>
                                <div>
                                    <strong>{ 
                                        if health.status == "healthy" { 
                                            "API disponible y funcionando correctamente" 
                                        } else { 
                                            "API no disponible o con problemas" 
                                        }
                                    }</strong>
                                    <br />
                                    <small class="has-text-grey">
                                        { "Verificación: " }{ &health.timestamp }
                                    </small>
                                </div>
                            </div>
                        </div>

                        // Error si existe
                        if let Some(error_msg) = &health.error {
                            <div class="notification is-warning">
                                <strong>{ "Detalles del error:" }</strong>
                                <p>{ error_msg }</p>
                            </div>
                        }

                        // Trace ID para debugging
                        if let Some(trace_id) = &health.trace_id {
                            <div class="is-size-7 has-text-grey">
                                { "Trace ID: " }{ trace_id }
                            </div>
                        }
                    </div>
                }
            </div>
        </div>
    }
}