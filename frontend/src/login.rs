use crate::Info;
use crate::Route;
use crate::UserInfo;
use crate::BASE_URL;
use common::AccountData;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    field_name: String,
    name: String,
    input_node_ref: NodeRef,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        field_name,
        name,
        input_node_ref,
    } = props;

    html! {
        <input type={field_name.clone()} name={name.clone()} ref={input_node_ref.clone()} />
    }
}

#[derive(Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[function_component(Login)]
pub fn login() -> HtmlResult {
    let navigator = use_navigator().unwrap();

    let username = use_node_ref();
    let password = use_node_ref();

    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let username = username.cast::<HtmlInputElement>().unwrap().value();
            let password = password.cast::<HtmlInputElement>().unwrap().value();
            let user_ctx = user_ctx.clone();
            let navigator = navigator.clone();

            let login_form = LoginForm { username, password };

            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();

                let res = client
                    .post(format!("{BASE_URL}/api/login"))
                    .form(&[
                        ("username", login_form.username.clone()),
                        ("password", login_form.password.clone()),
                    ])
                    .send()
                    .await
                    .unwrap()
                    .json::<Info>()
                    .await;

                let token = match res {
                    Ok(ref res) => Some(res.token.clone()),
                    Err(_) => None,
                };

                let res = client
                    .get(format!("{BASE_URL}/api/accounts/{}", login_form.username))
                    .send()
                    .await
                    .unwrap()
                    .json::<AccountData>()
                    .await;

                let data = match res {
                    Ok(res) => Some(res),
                    Err(_) => None,
                };

                user_ctx.set(UserInfo { token, data });
                navigator.push(&Route::Home);
            });
        })
    };

    Ok(html! {
        <div class="login-page">
            <form class="login-box" onsubmit={ onsubmit }>
                <h1>{ "Sign In" }</h1>
                <div class="entry">
                    <label for="username">{ "Username" }</label>
                    <InputField input_node_ref={ username } name={"username"} field_name={"text"} />
                </div>
                <div class="entry">
                    <label for="password">{ "Password" }</label>
                    <InputField input_node_ref={ password } name={"password"} field_name={"password"} />
                    <a href="">{ "Forgot Password?" }</a>
                </div>
                <button class="submit-button">{ "Sign In" }</button>
            </form>
        </div>
    })
}
