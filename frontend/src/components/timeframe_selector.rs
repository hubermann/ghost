use yew::prelude::*;
use crate::api::timeframes::{TimeframeService, TimeframeMetadata};
use crate::config::AppConfig;
use web_sys::wasm_bindgen::JsCast;
use js_sys;

#[derive(Clone, PartialEq, Properties)]
pub struct TimeframeSelectorProps {
    pub value: String,
    pub on_change: Callback<String>,
}

#[derive(Clone, PartialEq)]
pub enum TimeframeState {
    Loading,
    Loaded(Vec<TimeframeMetadata>),
    Error(String),
}

#[function_component]
pub fn TimeframeSelector(props: &TimeframeSelectorProps) -> Html {
    let timeframes_state = use_state(|| TimeframeState::Loading);
    let selected_timeframe = use_state(|| props.value.clone());

    // Cargar timeframes al montar el componente
    {
        let timeframes_state = timeframes_state.clone();
        use_effect(move || {
            let timeframes_state = timeframes_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut service = TimeframeService::new(
                    AppConfig::TIMEFRAMES_API_URL.to_string(),
                    AppConfig::API_KEY.to_string()
                );

                match service.fetch_config().await {
                    Ok(_) => {
                        match service.get_timeframes() {
                            Ok(timeframes) => {
                                timeframes_state.set(TimeframeState::Loaded(timeframes.clone()));
                            }
                            Err(error) => {
                                timeframes_state.set(TimeframeState::Error(error));
                            }
                        }
                    }
                    Err(error) => {
                        timeframes_state.set(TimeframeState::Error(error));
                    }
                }
            });
            || {}
        });
    }

    // Actualizar selected_timeframe cuando cambie el prop value
    {
        let selected_timeframe = selected_timeframe.clone();
        use_effect_with(props.value.clone(), move |new_value| {
            selected_timeframe.set(new_value.clone());
        });
    }

    let on_change = {
        let on_change = props.on_change.clone();
        let selected_timeframe = selected_timeframe.clone();
        Callback::from(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                    // Usar Reflect.get para obtener el valor
                    if let Ok(value_js) = js_sys::Reflect::get(&element, &"value".into()) {
                        if let Some(value) = value_js.as_string() {
                            selected_timeframe.set(value.clone());
                            on_change.emit(value);
                        }
                    }
                }
            }
        })
    };

    match (*timeframes_state).clone() {
        TimeframeState::Loading => {
            html! {
                <div class="select is-loading">
                    <select disabled={true}>
                        <option>{ "Cargando timeframes..." }</option>
                    </select>
                </div>
            }
        }
        TimeframeState::Error(error) => {
            html! {
                <div class="field">
                    <div class="select is-danger">
                        <select disabled={true}>
                            <option>{ "Sin conexión a la API" }</option>
                        </select>
                    </div>
                    <p class="help is-danger">{ error }</p>
                </div>
            }
        }
        TimeframeState::Loaded(timeframes) => {
            html! {
                <div class="select">
                    <select 
                        value={(*selected_timeframe).clone()}
                        onchange={on_change}
                    >
                        {for timeframes.iter().map(|timeframe| {
                            let is_selected = timeframe.name == *selected_timeframe;
                            html! {
                                <option 
                                    value={timeframe.name.clone()}
                                    selected={is_selected}
                                >
                                    { &timeframe.display_name }
                                </option>
                            }
                        })}
                    </select>
                </div>
            }
        }
    }
}