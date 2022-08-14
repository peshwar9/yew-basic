use yew::prelude::*;
mod video;
mod video_detail;
use crate::video::*;
use crate::video_detail::*;

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        } )
    };
    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    let videos = vec![
        Video {
            id: 1,
            title: "Bhagavad Gita Sahasragala parayana".to_string(),
            speaker: "HH Sri Ganapathy Sachchidananda Swamiji".to_string(),
            url: "www.dattapeetham.org".to_string(),
        },
        Video {
            id: 2,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 5,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
    html! {
        <>
        <div>
        <h3>{"Videos to watch"}</h3>
            <VideosList videos={videos} on_click={on_video_select.clone()} />
        </div>
        {for details}
        <div>
            <h3>{ "JGD. This is first static page with Yew and Wasm" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
        </>
    }
}
fn main() {
    yew::start_app::<App>();
}
