#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;
use ybc::NavbarFixed::Top;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::Four;
use yew::prelude::*;
struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _: Self::Message) -> bool {
    false
  }

  fn change(&mut self, _: Self::Properties) -> bool {
    false
  }

  fn view(&self) -> Html {
    html! {
        <>
        <ybc::Navbar fixed=Top
            classes="is-transparent is-black"
            padded=true
            navbrand=html!{
                <ybc::NavbarItem>
                    <ybc::Title classes="has-text-white" size=ybc::HeaderSize::Is4>{"Greetings Stranger"}</ybc::Title>
                </ybc::NavbarItem>
            }
            navstart=html!{}
            navend=html!{
                <>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes="is-danger is-outlined" href="00/">
                        {"Posts"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes="is-danger is-outlined" href="scratch/">
                        {"Scratch"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes="is-danger is-outlined" href="https://rwest.io">
                        {"Home"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                </>
            }
        />

        <ybc::Hero
            classes="is-dark"
            size=ybc::HeroSize::Medium
            body=html!{
                <ybc::Container classes="is-centered">
                <ybc::Tile ctx=Ancestor>
                    <ybc::Tile ctx=Parent size=ybc::TileSize::Six>
                        <ybc::Tile ctx=Parent>
                            <ybc::Tile ctx=Child classes="notification is-black">
                                <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"Trunk"}</ybc::Subtitle>
                                <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx=Parent>
                            <ybc::Tile ctx=Child classes="notification is-black">
                                <ybc::Icon size=ybc::Size::Large classes="is-pulled-right"><img src="yew.svg"/></ybc::Icon>
                                <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">
                                    {"Yew"}
                                </ybc::Subtitle>
                                <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx=Parent>
                            <ybc::Tile ctx=Child classes="notification is-black">
                                <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"YBC"}</ybc::Subtitle>
                                <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                </ybc::Tile>
                </ybc::Container>
            }>
        </ybc::Hero>

        <ybc::Container fluid=true>
          <ybc::Tile ctx=Ancestor>
            <ybc::Tile ctx=Parent vertical=true size=Four>
              <ybc::Tile ctx=Child classes="box">
                <p>{"Lorem ipsum dolor sit amet ..."}</p>
              </ybc::Tile>
            </ybc::Tile>
          </ybc::Tile>
        </ybc::Container>
        </>
    }
  }
}

fn main() {
  set_panic_hook();
  yew::start_app::<App>();
}
