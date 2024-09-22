use leptos::{component, view, Children, CollectView, IntoView};

#[component]
pub fn Project(
    author: Option<&'static str>, // In Contributions only
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

    let author_view = match author {
        Some(author) => view! { <h4>{author}</h4> },
        _ => view! { <h4></h4> },
    };

    view! {
        <a href=url target="_blank">
            <article class="project">
                <span class="projectInfo">
                    <img src=icon_url alt="Project Icon" />
                    <span class="projectText">
                        {author_view} <h3>{name}</h3> <p>{description}</p>
                    </span>
                </span>
                <div class="projectIcons">{icns}</div>
            </article>
        </a>
    }
}
