use crate::Info;
use crate::Route;
use crate::UserInfo;
use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::*;

#[function_component(Login)]
pub fn login() -> HtmlResult {
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
