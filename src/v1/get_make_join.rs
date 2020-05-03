//! [GET /_matrix/federation/v1/make_join/{roomId}/{userId}](https://matrix.org/docs/spec/server_server/r0.1.3#get-matrix-federation-v1-make-join-roomid-userid)

use std::time::SystemTime;

use ruma_api::ruma_api;
use ruma_identifiers::{RoomId, UserId};
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata {
        description: "Asks the receiving server to return information that the sending server will need to prepare a join event to get into the room.",
        method: GET,
        name: "get_make_join",
        path: "/_matrix/federation/v1/make_join/:room_id/:user_id",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The room ID that is about to be joined.
        #[ruma_api(path)]
        pub room_id: RoomId,
        /// The user ID the join event will be for.
        #[ruma_api(path)]
        pub user_id: UserId,
        /// The room versions the sending server has support for.
        #[serde(default = "default_versions")]
        #[ruma_api(query)]
        pub ver: Vec<String>
    }

    response {
        /// The version of the room where the server is trying to join.
        pub room_version: String,
        /// An unsigned template event.
        pub event: EventTemplate
    }
}

/// An unsigned template event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventTemplate {
    /// The user ID of the joining member.
    pub sender: String,
    /// The name of the resident homeserver.
    pub origin: String,
    /// A timestamp added by the resident homeserver.
    #[serde(with = "ruma_serde::time::ms_since_unix_epoch")]
    pub origin_server_ts: SystemTime,
    /// The value "m.room.member".
    #[serde(rename = "type")]
    pub event_type: String,
    /// The user ID of the joining member.
    pub state_key: String,
    /// The content of the event.
    pub content: MembershipEventContent,
}

/// The content of the event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MembershipEventContent {
    /// The value "join".
    pub membership: String,
}

fn default_versions() -> Vec<String> {
    vec!["1".into()]
}

#[cfg(test)]
mod tests {
    use super::Request;

    use std::convert::{TryFrom, TryInto};

    use ruma_identifiers::{RoomId, UserId};

    #[test]
    fn test_serialize_some_join_request() {
        let room_id = RoomId::try_from("!abc:matrix.org").unwrap();
        let user_id = UserId::try_from("@someone:example.org").unwrap();
        let req = Request {
            room_id,
            user_id,
            ver: vec!["1".into(), "2".into()],
        };
        let request: http::Request<Vec<u8>> = req.try_into().unwrap();
        assert_eq!(
            "%21abc123%3Amatrix.org/%40someone%3Aexample.org?ver=1&ver=2",
            request.uri().query().unwrap()
        );
    }
}
