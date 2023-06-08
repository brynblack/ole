use crate::auth;
use crate::CourseCard;
use crate::NavBar;
use crate::UserInfo;
use crate::BASE_URL;
use common::CourseInfo;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    auth(&user_ctx, &navigator);

    let courses = use_state_eq(|| Vec::<CourseInfo>::new());

    {
        let courses = courses.clone();

        wasm_bindgen_futures::spawn_local(async move {
            courses.set(
                reqwest::Client::new()
                    .get(format!("{BASE_URL}/api/courses"))
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
                        for courses.iter().map(|course| {
                            html! {
                                <CourseCard id={ course.name.to_lowercase().replace(" ", "-") } name={ course.name.clone() } img={ course.image.clone() } />
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}
