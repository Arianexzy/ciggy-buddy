use crate::storage::storage;
use dioxus::prelude::*;
use lucide_dioxus::Cigarette;
// use std::time::Duration;

#[component]
pub fn CigaretteButton() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            class: "cigarette-button",
            onclick: move |_| {
                storage::add_smoking_event();
                count.set(storage::get_total_count());
            },
            Cigarette {},
        }
        div {
            "Total: {count()}"
        }
    }
}
