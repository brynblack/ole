use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            action: "https://localhost:8081/api/auth",
            class: "login-box",
            method: "post",
            span { "Sign In" }
            input { r#type: "text", id: "username", name: "username", placeholder: "Username" }
            div {
                class: "password",
                input { r#type: "password", id: "password", name: "password", placeholder: "Password" }
                a { href: "", "Forgot Password?" }
            }
            button { class: "submit-button", r#type: "submit", "Log in" }
        }
    })
}
