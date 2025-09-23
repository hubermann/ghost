use yew::prelude::*;
use crate::api::timeframes::{TimeframeService, TimeframeMetadata};
use crate::config::AppConfig;
use std::collections::HashMap;
use web_sys::console;

#[derive(Clone, PartialEq, Properties)]
pub struct MultiTemporalAnalyzerProps {
    pub symbol: String,
}

#[derive(Clone, PartialEq)]
pub enum AnalysisState {
    Ready,
    Loading,
    Loaded(HashMap<String, f64>),
    Error(String),
}

#[function_component]
pub fn MultiTemporalAnalyzer(props: &MultiTemporalAnalyzerProps) -> Html {
    let timeframe_service = use_state(|| {
        TimeframeService::new(
            AppConfig::TIMEFRAMES_API_URL.to_string(),
            AppConfig::API_KEY.to_string()
        )
    });

    let analysis_state = use_state(|| AnalysisState::Ready);
    let multitemporal_timeframes = use_state(|| Vec::<TimeframeMetadata>::new());
    let confluence_score = use_state(|| 0.0);

    // Cargar configuración de timeframes al montar
    {
        let timeframe_service = timeframe_service.clone();
        let multitemporal_timeframes = multitemporal_timeframes.clone();
        use_effect(move || {
            let timeframe_service = timeframe_service.clone();
            let multitemporal_timeframes = multitemporal_timeframes.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let mut service = (*timeframe_service).clone();

                if let Err(error) = service.fetch_config().await {
                    console::log_1(&format!("Error loading timeframes config: {}", error).into());
                    return;
                }

                if let Ok(timeframes) = service.get_multitemporal_timeframes() {
                    console::log_1(&format!("✅ Loaded {} timeframes for multi-temporal analysis", timeframes.len()).into());
                    multitemporal_timeframes.set(timeframes);
                }
            });
            || {}
        });
    }

    let on_analyze = {
        let symbol = props.symbol.clone();
        let analysis_state = analysis_state.clone();
        let timeframe_service = timeframe_service.clone();
        let confluence_score = confluence_score.clone();
        let multitemporal_timeframes = multitemporal_timeframes.clone();

        Callback::from(move |_| {
            let symbol = symbol.clone();
            let analysis_state = analysis_state.clone();
            let timeframe_service = timeframe_service.clone();
            let confluence_score = confluence_score.clone();
            let multitemporal_timeframes = multitemporal_timeframes.clone();

            analysis_state.set(AnalysisState::Loading);

            wasm_bindgen_futures::spawn_local(async move {
                let service = (*timeframe_service).clone();
                let timeframes = (*multitemporal_timeframes).clone();

                console::log_1(&format!("🔄 Starting multi-temporal analysis for {}", symbol).into());

                let mut scores = HashMap::new();
                let mut successful_analyses = 0;

                // Analizar cada timeframe secuencialmente
                for timeframe in &timeframes {
                    console::log_1(&format!("📊 Analyzing {} on {}", symbol, timeframe.name).into());

                    // Convertir a formato API
                    match service.to_api_format(&timeframe.name) {
                        Ok(api_format) => {
                            console::log_1(&format!("✅ Mapped {} → {}", timeframe.name, api_format).into());

                            // Simular análisis (aquí iría la llamada real a /api/v1/indicators)
                            match simulate_technical_analysis(&symbol, &api_format).await {
                                Ok(score) => {
                                    scores.insert(timeframe.name.clone(), score);
                                    successful_analyses += 1;
                                    console::log_1(&format!("✅ {} analysis: {:.2}", timeframe.name, score).into());
                                }
                                Err(error) => {
                                    console::log_1(&format!("❌ {} analysis failed: {}", timeframe.name, error).into());
                                }
                            }
                        }
                        Err(error) => {
                            console::log_1(&format!("❌ Failed to map timeframe {}: {}", timeframe.name, error).into());
                        }
                    }

                    // Pequeña pausa entre requests
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        let _ = web_sys::window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 200)
                            .unwrap();
                    });
                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                }

                console::log_1(&format!("📈 Completed {} of {} timeframe analyses", successful_analyses, timeframes.len()).into());

                if !scores.is_empty() {
                    // Calcular confluence
                    match service.calculate_confluence(&scores) {
                        Ok(confluence) => {
                            console::log_1(&format!("🎯 Confluence score: {:.2}", confluence).into());
                            confluence_score.set(confluence);
                            analysis_state.set(AnalysisState::Loaded(scores));
                        }
                        Err(error) => {
                            analysis_state.set(AnalysisState::Error(format!("Failed to calculate confluence: {}", error)));
                        }
                    }
                } else {
                    analysis_state.set(AnalysisState::Error("No successful analyses".to_string()));
                }
            });
        })
    };

    // Función helper para renderizar cada timeframe
    let render_timeframe_result = |timeframe: &TimeframeMetadata, score: Option<f64>| {
        let (color_class, score_text) = match score {
            Some(s) => {
                let class = if s > 0.6 { "has-text-success" }
                           else if s > 0.4 { "has-text-warning" }
                           else { "has-text-danger" };
                (class, format!("{:.1}%", s * 100.0))
            }
            None => ("has-text-grey", "N/A".to_string())
        };

        html! {
            <div class="column is-one-fifth">
                <div class="box has-text-centered">
                    <h6 class="title is-6">{ &timeframe.display_name }</h6>
                    <p class="subtitle is-4">
                        <span class={color_class}>{ score_text }</span>
                    </p>
                    <p class="is-size-7 has-text-grey">
                        { format!("Weight: {:.1}", timeframe.weight) }
                    </p>
                </div>
            </div>
        }
    };

    html! {
        <div class="card">
            <div class="card-header">
                <p class="card-header-title">
                    { "🎯 Multi-Temporal Analysis" }
                </p>
            </div>
            <div class="card-content">
                <div class="field is-grouped">
                    <div class="control">
                        <p class="subtitle is-5">
                            { format!("Symbol: {}", props.symbol) }
                        </p>
                    </div>
                    <div class="control">
                        <button
                            class={classes!("button", "is-primary", if matches!(*analysis_state, AnalysisState::Loading) { Some("is-loading") } else { None })}
                            onclick={on_analyze}
                            disabled={multitemporal_timeframes.is_empty()}
                        >
                            { "Analyze" }
                        </button>
                    </div>
                </div>

                {match (*analysis_state).clone() {
                    AnalysisState::Ready => html! {
                        <div class="notification is-info is-light">
                            <p>{ format!("Ready to analyze {} timeframes", multitemporal_timeframes.len()) }</p>
                        </div>
                    },
                    AnalysisState::Loading => html! {
                        <div class="notification is-primary is-light">
                            <p>{ "🔄 Analyzing multiple timeframes..." }</p>
                        </div>
                    },
                    AnalysisState::Error(error) => html! {
                        <div class="notification is-danger is-light">
                            <p>{ format!("❌ Error: {}", error) }</p>
                        </div>
                    },
                    AnalysisState::Loaded(scores) => html! {
                        <>
                            <div class="notification is-success is-light">
                                <h5 class="title is-5">
                                    { format!("🎯 Confluence Score: {:.1}%", *confluence_score * 100.0) }
                                </h5>
                                <p>{ format!("Based on {} timeframe analyses", scores.len()) }</p>
                            </div>

                            <div class="columns is-multiline">
                                {for multitemporal_timeframes.iter().map(|tf| {
                                    let score = scores.get(&tf.name).copied();
                                    render_timeframe_result(tf, score)
                                })}
                            </div>
                        </>
                    }
                }}
            </div>
        </div>
    }
}

// Función de simulación - en la implementación real esto haría la llamada a /api/v1/indicators
async fn simulate_technical_analysis(symbol: &str, api_timeframe: &str) -> Result<f64, String> {
    console::log_1(&format!("🔬 Simulating analysis for {} on {}", symbol, api_timeframe).into());

    // Simular diferentes scores según el timeframe para demostrar la funcionalidad
    let score = match api_timeframe {
        "minute5" => 0.45,
        "minute15" => 0.52,
        "hour1" => 0.67,
        "hour4" => 0.73,
        "daily" => 0.81,
        "weekly" => 0.76,
        _ => 0.50,
    };

    // Simular variabilidad
    let variance = (js_sys::Math::random() - 0.5) * 0.2;
    let final_score = (score + variance).max(0.0).min(1.0);

    Ok(final_score)
}