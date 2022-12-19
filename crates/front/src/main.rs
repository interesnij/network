#![recursion_limit = "640"]
use async_std::task::current;
use gloo_utils::history;
//use models::user::{self, UserInfo};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::{Context, ContextProvider};
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

//mod models;
//mod pages;
//mod sections;
mod utils;
//use pages::*;
use utils::not_found::NotFound;
use utils::requests::*;


#[function_component(App)]
fn app() -> Html { 
    {
        html! {
            <>
                {"hello"}
            </>
        }
    }
}

//#[wasm_bindgen(start)]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().hydrate();
}
