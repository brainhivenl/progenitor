// Copyright 2021 Oxide Computer Company

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

fn main() {
    let client = Client::new("https://foo/bar");
    let _ = client.enrol(
        "auth-token",
        &EnrolBody {
            host: "".to_string(),
            key: "".to_string(),
        },
    );
}
