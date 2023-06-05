use crate::CourseProps;
use crate::NavBar;
use common::CourseInfo;
use yew::prelude::*;

#[function_component(Course)]
pub fn course(props: &CourseProps) -> Html {
    let courses = vec![CourseInfo {
        name: "Software Design Development".into(),
        description: "This course provides students with the opportunity to develop skills in designing and developing software solutions, project management and communication. It does this by looking at the different ways in which software can be developed, the tools that can be used to assist in this process and by considering the interaction between software and other components of computer systems. Students apply a systematic approach to develop and document software solutions using a variety of data structures and language facilities.".into(),
        image: "https://www.newsmaritime.com/wp-content/uploads/2016/06/Check-out-the-new-and-functional-programming-language.jpg".to_string()
    },
        CourseInfo {
        name: "Information Processes Technology".into(),
        description: "This course provides students with the opportunity to learn about information-based systems and how social, ethical and non-computer procedures resulting from the processes are considered. Students study different types of information systems and through project work create their own information systems to meet identified needs.".into(),
        image: "https://www.databankimx.com/wp-content/uploads/2016/01/Database-Services-page-banner.jpg".to_string()
    },
        CourseInfo {
        name: "Coding Club".into(),
        description: "Please come back coding club :(".into(),
        image: "https://theaxiom.ca/wp-content/uploads/2015/03/muchbetter.jpg".to_string()
    },
        CourseInfo {
        name: "Information Software Technology".into(),
        description: "Information and Software Technology Years 7–10 provides students with the opportunity to develop computational, systems and design thinking skills through the development of practical projects. The course provides students with specialised knowledge of past, current and advancing technologies, data, hardware, software and the roles of people involved in information and software technology. Students explore developments and future directions in the exciting and challenging field of information and software technology. The course fosters an interest in, enjoyment of and encourages critical reflection of information and software technology as an integral part of modern society.".into(),
        image: "https://www.myhtptech.com/wp-content/uploads/2020/07/businessIntelligenceservices.jpg".to_string()
    }
    ];

    let course = &courses[props.id];

    html! {
        <div class="course-page">
            <NavBar title={ course.name.clone() } />
            <div class="metadata">
                <p>{ &course.description }</p>
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
