use super::homepage::Homepage;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Routeeth {
    #[route("/")]
    Homepage {},
}
