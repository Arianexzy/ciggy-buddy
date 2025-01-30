use crate::components::*;
// use crate::database::*;
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    let time_period_selected = use_signal(|| "total".to_string());
    
    let mut count = 0;
    
    rsx! {
        div { class: "app-container",
            h1 { class: "app-title", "Ciggy Buddy" }
            h2 { class: "app-subtitle", "Your Ciggy Counting Friend" }
            TimePeriod { time_period_selected }
            CigaretteCounter { count }
            CigaretteButton {}
            
        }
    }
}
