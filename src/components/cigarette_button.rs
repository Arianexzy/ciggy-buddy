use dioxus::prelude::*;
use lucide_dioxus::Cigarette;
use crate::database;
// use std::time::Duration;


#[component]
pub fn CigaretteButton() -> Element {
    rsx! {
        button {
            class: "cigarette-button",
            onclick: move |_| async move {
                database::save_total_count().await;
                println!("Button clicked!!!");
            },
            Cigarette {},
        }
    }
}