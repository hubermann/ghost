use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement, Event};
use wasm_bindgen::JsCast;
use crate::domain::analysis_types::{AnalysisRequest, AnalysisState, TimeframesState};
use crate::services::analysis_api::analyze_asset;

#[function_component]
pub fn AssetAnalysisCard() -> Html {
    let symbol = use_state(|| String::new());
    let timeframe = use_state(|| String::new());
    let analysis_state = use_state(|| AnalysisState::Idle);
    // Usar timeframes estáticos con más opciones
    let timeframes_state = use_state(|| {
        use crate::domain::analysis_types::TimeframeConfig;
        let static_timeframes = vec![
            TimeframeConfig {
                name: "1m".to_string(),
                display_name: "1 Minuto".to_string(),
                duration_seconds: 60,
                weight: 0.1,
                category: "Short Term".to_string(),
                aliases: vec!["1m".to_string(), "1min".to_string()],
                recommended_limit: 1440,
                max_gap_hours: 1,
            },
            TimeframeConfig {
                name: "5m".to_string(),
                display_name: "5 Minutos".to_string(),
                duration_seconds: 300,
                weight: 0.2,
                category: "Short Term".to_string(),
                aliases: vec!["5m".to_string(), "5min".to_string()],
                recommended_limit: 576,
                max_gap_hours: 1,
            },
            TimeframeConfig {
                name: "15m".to_string(),
                display_name: "15 Minutos".to_string(),
                duration_seconds: 900,
                weight: 0.3,
                category: "Short Term".to_string(),
                aliases: vec!["15m".to_string(), "15min".to_string()],
                recommended_limit: 192,
                max_gap_hours: 2,
            },
            TimeframeConfig {
                name: "30m".to_string(),
                display_name: "30 Minutos".to_string(),
                duration_seconds: 1800,
                weight: 0.4,
                category: "Medium Term".to_string(),
                aliases: vec!["30m".to_string(), "30min".to_string()],
                recommended_limit: 96,
                max_gap_hours: 2,
            },
            TimeframeConfig {
                name: "1h".to_string(),
                display_name: "1 Hora".to_string(),
                duration_seconds: 3600,
                weight: 0.5,
                category: "Medium Term".to_string(),
                aliases: vec!["1h".to_string(), "hourly".to_string()],
                recommended_limit: 168,
                max_gap_hours: 4,
            },
            TimeframeConfig {
                name: "4h".to_string(),
                display_name: "4 Horas".to_string(),
                duration_seconds: 14400,
                weight: 0.7,
                category: "Medium Term".to_string(),
                aliases: vec!["4h".to_string(), "4hours".to_string()],
                recommended_limit: 180,
                max_gap_hours: 12,
            },
            TimeframeConfig {
                name: "1d".to_string(),
                display_name: "Diario".to_string(),
                duration_seconds: 86400,
                weight: 1.0,
                category: "Long Term".to_string(),
                aliases: vec!["1d".to_string(), "daily".to_string()],
                recommended_limit: 365,
                max_gap_hours: 48,
            },
            TimeframeConfig {
                name: "1w".to_string(),
                display_name: "Semanal".to_string(),
                duration_seconds: 604800,
                weight: 1.5,
                category: "Long Term".to_string(),
                aliases: vec!["1w".to_string(), "weekly".to_string()],
                recommended_limit: 260,
                max_gap_hours: 168,
            },
            TimeframeConfig {
                name: "1M".to_string(),
                display_name: "Mensual".to_string(),
                duration_seconds: 2592000,
                weight: 2.0,
                category: "Long Term".to_string(),
                aliases: vec!["1M".to_string(), "monthly".to_string()],
                recommended_limit: 120,
                max_gap_hours: 744,
            },
        ];
        TimeframesState::Loaded(static_timeframes)
    });

    // Establecer timeframe por defecto
    {
        let timeframe = timeframe.clone();
        use_effect(move || {
            if timeframe.is_empty() {
                timeframe.set("1d".to_string());
            }
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
                            TimeframesState::Loaded(timeframes) => {
                                html! {
                                    <div class="select">
                                        <select
                                            value={(*timeframe).clone()}
                                            onchange={on_timeframe_change.reform(|e: Event| {
                                                e.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
                                                    .map(|select| select.value())
                                                    .unwrap_or_default()
                                            })}
                                        >
                                            {for timeframes.iter().map(|tf| {
                                                html! {
                                                    <option
                                                        value={tf.name.clone()}
                                                        selected={tf.name == *timeframe}
                                                    >
                                                        { &tf.display_name }
                                                    </option>
                                                }
                                            })}
                                        </select>
                                    </div>
                                }
                            }
                            _ => {
                                html! {
                                    <div class="select">
                                        <select disabled={true}>
                                            <option>{ "Timeframes no disponibles" }</option>
                                        </select>
                                    </div>
                                }
                            }
                        }}
                    </div>
                    
                    <div class="control">
                        <label class="label">{ "\u{00A0}" }</label>
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