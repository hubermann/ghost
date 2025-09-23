use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::domain::analysis_types::{AnalysisRequest, AnalysisState, TimeframesState};
use crate::services::analysis_api::{analyze_asset, fetch_timeframes_config};
use crate::components::timeframe_selector::TimeframeSelector;

#[function_component]
pub fn AssetAnalysisCard() -> Html {
    let symbol = use_state(|| String::new());
    let timeframe = use_state(|| String::new());
    let analysis_state = use_state(|| AnalysisState::Idle);
    let timeframes_state = use_state(|| TimeframesState::Loading);

    // Cargar timeframes al montar el componente
    {
        let timeframes_state = timeframes_state.clone();
        let timeframe = timeframe.clone();
        use_effect(move || {
            let timeframes_state = timeframes_state.clone();
            let timeframe = timeframe.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_timeframes_config().await {
                    Ok(response) => {
                        let data = response.timeframes.clone();
                        timeframes_state.set(TimeframesState::Loaded(data.clone()));
                        // Establecer el primer timeframe como valor por defecto solo si está vacío
                        if timeframe.is_empty() {
                            if let Some(first_timeframe) = data.first() {
                                timeframe.set(first_timeframe.name.clone());
                            }
                        }
                    }
                    Err(error) => {
                        timeframes_state.set(TimeframesState::Error(error));
                    }
                }
            });
            || {}
        });
    }

    let on_symbol_change = {
        let symbol = symbol.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            symbol.set(input.value());
        })
    };

    let on_timeframe_change = {
        let timeframe = timeframe.clone();
        Callback::from(move |new_timeframe: String| {
            timeframe.set(new_timeframe);
        })
    };

    let on_submit = {
        let symbol = symbol.clone();
        let timeframe = timeframe.clone();
        let analysis_state = analysis_state.clone();
        let timeframes_state = timeframes_state.clone();
        
        Callback::from(move |_| {
            let symbol = symbol.clone();
            let timeframe = timeframe.clone();
            let analysis_state = analysis_state.clone();
            if (*symbol).is_empty() {
                analysis_state.set(AnalysisState::Error("El símbolo del activo es requerido".to_string()));
                return;
            }

            analysis_state.set(AnalysisState::Loading);
            
            // Busca el TimeframeConfig correspondiente y usa to_analysis_format():
            let analysis_timeframe = match (*timeframes_state).clone() {
                TimeframesState::Loaded(configs) => {
                    configs.iter()
                        .find(|config| config.name == *timeframe)
                        .map(|config| config.to_analysis_format())
                        .unwrap_or_else(|| (*timeframe).clone())
                }
                _ => (*timeframe).clone(),
            };

            let request = AnalysisRequest {
                symbol: (*symbol).clone(),
                timeframe: analysis_timeframe,
                include_fundamental: true,
            };
            
            wasm_bindgen_futures::spawn_local(async move {
                match analyze_asset(request).await {
                    Ok(response) => {
                        analysis_state.set(AnalysisState::Success(response));
                    }
                    Err(error) => {
                        // Parsear errores de símbolo no encontrado
                        if error.starts_with("SYMBOL_NOT_FOUND:") {
                            if let Ok(symbol_error) = serde_json::from_str::<crate::domain::analysis_types::SymbolErrorResponse>(&error[17..]) {
                                analysis_state.set(AnalysisState::SymbolNotFound(symbol_error));
                            } else {
                                analysis_state.set(AnalysisState::Error(error));
                            }
                        } else {
                            analysis_state.set(AnalysisState::Error(error));
                        }
                    }
                }
            });
        })
    };

    let clear_error = {
        let analysis_state = analysis_state.clone();
        Callback::from(move |_| {
            analysis_state.set(AnalysisState::Idle);
        })
    };

    let on_suggestion_click = {
        let symbol = symbol.clone();
        let analysis_state = analysis_state.clone();
        Callback::from(move |suggested_symbol: String| {
            symbol.set(suggested_symbol);
            analysis_state.set(AnalysisState::Idle);
        })
    };

    let is_loading = matches!(*analysis_state, AnalysisState::Loading);
    let timeframes_loading = matches!(*timeframes_state, TimeframesState::Loading);
    let timeframes_error = matches!(*timeframes_state, TimeframesState::Error(_));
    let is_disabled = is_loading || (*symbol).is_empty() || timeframes_loading || timeframes_error;

    html! {
        <div class="card">
            <div class="card-content">
                <h3 class="title is-5">{ "Análisis de Activos" }</h3>
                <p class="subtitle is-6">{ "Selecciona un activo y temporalidad para analizar" }</p>
                
                <div class="field is-grouped">
                    <div class="control is-expanded">
                        <label class="label">{ "Símbolo del Activo" }</label>
                        <input 
                            class="input" 
                            type="text" 
                            placeholder="Ej: AAPL, MSFT, GOOGL"
                            value={(*symbol).clone()}
                            oninput={on_symbol_change}
                            disabled={is_loading}
                        />
                    </div>
                    
                    <div class="control">
                        <label class="label">{ "Temporalidad" }</label>
                        {match (*timeframes_state).clone() {
                            TimeframesState::Loading => {
                                html! {
                                    <div class="select is-loading">
                                        <select disabled={true}>
                                            <option>{ "Cargando timeframes..." }</option>
                                        </select>
                                    </div>
                                }
                            }
                            TimeframesState::Error(error) => {
                                html! {
                                    <div class="select">
                                        <select disabled={true}>
                                            <option>{ format!("Error: {}", error) }</option>
                                        </select>
                                    </div>
                                }
                            }
                            TimeframesState::Loaded(_) => {
                                html! {
                                    <TimeframeSelector 
                                        value={(*timeframe).clone()}
                                        on_change={on_timeframe_change}
                                    />
                                }
                            }
                        }}
                    </div>
                    
                    <div class="control">
                        <label class="label">{ " " }</label>
                        <button 
                            class="button is-primary"
                            onclick={on_submit}
                            disabled={is_disabled}
                        >
                            {if is_loading {
                                html! { 
                                    <>
                                        <span class="spinner"></span>
                                        { "Analizando..." }
                                    </>
                                }
                            } else {
                                html! { "Analizar" }
                            }}
                        </button>
                    </div>
                </div>
                
                {match (*analysis_state).clone() {
                    AnalysisState::Error(error) => {
                        html! {
                            <div class="notification is-danger">
                                <button class="delete" onclick={clear_error}></button>
                                <strong>{ "Error: " }</strong>
                                { error }
                            </div>
                        }
                    }
                    AnalysisState::SymbolNotFound(symbol_error) => {
                        html! {
                            <div class="notification is-warning">
                                <button class="delete" onclick={clear_error}></button>
                                <strong>{ "Símbolo no encontrado: " }</strong>
                                <p>{ &symbol_error.message }</p>
                                <div class="mt-3">
                                    <p class="has-text-weight-semibold">{ "Sugerencias:" }</p>
                                    <div class="buttons">
                                        {symbol_error.suggestions.iter().map(|suggestion| {
                                            let suggestion_clone = suggestion.clone();
                                            let on_click = {
                                                let on_suggestion_click = on_suggestion_click.clone();
                                                let suggestion = suggestion.clone();
                                                Callback::from(move |_| {
                                                    on_suggestion_click.emit(suggestion.clone());
                                                })
                                            };
                                            html! {
                                                <button
                                                    class="button is-small is-outlined is-primary"
                                                    onclick={on_click}
                                                    key={suggestion_clone}
                                                >
                                                    { suggestion }
                                                </button>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                </div>
                            </div>
                        }
                    }
                    AnalysisState::Success(response) => {
                        html! {
                            <div class="notification is-success">
                                <h4 class="title is-6">{ "Resultado del Análisis:" }</h4>
                                <div class="content">
                                    <div class="columns">
                                        <div class="column">
                                            <p><strong>{ "Símbolo: " }</strong>{ &response.symbol }</p>
                                            <p><strong>{ "Timeframe: " }</strong>{ &response.timeframe }</p>
                                            <p><strong>{ "Puntuación General: " }</strong>{ format!("{:.2}", response.score) }</p>
                                            <p><strong>{ "Operación Sugerida: " }</strong>{ &response.suggested_operation }</p>
                                        </div>
                                        <div class="column">
                                            <p><strong>{ "Puntuación Técnica: " }</strong>{ format!("{:.2}", response.technical_score) }</p>
                                            <p><strong>{ "Puntuación de Tendencia: " }</strong>{ format!("{:.2}", response.trend_score) }</p>
                                            <p><strong>{ "Puntuación de Momentum: " }</strong>{ format!("{:.2}", response.momentum_score) }</p>
                                            <p><strong>{ "Puntuación de Volatilidad: " }</strong>{ format!("{:.2}", response.volatility_score) }</p>
                                        </div>
                                    </div>
                                    <div class="mt-3">
                                        <p><strong>{ "Explicación: " }</strong></p>
                                        <p class="is-italic">{ &response.explanation }</p>
                                    </div>
                                </div>
                            </div>
                        }
                    }
                    _ => html! {}
                }}
            </div>
        </div>
    }
}