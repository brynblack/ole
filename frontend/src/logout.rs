use crate::Redirect;
use crate::Route;
use crate::UserInfo;
use yew::html;
use yew::use_context;
use yew::UseStateHandle;
use yew::{function_component, Html};

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    user_ctx.set(UserInfo::default());

    html! {
        <Redirect<Route> to={Route::Login}/>
    }
}
