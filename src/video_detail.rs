use crate::video::Video;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct VideoDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    let video = video.clone();
    html! {
          <div>
          <h3>{ video.title.clone() }</h3>

    <iframe width="420" height="315"
    src= {video.url.clone()}>
    </iframe>

          </div>
      }
}
