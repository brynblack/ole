use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(PartialEq, Properties)]
struct Properties {
    token: Option<String>,
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home(props: &Properties) -> Html {
    let token = match props.token.clone() {
        Some(token) => token,
        None => {
            return html! {
                <Redirect<Route> to={Route::Login}/>
            }
        }
    };

    html! {
        <>
            <div>{ token }</div>
        </>
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
        <>
            <form class="login-box" action="https://localhost:8081/api/auth" method="post">
                <span>{ "Sign In" }</span>
                <div class="entry">
                    <label for="username">{ "Username" }</label>
                    <input id="username" name="username" type="text"/>
                </div>
                <div class="entry">
                    <label for="password">{ "Password" }</label>
                    <input id="password" name="password" type="password"/>
                    <a href="">{ "Forgot Password?" }</a>
                </div>
                <button class="submit-button">{ "Sign In" }</button>
            </form>
        </>
    }
}

fn switch(routes: Route) -> Html {
    let token = Some("meow".to_string());
    // let token: Option<String> = None;
    match routes {
        Route::Home => html! { <Home token={ token }/> },
        Route::Login => html! { <Login/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
