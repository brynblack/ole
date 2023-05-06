use crate::CourseCard;
use crate::NavBar;
use crate::Route;
use crate::UserInfo;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap();

    match &user_ctx.token {
        Some(token) => token,
        // Redirects to the login page when user is `None`.
        None => {
            return html! {
                <Redirect<Route> to={Route::Login}/>
            }
        }
    };

    html! {
        <div class="home-page">
            <div class="dashboard">
                <NavBar title={ "Home" } />
                <h4>{ "Courses" }</h4>
                <div class="courses">
                    <CourseCard id={ 0 } name="Software Design Development" img="https://www.newsmaritime.com/wp-content/uploads/2016/06/Check-out-the-new-and-functional-programming-language.jpg" />
                    <CourseCard id={ 1 } name="Information Processes Technology" img="https://www.databankimx.com/wp-content/uploads/2016/01/Database-Services-page-banner.jpg" />
                    <CourseCard id={ 2 } name="Coding Club" img="https://theaxiom.ca/wp-content/uploads/2015/03/muchbetter.jpg" />
                    <CourseCard id={ 3 } name="Information Software Technology" img="https://www.myhtptech.com/wp-content/uploads/2020/07/businessIntelligenceservices.jpg" />
                </div>
            </div>
        </div>
    }
}
