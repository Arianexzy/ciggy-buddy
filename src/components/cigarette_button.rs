use crate::storage::storage;
use dioxus::prelude::*;
use lucide_dioxus::Cigarette;
// use std::time::Duration;

#[derive(PartialEq, Clone, Props)]
pub struct CigaretteButtonProps {
    count_resource: Resource<i32>,
}

#[component]
pub fn CigaretteButton(props: CigaretteButtonProps) -> Element {
    let mut count_resource = props.count_resource;

    rsx! {
        button {
            class: "cigarette-button",
            onclick: move |_| {
                storage::add_smoking_event();
                count_resource.restart();
            },
            Cigarette {}
        }
    }
}
