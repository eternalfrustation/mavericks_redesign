use enum_iterator::{all, reverse_all, Sequence};
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(|cx| {
        view! { cx,
            Page(id="landing_page") {
                Navbar {}
                Landing {}
            }
            Page(id="projects") {Projects {}}
            Page(id="about") {About {}}
            Page(id="gallery") {Gallery {}}
            Page(id="team") {Team {}}
            Page(id="contact") {Contact {}}
            (footer(cx))
        }
    });
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, Hash)]
enum Routes {
    Home,
    About,
    Gallery,
    Projects,
    NotFound,
}

impl ToString for Routes {
    fn to_string(&self) -> String {
        match self {
            Self::Home => "Home".to_string(),
            Self::About => "About".to_string(),
            Self::Gallery => "Gallery".to_string(),
            Self::Projects => "Projects".to_string(),
            Self::NotFound => "Not Found".to_string(),
        }
    }
}

impl Routes {
    fn path(&self) -> String {
        match self {
            Self::Home => "/#landing".to_string(),
            Self::About => "/#about".to_string(),
            Self::Gallery => "/#gallery".to_string(),
            Self::Projects => "/#projects".to_string(),
            Self::NotFound => "not_found".to_string(),
        }
    }
}

#[component]
fn Navbar<G: Html>(cx: Scope) -> View<G> {
    let mut routes = reverse_all::<Routes>();
    routes.next();
    let routes = create_signal(cx, routes.collect::<Vec<Routes>>());
    view! {
        cx,
        div(class="navbar"){
            img(src="/images/logo.svg")
                div(class="navbar_buttons"){
                Indexed(
                    iterable=routes,
                    view=|cx, x| view! {
                        cx, a(href=(x.path())){div{(x.to_string())}}
                },
                )
            }
        }
    }
}

async fn transition_effect<'a>(
    current_signal: &'a Signal<String>,
    next: String,
    stay_time: u32,
    letter_time: u32,
) {
    let mut current = (*current_signal.get_untracked()).clone();

    while !current.is_empty() {
        TimeoutFuture::new(letter_time).await;
        current.pop();
        current_signal.set(current.clone());
    }
    let next = next.chars();
    for c in next {
        TimeoutFuture::new(letter_time).await;
        current.push(c);
        current_signal.set(current.clone());
    }
    TimeoutFuture::new(stay_time).await;
}

#[component]
fn Landing<G: Html>(cx: Scope) -> View<G> {
    let mottos = vec!["Born to be different", "Sample Line 2", "Sample Line 3"];
    let mut i = 0;
    let current_motto = create_signal(cx, String::new());
    spawn_local_scoped(cx, async move {
        loop {
            transition_effect(current_motto, mottos[i].to_string(), 2000, 150).await;
            i += 1;
            i %= mottos.len()
        }
    });
    view! {
        cx,
        div(class="landing") {
            div(class="heading"){
                "Team Mavericks"
            }
            div(class="motto"){
                (current_motto.get())
            }
            div(class="cta") {
                a(href="#projects"){"Explore"}
            }
        }
    }
}

enum Domain {
    Mechnical,
    Electrical,
    Electronics

}

struct Project {
    name: String,
    image: String,
    desc: String,
    domain: Domain,
}

#[component]
fn Projects<G: Html>(cx: Scope) -> View<G> {
    let projects = [
        Project {
            name: "Project Name 1".into(),
            desc: "Project 1 Description".into(),
            image: "/images/img1.png".into(),
            domain: Domain::Mechnical,
        },
        Project {
            name: "Project Name 2".into(),
            desc: "Project 2 Description".into(),
            image: "/images/img2.png".into(),
            domain: Domain::Mechnical,
        },
        Project {
            name: "Project Name 3".into(),
            desc: "Project 3 Description".into(),
            image: "/images/img3.png".into(),
            domain: Domain::Electrical,
        },
        Project {
            name: "Project Name 4".into(),
            desc: "Project 4 Description".into(),
            image: "/images/img4.png".into(),
            domain: Domain::Electronics,
        },
    ];
    let projects = [
        ("/images/img1.png", "Project Name 1", "Short Project Desc 1"),
        ("/images/img2.png", "Project Name 2", "Short Project Desc 2"),
        ("/images/img3.png", "Project Name 3", "Short Project Desc 3"),
        ("/images/img4.png", "Project Name 4", "Short Project Desc 4"),
    ];
    let current_image = create_signal(cx, 0);
    let current_caption_title = create_signal(cx, projects[0].1.to_string());
    let current_caption_content = create_signal(cx, projects[0].2.to_string());
    let state = create_signal(cx, true);
    view! {
        cx,
        button(class="btn", on:click= move |_| {spawn_local_scoped(cx, async move {
            if *current_image.get() == 0 {
                current_image.set(projects.len() - 1);
            }
            state.set(!*state.get_untracked());
            current_image.set(*current_image.get() - 1);
            transition_effect(
                current_caption_title,
                projects[*current_image.get()].1.to_string(),
                100,
                20,
            )
            .await;
            transition_effect(
                current_caption_content,
                projects[*current_image.get()].2.to_string(),
                100,
                20,
            ).await;
        })}) {
            "⮜"
        }
        div(class="heading"){
            "Projects"
        }
        div(class="content") {
            img(src=(if *state.get() {projects[*current_image.get()].0} else {projects[(*current_image.get() + 1) % projects.len()].0}), class=(if *state.get() {""} else {"hidden"}))
                span(class="caption_title") {(current_caption_title.get())}
            img(src=(if !*state.get() {projects[*current_image.get()].0} else {projects[(*current_image.get() + 1) % projects.len()].0}),class=(if *state.get() {"hidden"} else {""}))
                span(class="caption_title") {(current_caption_title.get())}
            span(class="caption_content") {(current_caption_content.get())}
        }
        button(class="btn", on:click= move |_| {spawn_local_scoped(cx, async move {
            state.set(!*state.get_untracked());
            current_image.set( (*current_image.get() + 1) % projects.len());
            transition_effect(
                current_caption_title,
                projects[*current_image.get()].1.to_string(),
                2000,
                20,
            )
            .await;
            transition_effect(
                current_caption_content,
                projects[*current_image.get()].2.to_string(),
                2000,
                20,
            )
            .await;
        })}) {
            "⮞"
        }
    }
}

