pub mod home;
pub mod login;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::infrastructure::ui::routes::home::Home;
use crate::infrastructure::ui::routes::login::Login;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
}
