error_chain! {
    foreign_links {
        Hyper(::hyper::Error);
        Tls(::native_tls::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
    }

    errors {
        PushoverError {
            status: i32,
            errors: Vec<String>,
            request: String
        }
    }
}