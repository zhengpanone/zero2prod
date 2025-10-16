// use crate::models::post::Post;
// use crate::state::SharedState;
// use axum::{extract::State, routing::get, Json, Router};
// use utoipa::path;

// #[path(
// get,
// path = "/posts",
// responses((status = 200, description = "List posts", body = [Post]))
// )]
// async fn list_posts(
// 	State(state): State<SharedState>,
// ) -> anyhow::Result<Json<Vec<Post>>> {
// 	let posts = sqlx::query_as::<_, Post>(
// 		"SELECT id, title, content, created_at FROM posts",
// 	)
// 	.fetch_all(&state.db)
// 	.await?;
// 	Ok(Json(posts))
// }

// pub fn routes(state: SharedState) -> Router {
// 	Router::new()
// 		.route("/posts", get(list_posts))
// 		.with_state(state)
// }