#[component]
fn Gallery<G: Html>(cx: Scope) -> View<G> {
    let images = [
        ("/images/img1.png", "caption 1"),
        ("/images/img2.png", "caption 2"),
        ("/images/img3.png", "caption 3"),
    ];
    let current_image = create_signal(cx, 0);
    let current_caption = create_signal(cx, images[0].1.to_string());
    view! {
            cx,
            button(class="btn", on:click= move |_| {spawn_local_scoped(cx, async move {
                if *current_image.get() == 0 {
                    current_image.set(images.len() - 1);
                }
                current_image.set(*current_image.get() - 1);
                transition_effect(
                    current_caption,
                    images[*current_image.get()].1.to_string(),
                    2000,
                    20,
                )
                .await;
            })}) {
                "⮜"
            }
            div(class="heading") { "Gallery" }
            div(class="content") {
                img(src=(images[*current_image.get()].0))
                span {(current_caption.get())}
        }
        button(class="btn", on:click= move |_| {spawn_local_scoped(cx, async move {
            current_image.set( (*current_image.get() + 1) % images.len());
            transition_effect(
                current_caption,
                images[*current_image.get()].1.to_string(),
                2000,
                20,
            )
            .await;
        })}) {
            "⮞"
        }
    }
}

#[component]
fn About<G: Html>(cx: Scope) -> View<G> {
    let content = create_signal(cx, [
        ("Establishment", vec![
            "Mavericks Club, SLIET was established by Dhurv Patel (GEE/2015). in 2017"]),

        ("Aim", vec![
            "Offering an excellent opportunity for students with backgrounds in Mechanical, Mechatronics, Electronics, Computers, and Electrical engineering to enhance their skills and demonstrate their talent through a wide range of interdisciplinary activities."]),

        ("Objectives", vec![
            "To promote creativity and to increase the technical now – how and productivity of all the students at the college.",
            "To promote hands -on and co-operative learning and also engages students in problem solving higher order thinking.",
            "To develop the spirit of entrepreneurship among the students of institute."]),

        ("Achievements", vec![
            "Won in Smart India Hackathon (SIH) in 2019 & 2020.",
            "Two Team qualified for the zonal rounds of DRDO'S Unmanned Vehicle Challenge.", 
            "Among the top teams in E-Yantra 2017, 2018, 2019 Competition organised by { MOE } in accordance with IIT Bombay.", 
            "Spreaded glories at various technical fest, including IIT Bombay, IIT Kanpur, IIT Delhi, IIT Bhu, IIT Ropar, PEC, Thapar University and recevied cash prize at Guru Nanak Dev University , Amritsar Punjab.",
            "In our team, we provide guidance for developing skills in various Coding Platforms, Development Board, Cricut Design Software, 3D Modelling Software, Hardware Implementation and the latest technologies."]),

        ("Student Ideas", vec![
            "Prashant Sharma (Drone Ninja) : The purpose of this idea is to make a drone that can help our farmers in farming.",
            "Ashish Bobby (Smart Irrigation) : Making an automated irrigation system that can judiciously use water for irrigation."
        ]),
    ]);
    let current_heading = create_signal(cx, content.get()[0].0.to_string());
    let current_content = create_signal(cx, content.get()[0].1[0].to_string());
    let i = create_signal(cx, 0 as usize);
    let j = create_signal(cx, 0 as usize);
    spawn_local_scoped(cx, async move {
        loop {
            j.set(*j.get() + 1);
            j.set(*j.get() % content.get()[*i.get()].1.len());
            if content.get()[*i.get()].1.len() > 1
                && content.get()[*i.get()].1[*j.get()].len() == current_content.get().len()
            {
                transition_effect(
                    current_content,
                    content.get()[*i.get()].1[*j.get()].to_string(),
                    2000,
                    20,
                )
                .await;
            } else {
                TimeoutFuture::new(2500).await;
            }
        }
    });
    view![
        cx,
        button(class="btn", on:click= move |_| {spawn_local_scoped(cx, async move {
            if *i.get() == 0 {
                i.set(content.get().len() - 1);
            }
            transition_effect(current_heading, content.get()[*i.get()].0.to_string(), 1000, 100).await;
            transition_effect(
                current_content,
                content.get()[*i.get()].1[0].to_string(),
                2000,
                20,
            )
            .await;
        })}) {
            "⮜"
        }
            div(class="heading") {
            (current_heading.get())
        }
            div(class="content") {
            (current_content.get())
        }
            button(class="btn", on:click= move |_| {
            spawn_local_scoped(cx, async move {
                i.set((*i.get() as isize + 1) as usize % content.get().len());
                transition_effect(current_heading, content.get()[*i.get()].0.to_string(), 1000, 100).await;
                transition_effect(
                    current_content,
                    content.get()[*i.get()].1[0].to_string(),
                    2000,
                    10,
                )
                .await;
            })
        }) {
            "⮞"
        }
    ]
}

