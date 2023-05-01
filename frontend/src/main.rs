use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
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
fn home() -> Html {
    html! {
        <>
            <div class="home">
                <div class="dashboard">
                    <span>{ "Welcome back!" }</span>
                    <div class="courses">
                        <span id="sdd">{ "Sofware Design Development" }</span>
                        <span id="ipt">{ "Information Processes Technology" }</span>
                        <span id="coc">{ "Coding Club" }</span>
                        <span id="ist">{ "Information Software Technology" }</span>
                    </div>
                </div>
            </div>
        </>
    }
}

#[function_component(Login)]
fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let onsubmit = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <>
            <div class="login">
                <form class="login-box" onsubmit={ onsubmit }>
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
            </div>
        </>
    }
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! {
        <>
            <h1>{ "The page you were looking for could not be found :(" }</h1>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { <Login/> },
        Route::NotFound => html! { <NotFound/> },
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
