extern crate spaceapi_server;

use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::Optional::{Value, Absent};


fn main() {

    // Create new minimal Status instance
    let status = api::Status::new(
        "coredump",
        "https://www.coredump.ch/logo.png",
        "https://www.coredump.ch/",
        api::Location {
            address: Value("Spinnereistrasse 2, 8640 Rapperswil, Switzerland".into()),
            lat: 47.22936,
            lon: 8.82949,
        },
        api::Contact {
            irc: Value("irc://freenode.net/#coredump".into()),
            twitter: Value("@coredump_ch".into()),
            foursquare: Absent,
            email: Absent,
        },
        vec![
            "email".into(),
            "twitter".into(),
        ],
    );

    // Set up server
    let listen = "127.0.0.1:8000";
    let redis = "redis://127.0.0.1/";
    let modifiers = Vec::new();
    let server = SpaceapiServer::new(listen, status, redis, modifiers);

    // Serve!
    let _ = server.unwrap().serve();
}