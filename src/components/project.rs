use leptos::{component, view, Children, CollectView, IntoView};

#[component]
pub fn Project(
    name: &'static str,
    description: &'static str,
    url: &'static str,
    icon_url: &'static str,
    children: Children,
) -> impl IntoView {
    let icns = children()
        .nodes
        .into_iter()
        .map(|child| view! { <div class="projectIcon">{child}</div> })
        .collect_view();

    view! {
        <a href=url target="_blank">
            <article class="project">
                <span class="projectInfo">
                    <img src=icon_url alt="Project Icon" />
                    <span className="projectText">
                        <h3>{name}</h3>
                        <p>{description}</p>
                    </span>
                </span>
                <div class="projectIcons">{icns}</div>
            </article>
        </a>
    }
}
