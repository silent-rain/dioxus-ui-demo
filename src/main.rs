#![allow(non_snake_case)]

use demo::div::Div;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use demo::core::{ComponentTrait, Wrap};
use demo::{button::Button, card::Card, link::Link};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/ui-demo/")]
    UiDemo {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            style: "basis-0 bg-sky-400 bg-sky-400;",
            "Go to blog"
        }
        Link {
            to: Route::UiDemo {
            },
            style: "basis-0 bg-sky-400 bg-sky-400;margin-left: 20px;",
            "Go to Ui demo"
        }
        div {
            h1 { class: "basis-0 bg-sky-400 bg-sky-400", "High-Five counter: {count}" }
            Wrap {
                ele: Card::new()
                .style("padding: 30px; display: flex; flex-direction: column;")
                .components(vec![Box::new(Link::new("Link")).build(),
                 Box::new(Link::new("Link"))
                 .style("margin-top: 20px;")
                 .build()
                 ])
            }
            Wrap {
                ele: Button::new("Up high")
                .class("up-high")
                .width(96)
                .height(32)
                .style("color: #fff;")
                .onclick(move |_| {
                    count += 1;
                    println!("Up high! Count is now {}", count);
                })
            }
            Wrap {
                ele: Button::new("Down low")
                .class("down-low")
                .width(96)
                .height(32)
                .style("color: #fff; margin-left: 20px;")
                .onclick(move |_| {
                    count -= 1;
                    println!("Down low! Count is now {}", count);
                })
            }
        }
    }
}

fn UiDemo() -> Element {
    let mut count = use_signal(|| 0);

    Card::new()
        .style("margin: 20px;padding: 30px; display: flex; flex-direction: column; background: antiquewhite;")
        .height(400)
        .components(vec![
            Div::new().text(&format!("count: {}", count)).build(),
            Link::new("Link1").style("margin-top: 20px;").build(),
            Link::new("Link2").style("margin-top: 20px;").build(),
            Button::new("Up high")
                .class("up-high")
                .width(96)
                .height(32)
                .style("color: #fff; margin-top: 20px;")
                .onclick(move |_| {
                    count += 1;
                    println!("Up high! Count is now {}", count);
                })
                .build(),
            Button::new("Down low")
                .class("down-low")
                .width(96)
                .height(32)
                .style("color: #fff; margin-top: 20px;")
                .onclick(move |_| {
                    count -= 1;
                    println!("Down low! Count is now {}", count);
                })
                .build(),
        ])
        .build()
}
