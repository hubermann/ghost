use crate::domain::analysis_types::{AnalysisRequest, AnalysisResponse, TimeframesConfigResponse, SymbolErrorResponse};
use gloo_net::http::Request;
use crate::config::AppConfig;

pub async fn analyze_asset(request: AnalysisRequest) -> Result<AnalysisResponse, String> {
    let request_body = serde_json::to_value(&request)
        .map_err(|e| format!("Error serializando request: {}", e))?;

    // Debug logging
    web_sys::console::log_1(&format!("Sending request: {:?}", request_body).into());

    let response = Request::post(&AppConfig::analyze_url())
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Error creando request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if response.ok() {
        let analysis_response: AnalysisResponse = response
            .json()
            .await
            .map_err(|e| format!("Error parseando respuesta: {}", e))?;
        Ok(analysis_response)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());

        // Intentar parsear como error de símbolo no encontrado
        if status == 400 {
            if let Ok(symbol_error) = serde_json::from_str::<SymbolErrorResponse>(&error_text) {
                return Err(format!("SYMBOL_NOT_FOUND:{}", serde_json::to_string(&symbol_error).unwrap()));
            }
        }

        Err(format!("Error del servidor ({}): {}", status, error_text))
    }
}

pub async fn fetch_timeframes_config() -> Result<TimeframesConfigResponse, String> {
    let response = Request::get(&AppConfig::timeframes_config_url())
        .send()
        .await
        .map_err(|e| format!("Error de conexión: {}", e))?;

    if response.ok() {
        let timeframes_response: TimeframesConfigResponse = response
            .json()
            .await
            .map_err(|e| format!("Error parseando respuesta: {}", e))?;
        Ok(timeframes_response)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
        Err(format!("Error del servidor ({}): {}", status, error_text))
    }
}
