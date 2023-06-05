use crate::CourseCard;
use crate::NavBar;
use crate::Route;
use crate::UserInfo;
use common::CourseInfo;
use serde::Serialize;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize)]
struct Auth {
    token: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    match &user_ctx.token {
        Some(_) => {}
        None => {
            return html! {
                <Redirect<Route> to={Route::Logout}/>
            }
        }
    };

    let courses = use_state(|| Vec::<CourseInfo>::new());

    {
        let courses = courses.clone();

        wasm_bindgen_futures::spawn_local(async move {
            courses.set(
                reqwest::Client::new()
                    .get("https://localhost:8081/api/courses")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<CourseInfo>>()
                    .await
                    .unwrap(),
            );
        });
    }

    html! {
        <div class="home-page">
            <div class="dashboard">
                <NavBar title={ "Home" } />
                <h4>{ "Courses" }</h4>
                <div class="courses">
                    {
                        for courses.iter().enumerate().map(|(id, course)| {
                            html! {
                                <CourseCard id={ id } name={ course.name.clone() } img={ course.image.clone() } />
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}
