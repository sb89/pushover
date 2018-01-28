error_chain! {
    foreign_links {
        Json(::serde_json::Error);
        Io(::std::io::Error);
        Reqqest(::reqwest::Error);
    }

    errors {
        PushoverError {
            status: i32,
            errors: Vec<String>,
            request: String
        }
    }
}