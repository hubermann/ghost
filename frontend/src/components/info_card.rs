use yew::prelude::*;
use crate::services::api::fetch_info;
use crate::domain::types::ApiInfo;

/// Componente que muestra información de la API
#[function_component(InfoCard)]
pub fn info_card() -> Html {
    let info = use_state(|| Option::<ApiInfo>::None);
    let err  = use_state(|| Option::<String>::None);

    {
        let info = info.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_info().await {
                    Ok(v) => info.set(Some(v)),
                    Err(e) => err.set(Some(e)),
                }
            });
            || ()
        });
    }

    html! {
      <>
        if let Some(e) = &*err {
          <div class="notification is-danger">{ format!("Error: {e}") }</div>
        } else if let Some(i) = &*info {
          <div class="card">
            <header class="card-header"><p class="card-header-title">{ "API Info" }</p></header>
            <div class="card-content">
              <p><b>{ "Name: " }</b>{ &i.name }</p>
              <p><b>{ "Version: " }</b>{ &i.version }</p>
              <p><b>{ "Description: " }</b>{ &i.description }</p>
              <p><b>{ "Data Mode: " }</b>{ &i.data_mode }</p>
              <p><b>{ "Auth Type: " }</b>{ &i.authentication.auth_type }</p>
              <p><b>{ "Endpoints: " }</b>{ i.endpoints.len() }</p>
            </div>
          </div>
        } else {
          <progress class="progress is-small is-primary" max="100">{ "Loading" }</progress>
        }
      </>
    }
}
