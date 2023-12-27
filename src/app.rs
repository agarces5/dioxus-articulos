use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::infrastructure::ui::routes::Route;

pub fn app(cx: Scope) -> Element {
    render! { Router::<Route> {} }
}
