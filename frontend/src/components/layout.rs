use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use web_sys::window;
use crate::routes::{Route, Home, ApiStatus, AssetAnalysis};

const THEME_KEY: &str = "ghost-theme";

#[function_component]
pub fn Layout() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::Home);
    
    // Cargar tema desde localStorage o usar tema claro por defecto
    let is_dark_theme = use_state(|| {
        LocalStorage::get::<bool>(THEME_KEY).unwrap_or(false)
    });

    let toggle_theme = {
        let is_dark_theme = is_dark_theme.clone();
        Callback::from(move |_| {
            let new_theme = !*is_dark_theme;
            is_dark_theme.set(new_theme);
            // Guardar en localStorage
            let _ = LocalStorage::set(THEME_KEY, new_theme);
        })
    };

    let theme_class = if *is_dark_theme { "is-dark" } else { "" };

    // Aplicar tema al body y html del documento - tanto al inicializar como al cambiar
    use_effect_with(
        *is_dark_theme,
        |is_dark| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    // Aplicar al body
                    if let Some(body) = document.body() {
                        let class_name = if *is_dark { "app is-dark" } else { "app" };
                        let _ = body.set_class_name(class_name);
                    }
                    // Aplicar al html
                    if let Some(html) = document.document_element() {
                        if *is_dark {
                            let _ = html.set_class_name("is-dark");
                        } else {
                            // Remover la clase is-dark explícitamente para asegurar tema claro
                            let _ = html.set_class_name("");
                        }
                    }
                }
            }
            || ()
        }
    );

    // También aplicar el tema inmediatamente al renderizar (inicial)
    use_effect_with((), |_| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                // Forzar tema claro inicial si no hay tema guardado
                let is_dark = LocalStorage::get::<bool>(THEME_KEY).unwrap_or(false);

                if let Some(body) = document.body() {
                    let class_name = if is_dark { "app is-dark" } else { "app" };
                    let _ = body.set_class_name(class_name);
                }

                if let Some(html) = document.document_element() {
                    if is_dark {
                        let _ = html.set_class_name("is-dark");
                    } else {
                        let _ = html.set_class_name("");
                    }
                }
            }
        }
        || ()
    });

    html! {
        <div class={format!("app {}", theme_class)} style="height: 100vh;">
            <div class="columns is-gapless" style="height: 100vh;">
                // Sidebar
                <div class="column is-narrow" style="width: 220px; min-width: 220px; max-width: 220px;">
                    <aside class="menu is-fullheight" style="height: 100vh; overflow-y: auto; padding: 0;">
                        <div class="menu-label">
                            <div class="is-flex is-justify-content-space-between is-align-items-center">
                                <span class="has-text-weight-bold">{ "Ghost Dashboard" }</span>
                                <button 
                                    class="button is-small is-ghost"
                                    onclick={toggle_theme}
                                    title={if *is_dark_theme { "Switch to light theme" } else { "Switch to dark theme" }}
                                >
                                    {if *is_dark_theme { "☀️" } else { "🌙" }}
                                </button>
                            </div>
                        </div>
                        <ul class="menu-list">
                            <li>
                                <Link<Route> to={Route::Home} classes={if current_route == Route::Home { "is-active" } else { "" }}>
                                    <span>{ "Home" }</span>
                                </Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::ApiStatus} classes={if current_route == Route::ApiStatus { "is-active" } else { "" }}>
                                    <span>{ "API Status" }</span>
                                </Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::AssetAnalysis} classes={if current_route == Route::AssetAnalysis { "is-active" } else { "" }}>
                                    <span>{ "Asset Analysis" }</span>
                                </Link<Route>>
                            </li>
                        </ul>
                    </aside>
                </div>

                // Main content
                <div class="column" style="flex: 1; min-width: 0;">
                    <div class="is-fullheight" style="overflow-y: auto; padding: 1.5rem; height: 100vh; box-sizing: border-box;">
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ApiStatus => html! { <ApiStatus /> },
        Route::AssetAnalysis => html! { <AssetAnalysis /> },
        Route::NotFound => html! {
            <div class="container">
                <h1 class="title">{ "404 - Page Not Found" }</h1>
                <p>{ "The page you're looking for doesn't exist." }</p>
            </div>
        },
    }
}
