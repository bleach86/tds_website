use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const HERO_VIDEO_AV1: Asset = asset!("/assets/tds_promo.webm");
const HERO_VIDEO_H265: Asset = asset!("/assets/tds_promo_h265.mp4");
const HERO_VIDEO_H264: Asset = asset!("/assets/tds_promo_h264.mp4");
const HERO_POSTER: Asset = asset!("/assets/tds_promo_screen.png");

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/fork-awesome/1.2.0/css/fork-awesome.min.css",
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: "/assets/apple-touch-icon.png",
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "32x32",
            href: "/assets/favicon-32x32.png",
        }
        document::Link {
            rel: "icon",
            r#type: "image/png",
            sizes: "16x16",
            href: "/assets/favicon-16x16.png",
        }
        document::Link { rel: "manifest", href: "/assets/site.webmanifest" }
        Hero {}
        DetailSection {}
        Footer {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "relative h-screen flex flex-col items-center justify-center text-center text-white py-0 px-3",
            div { class: "video-docker absolute top-0 left-0 w-full h-full overflow-hidden",
                video {
                    class: "min-w-full min-h-full absolute object-cover",
                    autoplay: "true",
                    muted: "true",
                    r#loop: "true",
                    playsinline: "true",
                    preload: "auto",
                    poster: HERO_POSTER,
                    source { r#type: "video/webm; codecs=av1", src: HERO_VIDEO_AV1 }
                    source {
                        r#type: "video/mp4; codecs=hvc1.1.6.L93.B0",
                        src: HERO_VIDEO_H265,
                    }
                    source {
                        r#type: "video/mp4; codecs=avc1.640028",
                        src: HERO_VIDEO_H264,
                    }
                }
            }
            div { class: "video-content space-y-2",
                h1 { class: "font-light text-7xl sm:text-9xl", "TDS: Delta" }
                h3 { class: "font-light text-5xl", "The ultimate 2d arena shooter" }
                h3 { class: "font-light text-3xl", "Coming spring 2025" }
                SocialLinksContainer {}
            }
        }
    }
}

#[component]
pub fn DetailCard(title_text: String, hook: String) -> Element {
    rsx! {
        div { class: "max-w-lg p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700",
            i { class: "fa fa-gift fa-2x", aria_hidden: "true" }
            a { href: "#",
                h5 { class: "mb-2 text-5xl font-semibold text-gray-900 dark:text-white",
                    "{title_text}"
                }
            }
            p { class: "mb-3 text-3xl text-gray-500 dark:text-gray-400", "{hook}" }
        }
    }
}

#[component]
pub fn DetailSection() -> Element {
    rsx! {
        section { class: "space-y-8 py-8 px-4 max-w-5xl mx-auto",
            div { class: "flex flex-col space-y-8 md:space-y-0 md:flex-row items-center",
                DetailCard {
                    title_text: "Multiplayer",
                    hook: "TDS: Delta supports up to 16 players in a single match.",
                }
            }
            div { class: "flex flex-col space-y-8 md:space-y-0 md:flex-row-reverse items-center",
                DetailCard {
                    title_text: "Cross Platform",
                    hook: "TDS: Delta will be available on Windows, macOS, and Linux.",
                }
            }
            div { class: "flex flex-col space-y-8 md:space-y-0 md:flex-row items-center",
                DetailCard {
                    title_text: "Multiple Game Modes",
                    hook: "TDS: Delta features multiple game modes, including Capture the Flag, Deathmatch, Zombies,and more.",
                }
            }
            div { class: "flex flex-col space-y-8 md:space-y-0 md:flex-row-reverse items-center",
                DetailCard {
                    title_text: "Fast Paced",
                    hook: "TDS: Delta features fast paced Arena Shooter gameplay.",
                }
            }
        }
    }
}

#[component]
pub fn SocialLink(href: String, icon: String) -> Element {
    rsx! {
        a {
            href,
            class: "text-white hover:text-gray-200",
            target: "_blank",
            rel: "noopener noreferrer",
            i { class: format!("fab fa-2x {}", icon) }
        }
    }
}

#[component]
pub fn SocialLinksContainer() -> Element {
    rsx! {
        div {
            class: "flex space-x-8 max-w-xs mx-auto justify-center",
            style: "margin-top: 4rem;",
            SocialLink { href: "https://discord.gg/kV7csu8sDH", icon: "fa fa-discord" }
            SocialLink {
                href: "https://matrix.to/#/#TDSD:matrix.org",
                icon: "fa fa-matrix-org",
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    let current_year = {
        let date_now = web_sys::js_sys::Date::new_0();
        let year = date_now.get_full_year();
        year
    };

    rsx! {
        div { class: "absolute left-0 w-full text-white py-4",
            p { class: "text-center", "V{VERSION} - \u{00a9} {current_year} tuxprint" }
        }
    }
}
