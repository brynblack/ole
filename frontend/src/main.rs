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

#[derive(Properties, PartialEq)]
struct CardProps {
    id: String,
    name: String,
    img: String,
}

#[function_component(CourseCard)]
fn course_card(props: &CardProps) -> Html {
    html! {
        <div id={ props.id.clone() } style={ "background-image: url(\"".to_owned() + &props.img + "\");" }>
            <div></div>
            <h3>{ props.name.clone() }</h3>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div class="home-page">
            <div class="dashboard">
                <nav>
                    <div>
                        <img class="ham" alt="Overview" src="https://upload.wikimedia.org/wikipedia/commons/b/b2/Hamburger_icon.svg" />
                        <h1>{ "Online Learning Environment" }</h1>
                    </div>
                    <img class="pfp" alt="Profile" src="https://avatars.githubusercontent.com/u/49110391?v=4" />
                </nav>
                <h4>{ "Courses" }</h4>
                <div class="courses">
                    <CourseCard id="sdd" name="Software Design Development" img="https://www.newsmaritime.com/wp-content/uploads/2016/06/Check-out-the-new-and-functional-programming-language.jpg" />
                    <CourseCard id="ipt" name="Information Processes Technology" img="https://www.databankimx.com/wp-content/uploads/2016/01/Database-Services-page-banner.jpg" />
                    <CourseCard id="coc" name="Coding Club" img="https://theaxiom.ca/wp-content/uploads/2015/03/muchbetter.jpg" />
                    <CourseCard id="ist" name="Information Services Technology" img="https://www.myhtptech.com/wp-content/uploads/2020/07/businessIntelligenceservices.jpg" />
                </div>
            </div>
        </div>
    }
}

#[function_component(Login)]
fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let onsubmit = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div class="login-page">
            <form class="login-box" onsubmit={ onsubmit }>
                <h1>{ "Sign In" }</h1>
                <div class="entry">
                    <label for="username">{ "Username" }</label>
                    <input id="username" name="username" type="text" />
                </div>
                <div class="entry">
                    <label for="password">{ "Password" }</label>
                    <input id="password" name="password" type="password" />
                    <a href="">{ "Forgot Password?" }</a>
                </div>
                <button class="submit-button">{ "Sign In" }</button>
            </form>
        </div>
    }
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
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={ switch } />
        </BrowserRouter>
    }
}
