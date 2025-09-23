use yew::prelude::*;
use crate::services::simple_api::get_health_status;
use crate::domain::simple_types::SimpleHealthStatus;

#[function_component]
pub fn SimpleHealthStatusCard() -> Html {
    let health_state = use_state(|| None::<SimpleHealthStatus>);
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
                match get_health_status().await {
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

    // Función para verificar salud manualmente
    let refresh_health = {
        let health_state = health_state.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let health_state = health_state.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match get_health_status().await {
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
        })
    };

    html! {
        <div class="card">
            <div class="card-content">
                <div class="is-flex is-justify-content-space-between is-align-items-center mb-4">
                    <h3 class="title is-5">{ "Estado de la API (Direct)" }</h3>
                    <div class="is-size-7 has-text-grey">
                        { "Última verificación: " }{ chrono::Utc::now().format("%H:%M:%S") }
                    </div>
                </div>

                if let Some(error) = (*error).as_ref() {
                    <div class="notification is-danger">
                        <strong>{ "Error al verificar la API:" }</strong>
                        <p>{ error }</p>
                        <button
                            class="button is-small is-danger is-outlined mt-2"
                            onclick={refresh_health}
                        >
                            { "Reintentar" }
                        </button>
                    </div>
                } else if *loading {
                    <div class="has-text-centered">
                        <div class="spinner"></div>
                        <p class="mt-2">{ "Verificando conexión..." }</p>
                    </div>
                } else if let Some(health) = (*health_state).as_ref() {
                    <div class="content">
                        // Estado general
                        <div class="notification is-light">
                            <div class="is-flex is-align-items-center">
                                <span class={format!("icon mr-2 {}", if health.status == "healthy" { "has-text-success" } else { "has-text-danger" })}>
                                    {if health.status == "healthy" { "✅" } else { "❌" }}
                                </span>
                                <div class="is-flex-grow-1">
                                    <strong>{
                                        if health.status == "healthy" {
                                            "API disponible y funcionando correctamente"
                                        } else {
                                            "API no disponible o con problemas"
                                        }
                                    }</strong>
                                    <br />
                                    <small class="has-text-grey">
                                        { "Respuesta del servidor: " }{ &health.message }
                                    </small>
                                </div>
                                <button
                                    class="button is-small is-light"
                                    onclick={refresh_health}
                                >
                                    { "Verificar" }
                                </button>
                            </div>
                        </div>

                        // Indicador de estado
                        <div class="field is-grouped is-grouped-multiline">
                            <div class="control">
                                <div class="tags has-addons">
                                    <span class="tag">{ "InBestia API" }</span>
                                    <span class={format!("tag is-{}", if health.status == "healthy" { "success" } else { "danger" })}>
                                        { &health.status }
                                    </span>
                                </div>
                            </div>
                            <div class="control">
                                <div class="tags has-addons">
                                    <span class="tag">{ "Connection" }</span>
                                    <span class="tag is-info">
                                        { "Direct" }
                                    </span>
                                </div>
                            </div>
                        </div>

                        <p class="has-text-grey-light is-size-7">
                            <strong>{ "Conexión directa a InBestia API (sin middleware)" }</strong>
                        </p>
                    </div>
                }
            </div>
        </div>
    }
}