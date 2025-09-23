use yew::prelude::*;
use crate::services::simple_api::get_providers_status;
use crate::domain::simple_types::ProviderStatus;

#[function_component]
pub fn SimpleProvidersStatusCard() -> Html {
    let providers_state = use_state(|| None::<Vec<ProviderStatus>>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Cargar estado de proveedores automáticamente al montar el componente
    {
        let providers_state = providers_state.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match get_providers_status().await {
                    Ok(providers) => {
                        providers_state.set(Some(providers));
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

    // Función para actualizar estado de proveedores manualmente
    let refresh_providers = {
        let providers_state = providers_state.clone();
        let loading = loading.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let providers_state = providers_state.clone();
            let loading = loading.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                match get_providers_status().await {
                    Ok(providers) => {
                        providers_state.set(Some(providers));
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
                    <h3 class="title is-5">{ "Estado de Proveedores (Direct API)" }</h3>
                    <button
                        class="button is-small is-light"
                        onclick={refresh_providers}
                        disabled={*loading}
                    >
                        {if *loading { "Cargando..." } else { "Actualizar" }}
                    </button>
                </div>

                if let Some(error) = (*error).as_ref() {
                    <div class="notification is-danger">
                        <strong>{ "Error:" }</strong>
                        <p>{ error }</p>
                    </div>
                } else if *loading {
                    <div class="has-text-centered">
                        <div class="spinner"></div>
                        <p class="mt-2">{ "Cargando estado de proveedores..." }</p>
                    </div>
                } else if let Some(providers) = (*providers_state).as_ref() {
                    <div class="content">
                        {for providers.iter().map(|provider| {
                            html! {
                                <div class="box mb-3">
                                    <div class="is-flex is-justify-content-space-between is-align-items-center">
                                        <div>
                                            <strong class="is-size-5">{ &provider.name }</strong>
                                            <br />
                                            <span class="has-text-grey is-size-7">{ &provider.type_code }</span>
                                        </div>
                                        <div class="field is-grouped is-grouped-multiline">
                                            <div class="control">
                                                <div class="tags has-addons">
                                                    <span class="tag">{ "Status" }</span>
                                                    <span class={format!("tag is-{}",
                                                        if provider.available && provider.active { "success" }
                                                        else if provider.available { "warning" }
                                                        else { "danger" }
                                                    )}>
                                                        {
                                                            if provider.available && provider.active { "Active" }
                                                            else if provider.available { "Available" }
                                                            else { "Unavailable" }
                                                        }
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="control">
                                                <div class="tags has-addons">
                                                    <span class="tag">{ "Response Time" }</span>
                                                    <span class={format!("tag is-{}",
                                                        if provider.response_time_ms < 500 { "success" }
                                                        else if provider.response_time_ms < 1000 { "warning" }
                                                        else { "danger" }
                                                    )}>
                                                        { format!("{}ms", provider.response_time_ms) }
                                                    </span>
                                                </div>
                                            </div>
                                            if let Some(rate_limit) = provider.rate_limit_remaining {
                                                <div class="control">
                                                    <div class="tags has-addons">
                                                        <span class="tag">{ "Rate Limit" }</span>
                                                        <span class="tag is-info">
                                                            { format!("{} remaining", rate_limit) }
                                                        </span>
                                                    </div>
                                                </div>
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        })}

                        <p class="has-text-grey-light is-size-7">
                            <strong>{ "Datos en tiempo real de InBestia API" }</strong>
                        </p>
                    </div>
                } else {
                    <div class="notification is-info">
                        { "No se pudo obtener el estado de los proveedores." }
                    </div>
                }
            </div>
        </div>
    }
}