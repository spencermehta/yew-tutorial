use crate::app::App;

mod app;
mod video;
mod video_details;
mod videos_list;

fn main() {
    yew::start_app::<App>();
}
