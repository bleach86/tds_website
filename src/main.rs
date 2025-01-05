use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const HERO_VIDEO_AV1: Asset = asset!("/assets/tds_promo.webm");
const HERO_VIDEO_VP9: Asset = asset!("/assets/tds_promo_vp9.webm");
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
                    source { r#type: "video/webm; codecs=vp9", src: HERO_VIDEO_VP9 }
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
                h3 { class: "font-light text-5xl", "The Ultimate 2D Arena Shooter" }
                h3 { class: "font-light text-3xl", "Coming spring 2025" }
                SocialLinksContainer {}
            }
        }
    }
}

#[component]
pub fn DetailCard(title_text: String, hook: String) -> Element {
    rsx! {
        div { class: "max-w-lg p-6 rounded-lg shadow bg-gray-800 border-gray-700",
            AwardIconSVG {}
            a { href: "#",
                h5 { class: "mb-2 text-5xl font-semibold text-white", "{title_text}" }
            }
            p { class: "mb-3 text-3xl text-gray-500", "{hook}" }
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

#[component]
pub fn AwardIconSVG() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "32",
            height: "32",
            style: "fill: #fff;",
            shape_rendering: "geometricPrecision",
            text_rendering: "geometricPrecision",
            image_rendering: "optimizeQuality",
            fill_rule: "evenodd",
            clip_rule: "evenodd",
            view_box: "0 0 512 500.59",
            path {
                fill_rule: "nonzero",
                d: "m107.22 313.44-23.58 55.2a7.338 7.338 0 0 1-6.57 4.47l-59.32 5.33 45.22 39.48c2.1 1.84 2.9 4.62 2.32 7.17l-13.36 58.52 51.51-30.8c2.4-1.44 5.3-1.34 7.54 0l51.53 30.8-5.87-25.66c4.88.55 9.91.19 14.8-1.15l6.68 29.31c.19 1.27.19 2.58-.02 3.89a12.8 12.8 0 0 1-1.62 4.47 12.559 12.559 0 0 1-7.71 5.73c-3.09.78-6.5.37-9.49-1.41l-52.07-31.13-51.62 30.88c-1.12.73-2.38 1.29-3.73 1.63l-.94.23c-1.44.26-2.99.26-4.55-.04l-.5-.09c-3.34-.78-6.07-2.84-7.78-5.56a12.573 12.573 0 0 1-1.67-9.24l13.57-59.39-45.68-39.89c-1.2-1.05-2.22-2.35-2.94-3.78a12.6 12.6 0 0 1-1.29-4.2l-.05-.65c-.22-3.31.91-6.45 2.94-8.87 2.01-2.4 4.9-4.05 8.2-4.42l60.67-5.41 23.78-55.71a12.59 12.59 0 0 1 2.7-4.02c.94-.96 2.06-1.77 3.31-2.4.45-.24.91-.44 1.4-.59 2.96-1.06 6.12-.91 8.86.19 2.75 1.11 5.12 3.19 6.54 6.01l18.82 44.01-5.47 23.93c-.4-.5-.72-1.05-.97-1.64l-23.59-55.2zM40.94 151.78c-5.7-2.4-8.38-8.96-5.98-14.66 2.4-5.7 8.97-8.38 14.67-5.98l46.73 19.8c5.7 2.4 8.38 8.97 5.98 14.66-2.4 5.7-8.97 8.38-14.67 5.98l-46.73-19.8zm47.52-72.84c-4.36-4.39-4.34-11.49.05-15.85 4.39-4.36 11.49-4.34 15.85.05l35.7 36.08c4.36 4.39 4.34 11.49-.05 15.85-4.39 4.36-11.49 4.34-15.85-.05l-35.7-36.08zm71.65-49.34c-2.35-5.72.38-12.27 6.1-14.62 5.73-2.35 12.27.38 14.63 6.1l19.3 46.95c2.35 5.72-.39 12.27-6.11 14.62-5.72 2.35-12.27-.38-14.62-6.1l-19.3-46.95zm302.26 101.54c5.7-2.4 12.27.28 14.67 5.98 2.4 5.7-.28 12.26-5.98 14.66l-46.73 19.8c-5.7 2.4-12.27-.28-14.67-5.98-2.4-5.69.28-12.26 5.98-14.66l46.73-19.8zm-54.73-68c4.36-4.39 11.46-4.41 15.85-.05 4.39 4.36 4.41 11.46.05 15.85l-35.7 36.08c-4.36 4.39-11.46 4.41-15.85.05-4.39-4.36-4.41-11.46-.05-15.85l35.7-36.08zm-76.48-42.06c2.36-5.72 8.9-8.45 14.63-6.1 5.72 2.35 8.45 8.9 6.1 14.62l-19.3 46.95c-2.35 5.72-8.9 8.45-14.62 6.1-5.72-2.35-8.46-8.9-6.11-14.62l19.3-46.95zm-86.75-9.84c0-6.2 5.03-11.24 11.24-11.24 6.2 0 11.24 5.04 11.24 11.24V62c0 6.2-5.04 11.24-11.24 11.24-6.21 0-11.24-5.04-11.24-11.24V11.24zm24.28 124.39 35.96 84.19 91.56 8.2c3.69.37 6.89 2.18 9.11 4.82l.37.49c1.97 2.57 3.05 5.83 2.8 9.26l-.04.51c-.18 1.7-.68 3.29-1.41 4.73l-.21.38c-.66 1.19-1.5 2.29-2.47 3.23L334.7 312.3l20.52 89.82c.73 3.57-.01 7.12-1.83 10.01-4.06 6.5-12.73 8.33-19.08 4.18L256 369.48l-78.96 47.2a13.632 13.632 0 0 1-10.17 1.44c-7.27-1.83-11.64-9.11-10.01-16.33l20.44-89.49-69.14-60.38a13.692 13.692 0 0 1-4.65-9.4c-.23-3.46.87-7.05 3.36-9.89 1.25-1.42 2.72-2.53 4.3-3.3 1.63-.8 3.43-1.26 5.25-1.36l90.93-8.15 36.05-84.42c1.48-3.47 4.25-6.04 7.5-7.34 7.02-2.83 14.87.59 17.74 7.44l.05.13zM416.44 303.3l23.72 55.56 60.67 5.41c3.3.37 6.19 2.03 8.2 4.42 2.03 2.42 3.16 5.56 2.94 8.87l-.05.65a12.6 12.6 0 0 1-1.29 4.2c-.72 1.43-1.73 2.72-2.94 3.78l-45.68 39.89 13.57 59.39c.68 3.29 0 6.58-1.67 9.24-1.7 2.71-4.44 4.78-7.78 5.56l-.5.09c-1.56.3-3.1.3-4.55.04l-.94-.23c-1.35-.34-2.61-.89-3.73-1.63l-51.63-30.88-52.06 31.13a12.513 12.513 0 0 1-9.49 1.41c-3.1-.78-5.93-2.76-7.71-5.73-.84-1.41-1.38-2.93-1.63-4.45l-.01-.08c-.21-1.25-.2-2.54 0-3.81l6.69-29.36c1.09.3 2.21.55 3.33.75 3.75.68 7.62.84 11.46.41l-5.87 25.68 51.52-30.8c2.25-1.34 5.14-1.44 7.54 0l51.52 30.8-13.37-58.52c-.57-2.55.23-5.33 2.33-7.17l45.22-39.48-59.32-5.33a7.338 7.338 0 0 1-6.57-4.47l-23.58-55.2-23.57 55.2c-.26.61-.61 1.18-1.02 1.69l-5.46-23.91 18.84-44.08c1.42-2.82 3.79-4.9 6.54-6.01 2.75-1.1 5.91-1.25 8.87-.19.49.16.96.36 1.4.6 1.26.63 2.36 1.44 3.3 2.39 1.16 1.17 2.08 2.55 2.7 4.02l.06.15z",
            }
        }
    }
}
