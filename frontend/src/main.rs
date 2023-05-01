use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <form class="login-box" action="https://localhost:8081/api/auth" method="post">
                <span>{ "Sign In" }</span>
                <input placeholder="Username" name="username" type="text"/>
                <div class="password">
                    <input placeholder="Password" name="password" type="password"/>
                    <a href="">{ "Forgot Password?" }</a>
                </div>
                <button class="submit-button">{ "Sign In" }</button>
            </form>
        </>
    }
}
