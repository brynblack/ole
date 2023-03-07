use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "ole",
            style { include_str!("./assets/style.css") }
            form {
                class: "login-box",
                action: "https://localhost:8081/api/auth",
                method: "post",
                target: "temp",
                iframe {
                    name: "temp",
                    display: "none",
                }
                h1 { "Online Learning Environment" }
                input {
                    placeholder: "Username",
                    name: "username",
                    autofocus: "true",
                }
                input {
                    placeholder: "Password",
                    name: "password",
                    r#type: "password",
                }
                input {
                    r#type: "submit",
                    value: "Submit",
                }
            }
        }
    })
}
