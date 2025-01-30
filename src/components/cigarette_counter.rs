use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CigaretteCounterProps {
    count: i32,
}

#[component]
pub fn CigaretteCounter(props: CigaretteCounterProps) -> Element {
    rsx! {
        div { class: "cigarette-counter-container",
            p { class: "counter-label", "Cigarettes smoked:" }
            p { class: "counter-value", "{props.count}" }
        }
    }
}