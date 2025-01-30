use dioxus::prelude::*;
use lucide_dioxus::{Calendar, Clock, Layers, Moon, Sun};


struct Period {
    id: &'static str,
    label: &'static str,
}

const PERIODS: [Period; 5] = [
    Period {
        id: "total",
        label: "Total",
    },
    Period {
        id: "today",
        label: "Today",
    },
    Period {
        id: "week",
        label: "This Week",
    },
    Period {
        id: "month",
        label: "This Month",
    },
    Period {
        id: "year",
        label: "This Year",
    },
];

#[derive(PartialEq, Props, Clone)]
pub struct TimePeriodProps {
    time_period_selected: Signal<String>,
}

#[component]
pub fn TimePeriod(props: TimePeriodProps) -> Element {
    let mut time_period_selected = props.time_period_selected;
    
    rsx! {
        div { class: "time-period-container",
            {
                PERIODS
                    .iter()
                    .map(|period| {
                        let is_selected = time_period_selected() == period.id;
                        let button_class = if is_selected {
                            "time-period-button selected"
                        } else {
                            "time-period-button"
                        };
                        rsx! {
                            button {
                                class: button_class,
                                key: "{period.id}",
                                onclick: move |_| async move {
                                    time_period_selected.set(period.id.to_string());
                                },
                                div { class: "time-period-button-icon", {render_icon(period.id)} }
                                span { class: "time-period-button-label", "{period.label}" }
                            }
                        }
                    })
            }
        }
    }
}

fn render_icon(period_id: &str) -> Element {
    match period_id {
        "total" => rsx! {
            Layers {}
        },
        "today" => rsx! {
            Sun {}
        },
        "week" => rsx! {
            Moon {}
        },
        "month" => rsx! {
            Clock {}
        },
        "year" => rsx! {
            Calendar {}
        },
        _ => rsx! {
            Layers {}
        },
    }
}