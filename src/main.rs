use dioxus::prelude::*;
mod home;
mod helper;
use home::Home;
use helper::{init_db, check_if_data_exists, insert_initial_data};
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let mut status = use_signal(|| "Initializing...".to_string());
    use_future(move || async move {
        status.set("Creating database...".to_string());
        match init_db().await {
            Ok(_) => {
                status.set("Database created successfully".to_string());
            }
            Err(e) => {
                status.set(format!("Error creating database: {:?}", e));
                return;
            }
        }
        status.set("Checking for existing data...".to_string());
        match check_if_data_exists().await {
            Ok(true) => {
                status.set("No data found".to_string());
                match insert_initial_data().await {
                    Ok(_) => {
                        status.set("Initial data inserted successfully".to_string());
                    }
                    Err(e) => {
                        status.set(format!("Error inserting initial data: {:?}", e));
                    }
                }
            }
            Ok(false) => {
                status.set("Database already contains data".to_string());
            }
            Err(e) => {
                status.set(format!("Error checking data: {:?}", e));
            }
        }
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { class: "min-h-screen bg-base-200",
            div { class: "container mx-auto px-4 py-8",
                div { class: "alert alert-info text-sm py-2 mb-2", "{status}" }
                Router::<Route> {}
            }
        }
    }
}
/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
