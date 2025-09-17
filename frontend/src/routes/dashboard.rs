use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{ "Home" }</h1>
            <div class="content">
                <p class="subtitle">{ "Welcome to Ghost Dashboard" }</p>
                <p>{ "Your financial dashboard powered by inBestia API" }</p>
                
                <div class="columns is-multiline mt-5">
                    <div class="column is-one-third">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "API Status" }</h3>
                                <p>{ "Check the current status of the inBestia API" }</p>
                            </div>
                        </div>
                    </div>
                    <div class="column is-one-third">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "Asset Analysis" }</h3>
                                <p>{ "Analyze your portfolio and assets" }</p>
                            </div>
                        </div>
                    </div>
                    <div class="column is-one-third">
                        <div class="card">
                            <div class="card-content">
                                <h3 class="title is-5">{ "Quick Stats" }</h3>
                                <p>{ "View your financial overview" }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
