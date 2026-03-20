use dioxus::prelude::*;
use std::{collections::HashSet, slice::Iter};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug)]
pub enum State {
    Six([String; 12]),
    Five([String; 10]),
    Four([String; 8]),
    Three([String; 6]),
}

impl State {
    pub const fn size(&self) -> usize {
        match self {
            State::Six(_) => 6,
            State::Five(_) => 5,
            State::Four(_) => 4,
            State::Three(_) => 3,
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut String {
        match self {
            State::Six(b) => &mut b[idx],
            State::Five(b) => &mut b[idx],
            State::Four(b) => &mut b[idx],
            State::Three(b) => &mut b[idx],
        }
    }

    pub fn get(&self, idx: usize) -> &str {
        match self {
            State::Six(b) => &b[idx],
            State::Five(b) => &b[idx],
            State::Four(b) => &b[idx],
            State::Three(b) => &b[idx],
        }
    }

    pub fn iter(&self) -> Iter<'_, String> {
        match self {
            State::Six(b) => b.iter(),
            State::Five(b) => b.iter(),
            State::Four(b) => b.iter(),
            State::Three(b) => b.iter(),
        }
    }
}

#[component]
pub fn Home() -> Element {
    let mut duplicate_heros = use_signal(|| false);
    let mut submitted = use_signal(|| false);
    let mut state = use_signal(|| State::Three([const { String::new() }; 6]));
    let mut gen_results = use_signal(|| Vec::new());
    rsx! {
        div { id: "home",
            h1 { "Deadlock Team Generator" }
            div { id: "border",
                div { id: "team-size",
                    label { "Team Size" }
                    select {
                        oninput: move |evt: Event<FormData>| {
                            let val: u8 = evt.value().parse().expect("");
                            let ns = match val {
                                3 => State::Three([const { String::new() }; 6]),
                                4 => State::Four([const { String::new() }; 8]),
                                5 => State::Five([const { String::new() }; 10]),
                                6 => State::Six([const { String::new() }; 12]),
                                _ => unreachable!(),
                            };
                            state.set(ns);
                        },
                        option { label: "3", value: "3" }
                        option { label: "4", value: "4" }
                        option { label: "5", value: "5" }
                        option { label: "6", value: "6" }
                    }
                }
                div { id: "checkbox",
                    label { "Duplicate Heroes?" }
                    input {
                        oninput: move |e| { duplicate_heros.set(e.checked()) },
                        r#type: "checkbox",
                    }
                }
                div { id: "player-name-label",
                    label { "Deadlock Player Names" }
                    div { id: "names",
                        for i in 0..state.read().size() * 2 {
                            div {
                                input {
                                    r#type: "text",
                                    value: state.read().get(i),
                                    oninput: move |event| {
                                        *state.write().get_mut(i) = event.value();
                                        debug!("{:?}", state);
                                    },
                                }
                            }
                        }
                    }
                }
                div { id: "buttons",
                    button {
                        onclick: move |_| {
                            let cleared = match *state.read() {
                                State::Six(_) => State::Six([const { String::new() }; 12]),
                                State::Five(_) => State::Five([const { String::new() }; 10]),
                                State::Four(_) => State::Four([const { String::new() }; 8]),
                                State::Three(_) => State::Three([const { String::new() }; 6]),
                            };
                            gen_results.set(Vec::new());
                            state.set(cleared);
                            submitted.set(false);
                            debug!("{:?}", state);
                        },
                        if !submitted() {
                            "Clear"
                        } else {
                            "Reset"
                        }
                    }
                    if !submitted() {
                        button {
                            onclick: move |_| {
                                submitted.set(true);

                                if duplicate_heros() {
                                    gen_results.set(gen_heroes_with_repeat(state.read().iter()));
                                } else {
                                    gen_results.set(gen_heroes(state.read().iter()));
                                }
                            },
                            "Generate"
                        }
                    } else {
                        button {
                            onclick: move |_| {
                                if duplicate_heros() {
                                    gen_results.set(gen_heroes_with_repeat(state.read().iter()));
                                } else {
                                    gen_results.set(gen_heroes(state.read().iter()));
                                }
                            },
                            "Regenerate"
                        }
                    }
                }
            }
            if submitted() {
                div { id: "border",
                    div {
                        ul {
                            for i in 0..gen_results.read().len() {
                                li {
                                    {
                                        let (hero, name) = &gen_results.read()[i];
                                        format!("{name} - {hero}")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[inline(always)]
pub fn random_hero() -> Result<&'static str, String> {
    let mut buf = [0u8; 1];
    getrandom::fill(&mut buf).map_err(|e| e.to_string())?;
    Ok(DEADLOCK_HEROES[(buf[0] % 37u8) as usize])
}

pub fn gen_heroes(names: Iter<'_, String>) -> Vec<(&'static str, String)> {
    let mut hs = HashSet::new();
    for _ in 0..names.len() {
        loop {
            let random = random_hero().expect("");
            if !hs.insert(random) {
                continue;
            }
            break;
        }
    }

    hs.into_iter().zip(names.cloned().into_iter()).collect()
}

pub fn gen_heroes_with_repeat(names: Iter<String>) -> Vec<(&'static str, String)> {
    (0..names.len() as usize)
        .into_iter()
        .zip(names.cloned().into_iter())
        .map(|i| (random_hero().expect(""), i.1))
        .collect()
}

pub static DEADLOCK_HEROES: &[&'static str] = &[
    "Abrams",
    "Apollo",
    "Bebop",
    "Billy",
    "Calico",
    "Celeste",
    "Doorman",
    "Drifter",
    "Dynamo",
    "Graves",
    "Grey Talon",
    "Haze",
    "Holliday",
    "Infernus",
    "Ivy",
    "Kelvin",
    "Lady Geist",
    "Lash",
    "McGinnis",
    "Mina",
    "Mirage",
    "Mo & Krill",
    "Paige",
    "Paradox",
    "Pocket",
    "Rem",
    "Seven",
    "Shiv",
    "Silver",
    "Sinclair",
    "Venator",
    "Victor",
    "Vindicta",
    "Viscous",
    "Vyper",
    "Warden",
    "Wraith",
    "Yamato",
];
