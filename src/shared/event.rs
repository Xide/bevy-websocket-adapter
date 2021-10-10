use thiserror::Error as TError;
use super::ConnectionHandle;

#[derive(TError, Debug)]
pub enum NetworkError {}

#[derive(Debug)]
pub enum NetworkEvent {
    Connected(ConnectionHandle),
    Disconnected(ConnectionHandle),
    Message(ConnectionHandle, Vec<u8>),
    Error(Option<ConnectionHandle>, anyhow::Error),
}
