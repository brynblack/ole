use crate::course::Course;
use crate::home::Home;
use crate::login::Login;
use crate::logout::Logout;
use common::{AccountData, AuthRequest};
use lesson::Lesson;
use yew::prelude::*;
use yew_router::prelude::*;

use serde::Deserialize;

mod course;
mod home;
mod lesson;
mod login;
mod logout;

pub const BASE_URL: &str = "https://localhost:8081";

pub fn run() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/courses/:id")]
    Course { id: String },
    #[at("/lessons/:id")]
    Lesson { id: String },
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
struct CardProps {
    id: String,
    name: String,
    img: String,
}

fn auth(user_ctx: &UseStateHandle<UserInfo>, navigator: &Navigator) {
    match &user_ctx.token {
        Some(token) => {
            let token = token.clone();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let res = reqwest::Client::new()
                    .post(format!("{BASE_URL}/api/auth"))
                    .json(&AuthRequest {
                        token: token.to_string(),
                    })
                    .send()
                    .await
                    .unwrap()
                    .status()
                    .is_success();

                if !res {
                    navigator.push(&Route::Logout);
                }
            });
        }
        None => {
            navigator.push(&Route::Logout);
        }
    }
}

#[function_component(CourseCard)]
fn course_card(props: &CardProps) -> Html {
    html! {
        <Link<Route> to={Route::Course { id: props.id.clone() }}>
            <div style={ format!("background-image: url({});", props.img) }>
                <div></div>
                <h3>{ props.name.clone() }</h3>
            </div>
        </Link<Route>>
    }
}

#[function_component(LessonCard)]
fn lesson_card(props: &CardProps) -> Html {
    html! {
        <Link<Route> to={Route::Lesson { id: props.id.clone() }}>
            <div style={ format!("background-image: url({});", props.img) }>
                <div></div>
                <h3>{ props.name.clone() }</h3>
            </div>
        </Link<Route>>
    }
}

#[derive(Properties, PartialEq)]
struct NavProps {
    title: String,
}

#[function_component(NavBar)]
fn navbar(props: &NavProps) -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    html! {
        <nav>
            <div>
                <img class="ham" alt="Overview" src="https://upload.wikimedia.org/wikipedia/commons/b/b2/Hamburger_icon.svg" />
                <h1>{ &props.title }</h1>
            </div>
            <Link<Route> to={Route::Logout} classes="pfp">
                <img alt="Profile" src={user_ctx.data.clone().unwrap_or_default().pfp} />
            </Link<Route>>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct CourseProps {
    id: String,
}

#[derive(Properties, PartialEq)]
pub struct LessonProps {
    id: String,
}

#[derive(Deserialize)]
struct Info {
    token: String,
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <div class="notfound-page">
            <h1>{ "404: The page you were looking for could not be found :(" }</h1>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Course { id } => html! { <Course id={ id } /> },
        Route::Lesson { id } => html! { <Lesson id={ id } /> },
        Route::Login => html! { <Login /> },
        Route::Logout => html! { <Logout /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[derive(Default, PartialEq)]
struct UserInfo {
    token: Option<String>,
    data: Option<AccountData>,
}

#[function_component(Loading)]
fn loading() -> Html {
    html! {
        <div class="loading-page">
            <h1>{ "Loading..." }</h1>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let user_ctx = use_state(UserInfo::default);

    html! {
        <BrowserRouter>
            <Suspense fallback={ html! { <Loading /> } }>
                <ContextProvider<UseStateHandle<UserInfo>> context={ user_ctx }>
                    <Switch<Route> render={ switch } />
                </ContextProvider<UseStateHandle<UserInfo>>>
            </Suspense>
        </BrowserRouter>
    }
}
