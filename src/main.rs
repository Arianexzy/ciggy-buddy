use dioxus::prelude::*;

mod components;
mod storage;
mod views;
use views::Home;

// mod synth_data;
// use synth_data::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TIME_PERIOD_CSS: Asset = asset!("/assets/styling/time_period.css");
const CIGARETTE_COUNTER_CSS: Asset = asset!("/assets/styling/cigarette_counter.css");
const CIGARETTE_BUTTON_CSS: Asset = asset!("/assets/styling/cigarette_button.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TIME_PERIOD_CSS }
        document::Link { rel: "stylesheet", href: CIGARETTE_COUNTER_CSS }
        document::Link { rel: "stylesheet", href: CIGARETTE_BUTTON_CSS }

        Home {}
    }
}
