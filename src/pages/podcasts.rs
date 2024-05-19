use axum::{response::IntoResponse, routing, Router};
use maud::{html, PreEscaped};
use tokio::fs;

use crate::state::AppState;

use super::page::PageKind;

pub async fn podcasts(page_kind: PageKind) -> impl IntoResponse {
    let markup = fs::read_to_string("generated/pages/podcasts.html")
        .await
        .unwrap();
    page_kind.builder("Home").build(html! {
        div .padded-when-small {
            (PreEscaped(markup))
        }
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/podcasts", routing::get(podcasts))
}
