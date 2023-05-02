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
    let token = Some("".to_string());
    // let token: Option<String> = None;

    match token {
        Some(_) => {}
        None => {
            return html! {
                <Redirect<Route> to={Route::Login}/>
            }
        }
    };

    html! {
        <>
            <div class="home">
                <div class="dashboard">
                    <nav>
                        <img class="ham" alt="Overview" src="https://upload.wikimedia.org/wikipedia/commons/b/b2/Hamburger_icon.svg"/>
                        <img class="pfp" alt="Profile" src="https://avatars.githubusercontent.com/u/49110391?v=4"/>
                    </nav>
                    <h4>{ "Courses" }</h4>
                    <div class="courses">
                        <div id="sdd">
                            <div></div>
                            <h3>{ "Sofware Design Development" }</h3>
                        </div>
                        <div id="ipt">
                            <div></div>
                            <h3>{ "Information Processes Technology" }</h3>
                        </div>
                        <div id="coc">
                            <div></div>
                            <h3>{ "Coding Club" }</h3>
                        </div>
                        <div id="ist">
                            <div></div>
                            <h3>{ "Information Software Technology" }</h3>
                        </div>
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
            <div class="notfound">
                <h1>{ "The page you were looking for could not be found :(" }</h1>
            </div>
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
