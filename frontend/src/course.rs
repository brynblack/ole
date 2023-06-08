use crate::auth;
use crate::CourseProps;
use crate::NavBar;
use crate::UserInfo;
use crate::BASE_URL;
use common::CourseInfo;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Course)]
pub fn course(props: &CourseProps) -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    auth(&user_ctx, &navigator);

    let course = use_state_eq(|| CourseInfo::default());

    {
        let course = course.clone();
        let id = props.id.clone();

        wasm_bindgen_futures::spawn_local(async move {
            course.set(
                reqwest::Client::new()
                    .get(format!("{BASE_URL}/api/courses/{}", id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap(),
            );
        });
    }

    html! {
        <div class="course-page">
            <NavBar title={ course.name.clone() } />
            <div class="metadata">
                <p>{ course.description.clone() }</p>
                <div style={ format!("background-image: url({});", course.image.clone()) } />
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
