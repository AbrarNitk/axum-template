#[tokio::main]
async fn main() {
    let profile = std::env::var("PROFILE").ok().unwrap_or("dev".to_owned());
    let settings = match server::settings::Settings::with_file(profile.as_str()) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("error in reading service settings: {}", err);
            std::process::exit(1);
        }
    };

    server::listener::http_main(&settings)
        .await
        .expect("somethings went wrong");
}
