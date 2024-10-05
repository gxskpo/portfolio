#![allow(clippy::too_many_lines, clippy::module_name_repetitions)]

mod components;
mod icons;
mod utils;

use leptos::{component, view, IntoView};

#[component]
fn App() -> impl IntoView {
    view! {
        <main class="indexLayout">
            <section class="topbar">
                <button class="active">
                    <icons::Home />
                </button>
                <a
                    href="https://github.com/gxskpo/portfolio"
                    target="_blank"
                    aria-label="Source code"
                >
                    <icons::Code />
                </a>
                <components::ThemeButton />
            </section>
            <section class="info">
                <div class="aboutMe">
                    <span class="avatar">
                        <img
                            alt="avatar"
                            src="https://avatars.githubusercontent.com/u/97119998"
                            width="80"
                            height="80"
                        />
                        <div class="details">
                            <h1>Larissa</h1>
                            <p>16yo backend dev</p>
                        </div>
                    </span>
                    <div class="socialButtons">
                        <a href="https://paypal.me/gxskpo" target="_blank" aria-label="Paypal">
                            <icons::Paypal />
                        </a>
                        <a href="mailto:hello@hawruka.de" target="_blank">
                            <icons::Mail />
                        </a>
                        <a href="https://github.com/gxskpo" target="_blank" aria-label="github">
                            <icons::Github />
                        </a>
                        <a
                            href="https://discordapp.com/users/538821983606145044"
                            target="_blank"
                            aria-label="discord"
                        >
                            <icons::Discord color=Some("var(--text)") />
                        </a>
                        <a
                            href="https://t.me/hawruka_de"
                            target="_blank"
                            aria-label="twitter"
                        >
                            <icons::Telegram />
                        </a>
                    </div>
                </div>
                <div class="description">
                    <h2>About me</h2>
                    <p>
                        "Hi! my name is Larissa, online I go by Haruka, I'm a 16yo mexican
                        girl who likes programming and web backend development."
                    </p>
                </div>
                <div class="skills">
                    <h3>Skills</h3>
                    <div class="skillsIcons">
                        <components::Skill name="C#">
                            <icons::CSharp />
                        </components::Skill>
                        <components::Skill name="Javascript">
                            <icons::JS />
                        </components::Skill>
                        <components::Skill name="Typescript">
                            <icons::TS />
                        </components::Skill>
                        <components::Skill name="Python">
                            <icons::Python />
                        </components::Skill>
                        <span style="background: transparent;"></span>

                        <components::Skill name="React">
                            <icons::React />
                        </components::Skill>
                        <components::Skill name="NextJS">
                            <icons::NextJS />
                        </components::Skill>
                        <components::Skill name="Node.js">
                            <icons::NodeJS />
                        </components::Skill>
                        <components::Skill name=".NET">
                            <icons::NET />
                        </components::Skill>
                        <components::Skill name="Flask">
                            <icons::Flask />
                        </components::Skill>

                        <components::Skill name="SQLite">
                            <icons::SQLite />
                        </components::Skill>
                        <components::Skill name="MySQL">
                            <icons::MySQL />
                        </components::Skill>

                    </div>
                </div>
            </section>
            <components::Showcase />
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App /> });
}
