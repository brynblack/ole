use crate::home::Home;
use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;

use serde::Deserialize;

mod home;

pub fn run() {
    yew::Renderer::<App>::new().render();
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/courses/:id")]
    Course { id: usize },
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
struct CardProps {
    id: usize,
    name: String,
    img: String,
}

#[function_component(CourseCard)]
fn course_card(props: &CardProps) -> Html {
    html! {
        <Link<Route> to={Route::Course { id: props.id }}>
            <div style={ "background-image: url(\"".to_owned() + &props.img + "\");" }>
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
    html! {
        <nav>
            <div>
                <img class="ham" alt="Overview" src="https://upload.wikimedia.org/wikipedia/commons/b/b2/Hamburger_icon.svg" />
                <h1>{ &props.title }</h1>
            </div>
            <Link<Route> to={Route::Login} classes="pfp">
                <img alt="Profile" src="https://avatars.githubusercontent.com/u/49110391?v=4" />
            </Link<Route>>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
struct CourseProps {
    id: usize,
}

struct CourseInfo {
    name: String,
    desc: String,
    img: String,
}

#[function_component(Course)]
fn course(props: &CourseProps) -> Html {
    let courses = vec![CourseInfo {
        name: "Software Design Development".into(),
        desc: "This course provides students with the opportunity to develop skills in designing and developing software solutions, project management and communication. It does this by looking at the different ways in which software can be developed, the tools that can be used to assist in this process and by considering the interaction between software and other components of computer systems. Students apply a systematic approach to develop and document software solutions using a variety of data structures and language facilities.".into(),
        img: "https://www.newsmaritime.com/wp-content/uploads/2016/06/Check-out-the-new-and-functional-programming-language.jpg".to_string()
    },
        CourseInfo {
        name: "Information Processes Technology".into(),
        desc: "This course provides students with the opportunity to learn about information-based systems and how social, ethical and non-computer procedures resulting from the processes are considered. Students study different types of information systems and through project work create their own information systems to meet identified needs.".into(),
        img: "https://www.databankimx.com/wp-content/uploads/2016/01/Database-Services-page-banner.jpg".to_string()
    },
        CourseInfo {
        name: "Coding Club".into(),
        desc: "Please come back coding club :(".into(),
        img: "https://theaxiom.ca/wp-content/uploads/2015/03/muchbetter.jpg".to_string()
    },
        CourseInfo {
        name: "Information Software Technology".into(),
        desc: "Information and Software Technology Years 7–10 provides students with the opportunity to develop computational, systems and design thinking skills through the development of practical projects. The course provides students with specialised knowledge of past, current and advancing technologies, data, hardware, software and the roles of people involved in information and software technology. Students explore developments and future directions in the exciting and challenging field of information and software technology. The course fosters an interest in, enjoyment of and encourages critical reflection of information and software technology as an integral part of modern society.".into(),
        img: "https://www.myhtptech.com/wp-content/uploads/2020/07/businessIntelligenceservices.jpg".to_string()
    }
    ];

    let course = &courses[props.id];

    html! {
        <div class="course-page">
            <NavBar title={ course.name.clone() } />
            <div class="metadata">
                <p>{ &course.desc }</p>
                <img src={ course.img.clone() } />
            </div>
            <h4>{ "Lessons" }</h4>
            <div class="lessons">
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
            </div>
        </div>
    }
}

#[derive(Deserialize)]
struct Info {
    token: String,
}

#[function_component(Login)]
fn login() -> HtmlResult {
    let navigator = use_navigator().unwrap();
    let onsubmit = Callback::from(move |_| navigator.push(&Route::Home));

    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    let res = use_future(|| async {
        let client = reqwest::Client::new();

        client
            .post("https://localhost:8081/api/auth")
            .form(&[("username", "brynleyl"), ("password", "meow")])
            .send()
            .await?
            .json::<Info>()
            .await
    })?;

    let token = match *res {
        Ok(ref res) => Some(res.token.clone()),
        Err(_) => None,
    };

    user_ctx.set(UserInfo { token });

    Ok(html! {
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
    })
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
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[derive(Default, PartialEq)]
struct UserInfo {
    token: Option<String>,
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
