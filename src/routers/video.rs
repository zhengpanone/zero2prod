// use std::sync::Arc;

// use axum::{routing::get, Router};

// use crate::{handlers::video::stream_video, AppState};

// pub fn video_routes(state: Arc<AppState>) -> Router {
//     Router::new()
//         .route("video", get(stream_video))
//         .with_state(state)
// }
