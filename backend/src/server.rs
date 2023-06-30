use crate::handlers::accounts::create_acc;
use crate::handlers::courses::create_course;
use crate::handlers::lessons::create_lesson;
use crate::AppState;
use crate::{database, routes::routes};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Json;
use actix_web::{web::Data, App, HttpServer};
use common::{CourseInfo, LessonInfo, NewAcc};
use log::info;
use std::env;

/// Starts the server.
pub async fn run() -> std::io::Result<()> {
    let app_port = env::var("APP_PORT")
        .expect("APP_PORT must be set")
        .parse()
        .unwrap();

    info!("starting up server...");

    let db_pool = database::db_connect();
    let app_data = Data::new(AppState { db_pool });
    let builder = crate::ssl_builder();

    // Create default admin account if first time setup
    create_acc(
        app_data.clone(),
        Json(NewAcc {
            username: "admin".to_string(),
            password: "password".to_string(),
            pfp: "https://upload.wikimedia.org/wikipedia/commons/b/b5/Windows_10_Default_Profile_Picture.svg".to_string(),
        }),
    )
    .await;

    create_course(
        app_data.clone(),
        Json(CourseInfo {
            name: "Software Design Development".to_string(),
            description: "This course provides students with the opportunity to develop skills in designing and developing software solutions, project management and communication. It does this by looking at the different ways in which software can be developed, the tools that can be used to assist in this process and by considering the interaction between software and other components of computer systems. Students apply a systematic approach to develop and document software solutions using a variety of data structures and language facilities.".to_string(),
            image: "http://wallup.net/wp-content/uploads/2017/03/29/476208-code-web_development-JavaScript-Computer_screen-pixels-programming-PHP-syntax_highlighting-programming_language-HTML.jpg".to_string(),
        }),
    )
    .await;

    create_lesson(
        app_data.clone(),
        Json(LessonInfo {
            reference: "software-design-development".to_string(),
            name: "Data Structures and Algorithms".to_string(),
            content: "A data structure is a named location that can be used to store and organize data. And, an algorithm is a collection of steps to solve a particular problem. Learning data structures and algorithms allow us to write efficient and optimized computer programs.".to_string(),
            image: "https://blog-c7ff.kxcdn.com/blog/wp-content/uploads/2019/11/Banner-Blog-1A-1.jpg".to_string(),
        }),
    )
    .await;

    // Initialise and run a new HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .configure(routes)
    })
    .bind_openssl(("localhost", app_port), builder)?
    .run()
    .await
}
