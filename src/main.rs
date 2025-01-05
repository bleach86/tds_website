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
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/fork-awesome/1.2.0/css/fork-awesome.min.css",
        }
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
    let icon_element = match icon.as_str() {
        "discord" => DiscordSVG,
        "matrix" => MatrixSVG,
        _ => DiscordSVG,
    };

    rsx! {
        a {
            href,
            class: "text-white hover:text-gray-200",
            target: "_blank",
            rel: "noopener noreferrer",
            icon_element {}
        }
    }
}

#[component]
pub fn SocialLinksContainer() -> Element {
    rsx! {
        div {
            class: "flex space-x-8 max-w-xs mx-auto justify-center",
            style: "margin-top: 4rem;",
            SocialLink { href: "https://discord.gg/kV7csu8sDH", icon: "discord" }
            SocialLink { href: "https://matrix.to/#/#TDSD:matrix.org", icon: "matrix" }
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

#[component]
pub fn MatrixSVG() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "32",
            height: "32",
            view_box: "0 0 32 32",
            style: "fill: #fff;",
            path { d: "M 30,2.0000001 V 30 h -1 -2 v 2 h 5 V -3.3333334e-8 L 27,0 v 2 z" }
            path {
                d: "M 9.9515939,10.594002 V 12.138 h 0.043994 c 0.3845141,-0.563728 0.8932271,-1.031728 1.4869981,-1.368 0.580003,-0.322998 1.244999,-0.485 1.993002,-0.485 0.72,0 1.376999,0.139993 1.971998,0.42 0.595,0.279004 1.047001,0.771001 1.355002,1.477001 0.338003,-0.500001 0.795999,-0.941 1.376999,-1.323001 0.579999,-0.382998 1.265998,-0.574 2.059998,-0.574 0.602003,0 1.160002,0.074 1.674002,0.220006 0.514,0.148006 0.953998,0.382998 1.321999,0.706998 0.36601,0.322999 0.653001,0.746 0.859,1.268002 0.205001,0.521998 0.307994,1.15 0.307994,1.887001 v 7.632997 h -3.127 v -6.463997 c 0,-0.383002 -0.01512,-0.743002 -0.04399,-1.082003 -0.02079,-0.3072 -0.103219,-0.607113 -0.242003,-0.881998 -0.133153,-0.25081 -0.335962,-0.457777 -0.584001,-0.596002 -0.257008,-0.146003 -0.605998,-0.220006 -1.046997,-0.220006 -0.440002,0 -0.796003,0.085 -1.068,0.253002 -0.272013,0.170003 -0.485001,0.390002 -0.639001,0.662003 -0.159119,0.287282 -0.263585,0.601602 -0.307994,0.926997 -0.05197,0.346923 -0.07801,0.697217 -0.07801,1.048002 v 6.353999 h -3.128005 v -6.398 c 0,-0.338003 -0.0072,-0.673001 -0.02116,-1.004001 -0.01134,-0.313663 -0.07487,-0.623229 -0.187994,-0.915999 -0.107943,-0.276623 -0.300435,-0.512126 -0.550001,-0.673001 -0.25799,-0.168 -0.636,-0.253002 -1.134999,-0.253002 -0.198123,0.0083 -0.394383,0.04195 -0.584002,0.100006 -0.258368,0.07446 -0.498455,0.201827 -0.704999,0.373985 -0.227981,0.183987 -0.421999,0.449 -0.583997,0.794003 -0.161008,0.345978 -0.242003,0.797998 -0.242003,1.356998 v 6.618999 H 6.99942 V 10.590001 Z",
            }
            path { d: "M 2,2.0000001 V 30 h 3 v 2 H 0 V 9.2650922e-8 L 5,0 v 2 z" }
        }
    }
}

#[component]
pub fn DiscordSVG() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            shape_rendering: "geometricPrecision",
            text_rendering: "geometricPrecision",
            image_rendering: "optimizeQuality",
            fill_rule: "evenodd",
            clip_rule: "evenodd",
            view_box: "0 0 512 388.049",
            width: "32",
            height: "32",
            style: "fill: #fff;",
            path {
                fill_rule: "nonzero",
                d: "M433.713 32.491A424.231 424.231 0 00328.061.005c-4.953 8.873-9.488 18.156-13.492 27.509a393.937 393.937 0 00-58.629-4.408c-19.594 0-39.284 1.489-58.637 4.37-3.952-9.33-8.543-18.581-13.525-27.476-36.435 6.212-72.045 17.196-105.676 32.555-66.867 98.92-84.988 195.368-75.928 290.446a425.967 425.967 0 00129.563 65.03c10.447-14.103 19.806-29.116 27.752-44.74a273.827 273.827 0 01-43.716-20.862c3.665-2.658 7.249-5.396 10.712-8.055 40.496 19.019 84.745 28.94 129.514 28.94 44.77 0 89.019-9.921 129.517-28.943 3.504 2.86 7.088 5.598 10.712 8.055a275.576 275.576 0 01-43.796 20.918 311.49 311.49 0 0027.752 44.705 424.235 424.235 0 00129.65-65.019l-.011.011c10.632-110.26-18.162-205.822-76.11-290.55zM170.948 264.529c-25.249 0-46.11-22.914-46.11-51.104 0-28.189 20.135-51.304 46.029-51.304 25.895 0 46.592 23.115 46.15 51.304-.443 28.19-20.336 51.104-46.069 51.104zm170.102 0c-25.29 0-46.069-22.914-46.069-51.104 0-28.189 20.135-51.304 46.069-51.304s46.472 23.115 46.029 51.304c-.443 28.19-20.296 51.104-46.029 51.104z",
            }
        }
    }
}
