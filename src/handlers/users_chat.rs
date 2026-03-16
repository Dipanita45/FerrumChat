use axum::extract::ws::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, mpsc::UnboundedSender};
use uuid::Uuid;

use crate::{AppState, error::HttpError};

pub async fn register_user(user_id: Uuid, sender: UnboundedSender<Message>, state: &Arc<AppState>) {
    let mut sessions = state.active_sessions.lock().await;
    sessions.insert(user_id, sender);
}

pub async fn unregister_user(user_id: Uuid, state: &Arc<AppState>) {
    let mut sessions = state.active_sessions.lock().await;
    sessions.remove(&user_id);
}

pub async fn send_to_user(
    user_id: Uuid,
    message: Message,
    state: &Arc<AppState>,
) -> Result<(), HttpError> {
    let sessions = state.active_sessions.lock().await;

    if let Some(sender) = sessions.get(&user_id) {
        sender
            .send(message)
            .map_err(|_| HttpError::server_error("User disconnected"))?;
        Ok(())
    } else {
        Err(HttpError::bad_request("User offline"))
    }
}

pub async fn send_to_many(user_ids: Vec<Uuid>, message: Message, state: &Arc<AppState>) {
    for user_id in user_ids {
        let _ = send_to_user(user_id, message.clone(), state).await;
    }
}
