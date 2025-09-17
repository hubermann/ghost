use yew::prelude::*;
use std::rc::Rc;
use crate::services::api::fetch_system_metrics;
use crate::domain::types::SystemMetricsResponse;

#[function_component]
pub fn SystemMetricsCard() -> Html {
    let metrics_state = use_state(|| None::<SystemMetricsResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Cargar métricas automáticamente al montar el componente
    let metrics_state_clone = metrics_state.clone();
    let loading_clone = loading.clone();
    let error_clone = error.clone();
    
    use_effect(move || {
        let metrics_state = metrics_state_clone.clone();
        let loading = loading_clone.clone();
        let error = error_clone.clone();
        
        loading.set(true);
        error.set(None);
        
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_system_metrics().await {
                Ok(metrics) => {
                    metrics_state.set(Some(metrics));
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

    // Función para actualizar métricas manualmente
    let refresh_metrics = {
        let metrics_state = Rc::new(metrics_state.clone());
        let loading = Rc::new(loading.clone());
        let error = Rc::new(error.clone());
        
        Callback::from(move |_| {
            let metrics_state = metrics_state.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_system_metrics().await {
                    Ok(metrics) => {
                        metrics_state.set(Some(metrics));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
        })
    };

    // Función para obtener el color de la barra de progreso según el valor
    let get_progress_color = |value: f64, is_reverse: bool| {
        let threshold = if is_reverse { 0.5 } else { 0.7 };
        if is_reverse {
            if value >= threshold { "is-success" } else if value >= 0.3 { "is-warning" } else { "is-danger" }
        } else {
            if value <= threshold { "is-success" } else if value <= 0.9 { "is-warning" } else { "is-danger" }
        }
    };

    html! {
        <div class="card">
            <div class="card-content">
                <div class="is-flex is-justify-content-space-between is-align-items-center mb-4">
                    <h3 class="title is-5">{ "Métricas del Sistema" }</h3>
                    <button 
                        class="button is-small is-light"
                        onclick={refresh_metrics}
                        disabled={*loading}
                    >
                        {if *loading { "Cargando..." } else { "Actualizar" }}
                    </button>
                </div>

                if let Some(error) = (*error).as_ref() {
                    <div class="notification is-danger">
                        <strong>{ "Error al obtener métricas:" }</strong>
                        <p>{ error }</p>
                    </div>
                }

                if *loading {
                    <div class="has-text-centered">
                        <div class="spinner"></div>
                        <p class="mt-2">{ "Cargando métricas del sistema..." }</p>
                    </div>
                } else if let Some(metrics_response) = (*metrics_state).as_ref() {
                    <div class="content">
                        <div class="columns is-multiline">
                            // CPU Usage
                            <div class="column is-half">
                                <div class="box">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center mb-2">
                                        <span class="has-text-weight-semibold">{ "Uso de CPU" }</span>
                                        <span class="tag is-light">{ format!("{:.1}%", metrics_response.data.cpu_usage) }</span>
                                    </div>
                                    <progress 
                                        class={format!("progress {}", get_progress_color(metrics_response.data.cpu_usage / 100.0, false))}
                                        value={metrics_response.data.cpu_usage.to_string()}
                                        max="100"
                                    ></progress>
                                </div>
                            </div>

                            // Memory Usage
                            <div class="column is-half">
                                <div class="box">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center mb-2">
                                        <span class="has-text-weight-semibold">{ "Uso de Memoria" }</span>
                                        <span class="tag is-light">{ format!("{:.1}%", metrics_response.data.memory_usage) }</span>
                                    </div>
                                    <progress 
                                        class={format!("progress {}", get_progress_color(metrics_response.data.memory_usage / 100.0, false))}
                                        value={metrics_response.data.memory_usage.to_string()}
                                        max="100"
                                    ></progress>
                                </div>
                            </div>

                            // Database Connections
                            <div class="column is-half">
                                <div class="box">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center mb-2">
                                        <span class="has-text-weight-semibold">{ "Conexiones DB" }</span>
                                        <span class="tag is-info">{ metrics_response.data.database_connections }</span>
                                    </div>
                                    <div class="has-text-centered">
                                        <span class="has-text-weight-bold is-size-3">{ metrics_response.data.database_connections }</span>
                                        <p class="help">{ "conexiones activas" }</p>
                                    </div>
                                </div>
                            </div>

                            // Cache Hit Ratio
                            <div class="column is-half">
                                <div class="box">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center mb-2">
                                        <span class="has-text-weight-semibold">{ "Cache Hit Ratio" }</span>
                                        <span class="tag is-light">{ format!("{:.1}%", metrics_response.data.cache_hit_ratio * 100.0) }</span>
                                    </div>
                                    <progress 
                                        class={format!("progress {}", get_progress_color(metrics_response.data.cache_hit_ratio, true))}
                                        value={(metrics_response.data.cache_hit_ratio * 100.0).to_string()}
                                        max="100"
                                    ></progress>
                                </div>
                            </div>

                            // Active Requests
                            <div class="column is-full">
                                <div class="box">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center mb-2">
                                        <span class="has-text-weight-semibold">{ "Requests Activos" }</span>
                                        <span class="tag is-primary">{ metrics_response.data.active_requests }</span>
                                    </div>
                                    <div class="has-text-centered">
                                        <span class="has-text-weight-bold is-size-2">{ metrics_response.data.active_requests }</span>
                                        <p class="help">{ "requests procesándose actualmente" }</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        if let Some(trace_id) = metrics_response.trace_id.as_ref() {
                            <p class="has-text-grey-light is-size-7">
                                <strong>{ "Trace ID:" }</strong> { trace_id }
                            </p>
                        }
                    </div>
                } else {
                    <div class="notification is-info">
                        { "No se pudieron obtener las métricas del sistema." }
                    </div>
                }
            </div>
        </div>
    }
}
