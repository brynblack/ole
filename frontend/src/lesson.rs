use crate::NavBar;
use crate::BASE_URL;
use crate::{auth, LessonProps, UserInfo};
use common::LessonInfo;
use yew::{function_component, html, use_context, use_state_eq, Html, UseStateHandle};
use yew_router::prelude::use_navigator;

#[function_component(Lesson)]
pub fn lesson(props: &LessonProps) -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let navigator = use_navigator().unwrap();

    auth(&user_ctx, &navigator);

    let lesson = use_state_eq(LessonInfo::default);

    {
        let lesson = lesson.clone();
        let id = props.id.clone();

        wasm_bindgen_futures::spawn_local(async move {
            lesson.set(
                reqwest::Client::new()
                    .get(format!("{BASE_URL}/api/lessons/{}", id))
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
        <div class="lesson-page">
            <NavBar title={ lesson.name.clone() } />
            <div class="metadata">
                <p>{ lesson.content.clone() }</p>
                <div style={ format!("background-image: url({});", lesson.image.clone()) } />
            </div>
        </div>
    }
}
