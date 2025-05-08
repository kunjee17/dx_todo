use dioxus::prelude::*;
use crate::helper::{get_people, delete_all_people, insert_initial_data, sync_db};
/// Home page
#[component]
pub fn Home() -> Element {
    let mut people = use_resource(move || async move { get_people().await });
    let mut reset_status = use_signal(|| "".to_string());
    let reset_data = move |_| {
        spawn(async move {
            if let Err(e) = delete_all_people().await {
                reset_status.set(format!("Error deleting data: {:?}", e));
                return;
            }
            if let Err(e) = insert_initial_data().await {
                reset_status.set(format!("Error inserting data: {:?}", e));
                return;
            }
            reset_status.set("Data reset successfully!".to_string());
            people.restart();
        });
    };
    let sync_data = move |_| {
        spawn(async move {
            if let Err(e) = sync_db().await {
                reset_status.set(format!("Error syncing data: {:?}", e));
                return;
            }
            reset_status.set("Data synced successfully!".to_string());
            people.restart();
        });
    };
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-2xl font-bold", "Person List" }
                div { class: "flex gap-2",
                    button { class: "btn btn-primary", onclick: sync_data, "Sync Data" }
                    button { class: "btn btn-primary", onclick: reset_data, "Reset Data" }
                }
            }
            if !reset_status().is_empty() {
                div { class: "alert alert-info text-sm py-2 mb-4", "{reset_status}" }
            }
            div { class: "overflow-x-auto",
                table { class: "table table-zebra w-full",
                    thead {
                        tr {
                            th { "ID" }
                            th { "First Name" }
                            th { "Last Name" }
                        }
                    }
                    tbody {
                        match &*people.read_unchecked() {
                            Some(Ok(people)) => rsx! {
                                for person in people {
                                    tr {
                                        td { "{person.id}" }
                                        td { "{person.first_name}" }
                                        td { "{person.last_name}" }
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                tr {
                                    td { colspan: "3", class: "text-center text-red-500", "Error loading data: {e}" }
                                }
                            },
                            None => rsx! {
                                tr {
                                    td { colspan: "3", class: "text-center", "Loading..." }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
