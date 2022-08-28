use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnspecifiedError {
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToClient {
    MatchmakingQueue(MatchmakingQueue),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchmakingQueue {
    Register(Register),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Register {
    Ok {},
    Error(RegErr)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegErr {
    BadName { description: String },
    AlreadyRegistered { description: String },
    UnspecifiedError { description: String }
}

#[macro_export]
macro_rules! reg_ok {
    () => {
        fpc_proto::to_client::ToClient::MatchmakingQueue(
            fpc_proto::to_client::MatchmakingQueue::Register(fpc_proto::to_client::Register::Ok {}),
        )
    };
}

#[macro_export]
macro_rules! reg_err_bad_name {
    ($e:expr) => {
        fpc_proto::to_client::ToClient::MatchmakingQueue(
            fpc_proto::to_client::MatchmakingQueue::Register(
                fpc_proto::to_client::Register::Error(
                    fpc_proto::to_client::RegErr::BadName { description: $e.into() }
                )
            )
        )
    };
}

#[macro_export]
macro_rules! reg_err_already {
    ($e:expr) => {
        fpc_proto::to_client::ToClient::MatchmakingQueue(
            fpc_proto::to_client::MatchmakingQueue::Register(
                fpc_proto::to_client::Register::Error(
                    fpc_proto::to_client::RegErr::AlreadyRegistered { description: $e.into() }
                )
            )
        )
    };
}

#[macro_export]
macro_rules! reg_err_unspec {
    ($e:expr) => {
        fpc_proto::to_client::ToClient::MatchmakingQueue(
            fpc_proto::to_client::MatchmakingQueue::Register(
                fpc_proto::to_client::Register::Error(
                    fpc_proto::to_client::RegErr::UnspecifiedError { description: $e.into() }
                )
            )
        )
    };
}