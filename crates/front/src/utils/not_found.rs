use yew::{function_component, html, Component, Context, Properties};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    
    html!{
        <section  class="section">
            <h2 class="title">{"404 Page not found"}</h2>

            <a href="/" class="button is-primary">{"back"}</a>
        </section>
    }

}