use crate::icons::{Cloud, CloudSnow};
use crate::utils::{getItem, setItem};
use leptos::wasm_bindgen;
use leptos::wasm_bindgen::prelude::*;
use leptos::{component, create_effect, create_signal, view, IntoView};

#[wasm_bindgen(module = "/public/utils.js")]
extern "C" {
    pub fn getSnowStatus() -> JsValue;
    pub fn setSnowStatus(value: bool) -> JsValue;
}

#[wasm_bindgen]
pub fn load_pref() -> bool {
    let snow_enabled = {
        let value = getItem("snow");
        if value.is_null() || value.is_undefined() {
            setItem("snow", "true");
            "true".to_string()
        } else {
            value.as_string().unwrap_or_default()
        }
    };

    if snow_enabled != "true" {
        setSnowStatus(false);
    }

    return snow_enabled == "true";
}

#[component]
pub fn SnowToggle() -> impl IntoView {
    let (enabled, set_enabled) = create_signal(true);

    let icon = move || {
        if enabled() {
            view! { <CloudSnow /> }
        } else {
            view! { <Cloud /> }
        }
    };

    create_effect(move |_| {
        let s = load_pref();
        set_enabled(s);
    });

    view! {
        <button on:click=move |_| {
            let status = setSnowStatus(!getSnowStatus().as_bool().unwrap()).as_bool().expect(".");
            set_enabled(status);
            setItem("snow", format!("{}", status).as_str());
        }>{icon}</button>
    }
}