#[derive(Prop)]
pub struct PageProps<'a, G: Html> {
    children: Children<'a, G>,
    id: &'a str,
}

#[component]
pub fn Page<'a, G: Html>(cx: Scope<'a>, props: PageProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    view! {
        cx,
        div(class="page", id=(props.id)) {
            (children)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Rank {
    FacultyAdvisor,
    SenCoordinator,
    SocialMedia,
    MechanicalHead,
    WebDev,
}

impl ToString for Rank {
    fn to_string(&self) -> String {
        match self {
            Self::FacultyAdvisor => "Faculty Advisor",
            Self::SenCoordinator => "Senior Coordinator",
            Self::SocialMedia => "Social Media",
            Self::MechanicalHead => "Mechanical Head",
            Self::WebDev => "Web Development",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Deptt {
    GEE,
    GEC,
    GME,
    GCS,
    GFT,
    GCT,
}

impl ToString for Deptt {
    fn to_string(&self) -> String {
        match self {
            Self::GEC => "GEC",
            Self::GEE => "GEE",
            Self::GME => "GME",
            Self::GFT => "GFT",
            Self::GCS => "GCS",
            Self::GCT => "GCT",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct Faculty {
    photo: String,
    name: String,
    rank: Rank,
    deptt: Deptt,
}

impl Faculty {
    fn view<'a, G: Html>(self, cx: Scope<'a>) -> View<G> {
        view! {
            cx,
            div(class="card card_faculty") {
                img(src=(self.photo), class="card_photo")
                div(class="card_info") {
                    div(class="card_name") {span{"Name"} span{(self.name)}}
                    div(class="card_rank") {span{"Rank"} span{(self.rank.to_string())}}
                    div(class="card_deptt") {span{"Deptt"} span{(self.deptt.to_string())}}
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Member {
    name: String,
    rank: Rank,
    deptt: Deptt,
    roll_no: String,
    mail: String,
    phone: String,
    photo: String,
}

impl Member {
    fn view<'a, G: Html>(self, cx: Scope<'a>) -> View<G> {
        view! {
            cx,
            div(class="card card_member") {
                img(src=(self.photo), class="card_photo")
                    div(class="card_info") {
                        div(class="card_name") {span{"Name"} span{(self.name)}}
                        div(class="card_rank") {span{"Rank"} span{(self.rank.to_string())}}
                        div(class="card_deptt") {span{"Deptt"} span{(self.deptt.to_string())}}
                        div(class="card_roll_no") {span{"Roll No."} span{(self.roll_no)}}
                        div(class="card_mail") {span{"e-Mail"} span{(self.mail)}}
                        div(class="card_phone") {span{"Phone"} span{(self.phone)}}
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, Hash)]
enum TeamSections {
    Faculty,
    CommMembers,
    Alumni,
    Members,
}

#[component]
pub fn Team<G: Html>(cx: Scope) -> View<G> {
    let faculty = [Faculty {
        name: "Dr. J.S. Ubhi".to_string(),
        rank: Rank::FacultyAdvisor,
        deptt: Deptt::GEC,
        photo: "/images/img2.png".to_string(),
    }];
    let faculty = create_signal(cx, faculty.to_vec());
    let members = [
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
    ];
    let alumni = [
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
    ];
    let committee = [
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
    ];
    let committee = create_signal(cx, committee.to_vec());
    let members_signal = create_signal(cx, members[0..5].to_vec());
    let alumni_signal = create_signal(cx, alumni[0..5].to_vec());
    let mut members_index = 0;
    let mut alumni_index = 0;
    let faculty_visibile = create_signal(cx, false);
    let members_visibile = create_signal(cx, false);
    let alumni_visibile = create_signal(cx, false);
    view![
        cx,
        div(class="heading") {
            "Our Team"
        }
        div(class="content") {
            div(class="team-buttons") {
                button(id="faculty-btn", on:click=|_| {
                    faculty_visibile.set(!*faculty_visibile.get());
                    members_visibile.set(false);
                    alumni_visibile.set(false);
                }) {
                    "Faculty"
                }
                button(id="members-btn", on:click=|_| {
                    members_visibile.set(!*members_visibile.get());
                    faculty_visibile.set(false);
                    alumni_visibile.set(false);
                }) {
                    "Members"
                }
                button(id="alumni-btn", on:click=|_| {
                    alumni_visibile.set(!*alumni_visibile.get());
                    faculty_visibile.set(false);
                    members_visibile.set(false);
                }) {
                    "Alumni"
                }
            }
            div(class= {let mut classes = String::from("faculty faculty_section");if (!*faculty_visibile.get()) {classes += " hidden";classes} else {classes}}) {
                div(class="title") {"Faculty"}
                    div(class="cards") {
                        Keyed(
                            iterable=faculty,
                            view=|cx, x| (x.view(cx)),
                            key=|x| x.clone(),
                        )
                    }
            }
            div(class={let mut classes = String::from("members faculty_section");if (!*members_visibile.get()) {classes += " hidden";classes} else {classes}}) {
                div(class="title") {"Members"}
                    div(class="cards", on:click=move |_| {
                        let l = members_signal.get_untracked().len();
                        let mut members_signal = members_signal.modify();
                        members_signal.remove(0);
                        members_signal.push(members[members_index+l].clone());
                        members_index+=1;
                        if members_index + l >= members.len()  {
                            members_index = 0;
                        }
                    }) {
                    Keyed(
                        iterable=members_signal,
                        view=|cx, x| (x.view(cx)),
                        key=|x| x.clone(),
                    )
                }
            }
            div(class={let mut classes = String::from("alumni faculty_section");if (!*alumni_visibile.get()) {classes += " hidden";classes} else {classes}}) {
                div(class="title") {"Alumni"}
                    div(class="cards", on:click=move |_| {
                        let l = alumni_signal.get_untracked().len();
                        let mut alumni_signal = alumni_signal.modify();
                        alumni_signal.remove(0);
                        alumni_signal.push(alumni[alumni_index+l].clone());
                        alumni_index+=1;
                        if alumni_index + l >= alumni.len()  {
                            alumni_index = 0;
                        }
                    }) {
                    Indexed(
                        iterable=alumni_signal,
                        view=|cx, x| (x.view(cx)),
                    )
                }
            }
        }
    ]
}

#[component]
pub fn Contact<G: Html>(cx: Scope) -> View<G> {
    let contact_people = Vec::from([
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
        Member {
            name: "Prashant Sharma".to_string(),
            rank: Rank::SenCoordinator,
            roll_no: "2040301".to_string(),
            mail: "2040301@sliet.ac.in".to_string(),
            phone: "8393021054".to_string(),
            photo: "/images/img1.png".to_string(),
            deptt: Deptt::GEE,
        },
    ]);
    let members = View::new_fragment(contact_people.iter().map(|x| x.clone().view(cx)).collect());
    view! {
        cx,
        div(class="heading") {"Contact US"}
        div(class="content") {
            div(id="email") {
                "mavericks@sliet.ac.in"
            }
            div(id="favicons") {
                a(href="") {img(src="https://upload.wikimedia.org/wikipedia/commons/a/a0/YouTube_social_red_circle_%282017%29.svg", width=64)}
                a(href="https://instagram.com/mavericks_sliet?igshid=YmMyMTA2M2Y=") {img(src="https://upload.wikimedia.org/wikipedia/commons/9/95/Instagram_logo_2022.svg", width=64)}
                a(href="https://www.linkedin.com/company/team-mavericks/") {img(src= "https://upload.wikimedia.org/wikipedia/commons/8/81/LinkedIn_icon.svg" , width=64)}

            }
            div(class="cards") {
                (members)
            }
        }
    }
}

fn footer<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(id="footer") {
            div(id="copyright") {"CopyrightⒸ Mavericks Club SLIET"}
            div(id="designed_by") {"Designed & Developed | Ritesh Prasad | Sandeep Kumar"}
        }
    }
}
