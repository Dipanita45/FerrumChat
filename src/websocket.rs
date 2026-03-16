use crate::error::HttpError;
use crate::handlers::chat::{read_loop, write_loop};
use crate::handlers::users_chat::{register_user, unregister_user};
use crate::utils::token::decode_token;
use crate::{AppState, handlers::user};
use axum::Extension;
use axum::extract::State;
use axum::{
    Router,
    extract::{
        Query,
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::any,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::sync::Arc;
use tracing;
use uuid::Uuid;

#[derive(Deserialize)]
struct WsQuery {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    tracing::debug!("WS connection attempt with token: {}", &query.token[..20]);

    let user_id: Uuid = match decode_token(&query.token, state.env.jwt_secret.as_bytes()) {
        Ok(id) => {
            tracing::debug!("Token decoded, id: {}", id);
            match Uuid::parse_str(&id) {
                Ok(uuid) => uuid,
                Err(e) => {
                    tracing::error!("UUID parse failed: {}", e);
                    return HttpError::unauthorized("Invalid user id").into_response();
                }
            }
        }
        Err(e) => {
            tracing::error!("Token decode failed: {:?}", e);
            return e.into_response();
        }
    };

    tracing::debug!("WS upgrading for user: {}", user_id);
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

pub fn ws_routes() -> Router<Arc<AppState>> {
    Router::new().route("/ws", any(ws_handler))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    let (ws_sender, ws_receiver) = socket.split();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    tracing::debug!("Registering user: {}", user_id);

    register_user(user_id, tx.clone(), &state).await;

    tracing::debug!("User registered: {}", user_id);

    let state_clone = state.clone();

    let mut write_task = tokio::spawn(write_loop(ws_sender, rx, user_id));

    let mut read_task = tokio::spawn(read_loop(ws_receiver, user_id, state_clone));

    tokio::select! {
        _ = &mut write_task => {
            tracing::debug!("Write task ended for user: {}", user_id);
            read_task.abort();
        }
        _ = &mut read_task => {
            tracing::debug!("Read task ended for user: {}", user_id);
            write_task.abort();
        }
    }

    drop(tx);
    unregister_user(user_id, &state).await;
}

async fn socket_close(
    user_id: Uuid,
    state: Arc<AppState>,
    mut socket: WebSocket,
    reason: &str,
    code: u16,
) {
    unregister_user(user_id, &state).await;
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: code.into(),
            reason: reason.to_string().into(),
        })))
        .await;
}
