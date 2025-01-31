use crate::components::*;
use crate::storage::storage;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {    
    let time_period_selected = use_signal(|| "total".to_string());

    let count_resource = use_resource(move || {
        let period = time_period_selected.read().to_string();
        async move {
            match period.as_str() {
                "total" => storage::get_total_count(),
                "today" => storage::get_today_count(),
                "week" => storage::get_week_count(),
                "month" => storage::get_month_count(),
                "year" => storage::get_year_count(),
                _ => storage::get_total_count(),
            }
        }
    });

    let count = match &*count_resource.read_unchecked() {
        Some(count) => *count,
        _ => 0,
    };    
    
    rsx! {
        div { class: "app-container",
            h1 { class: "app-title", "Ciggy Buddy" }
            h2 { class: "app-subtitle", "Your Ciggy Counting Friend" }
            TimePeriod { time_period_selected }
            CigaretteCounter { count }
            CigaretteButton { count_resource: count_resource }
        }
    }
}
