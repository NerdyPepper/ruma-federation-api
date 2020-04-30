//! [PUT ](https://matrix.org/docs/spec/server_server/r0.1.3#get-matrix-federation-v1-make-join-roomid-userid)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_events::EventJson;
use ruma_identifiers::RoomId;
use serde::{Deserialize, Serialize};

use crate::RoomV3Pdu;

ruma_api! {
    metadata {
        description: "Send a join event to a resident server.",
        name: "send_join_event",
        method: PUT,
        path: "/_matrix/federation/v1/send_join/:room_id/:event_id",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The room ID that is about to be joined.
        #[ruma_api(path)]
        pub room_id: RoomId,
        /// The user ID the join event will be for.
        #[ruma_api(path)]
        pub event_id: String,
        /// Join membership event to send to resident server.
        #[serde(flatten)]
        pub event: EventJson<RoomV3Pdu>,

    }

    response {
        // TODO: What is this integer for?
        pub response: JoinResponse,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JoinResponse {
    // TODO: Don't know what this is for...
    pub some_number: UInt,
    /// The room state and authorization chain.
    pub state: RoomState,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoomState {
    /// The resident server's DNS name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// The full set of authorization events that make up the state of the room,
    /// and their authorization events, recursively.
    pub auth_chain: Vec<EventJson<RoomV3Pdu>>,
    /// The room state.
    pub state:  Vec<EventJson<RoomV3Pdu>>,
}