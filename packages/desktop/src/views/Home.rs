use dioxus::prelude::*;

#[component]
pub fn ProjectCard() -> Element {
    rsx! {
        div { class: "ProjectCard" }
    }
}

#[component]
pub fn ProjectsDiv() -> Element {
    rsx! {
        div { class: "Projects" }
    }
}

#[component]
pub fn Search() -> Element {
    rsx! {
        div { class: "SearchArea",
            input { class: "SearchBox", r#type: "Text" }
        }
    }
}



#[component]
pub fn Home() -> Element {
    rsx! {
        Search {}
        ProjectsDiv {}
    }
}
