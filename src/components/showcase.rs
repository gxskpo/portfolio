use crate::components::Project;
use crate::icons;
use crate::icons::{Briefcase, FileCode, PullRequest};
use leptos::{component, create_memo, create_signal, view, Children, IntoView, ReadSignal};

#[component]
pub fn Page(no: i32, active_page: ReadSignal<i32>, children: Children) -> impl IntoView {
    let class = create_memo(move |_| {
        if no == active_page() {
            "projectsContainer activePage"
        } else {
            "projectsContainer"
        }
    });

    view! {
        <div class=class id=no>
            {children()}
        </div>
    }
}

#[component]
pub fn Showcase() -> impl IntoView {
    let (page, set_page) = create_signal(1);

    view! {
        <section class="projectSection">
            <div class="projectSectionSelector">
                <button
                    class=move || {
                        if page() == 1 { "projectsTitle active" } else { "projectsTitle" }
                    }
                    on:click=move |_| set_page(1)
                >
                    <span class="flex flex-row justify-center items-center gap-1">
                        <FileCode />
                        Projects
                    </span>
                </button>
                <button
                    class=move || {
                        if page() == 2 { "projectsTitle active" } else { "projectsTitle" }
                    }
                    on:click=move |_| set_page(2)
                >
                    <span class="flex flex-row justify-center items-center gap-1">
                        <PullRequest />
                        Contributions
                    </span>
                </button>
                <button
                    class=move || {
                        if page() == 3 { "projectsTitle active" } else { "projectsTitle" }
                    }
                    on:click=move |_| set_page(3)
                >
                    <span class="flex flex-row justify-center items-center gap-1">
                        <Briefcase />
                        Work
                    </span>
                </button>
            </div>
            <Page no=1 active_page=page>
                <p>"Nothing to see here yet."</p>
            </Page>
            <Page no=2 active_page=page>
                <Project
                    author=Some("RustLangES")
                    name=".NET - Rust example"
                    description="Example of interoperability between dotnet and Rust"
                    url="https://github.com/gxskpo/dotnet-rust-example/"
                    icon_url="https://avatars.githubusercontent.com/u/74681819?"
                >
                    <icons::NET />
                    <icons::CSharp />
                    <icons::Rust />
                </Project>
                <Project
                    author=Some("RustLangES")
                    name="Resume"
                    description="RustLangES web redesign, built with Astro and tailwind"
                    url="https://github.com/RustLangES/resume"
                    icon_url="https://avatars.githubusercontent.com/u/74681819?"
                >
                    <icons::Astro />
                </Project>
            </Page>
            <Page no=3 active_page=page>
                <article class="project">
                    <span class="projectInfo">
                        <span class="projectText">
                            <h3>Guia Vegana</h3>
                            <p>Backend Developer</p>
                            <p>2025</p>
                        </span>
                    </span>
                    <div class="projectIcons">
                        <div class="projectIcon">
                            <icons::Rust />
                        </div>
                    </div>
                </article>
            </Page>
        </section>
    }
}
