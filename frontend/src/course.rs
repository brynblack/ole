use crate::CourseProps;
use crate::NavBar;
use common::CourseInfo;
use yew::prelude::*;

#[function_component(Course)]
pub fn course(props: &CourseProps) -> Html {
    let course = use_state(|| CourseInfo::default());

    {
        let course = course.clone();
        let id = props.id.clone();

        wasm_bindgen_futures::spawn_local(async move {
            course.set(
                reqwest::Client::new()
                    .get(format!("https://localhost:8081/api/courses/{}", id + 1))
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
                <img src={ course.image.clone() } />
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
