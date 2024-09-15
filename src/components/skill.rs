use leptos::{component, view, Children, IntoView};

#[component]
pub fn Skill(name: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="group skillIcon">
            <span class="skillTooltip">" " {name}</span>
            {children()}
        </div>
    }
}
