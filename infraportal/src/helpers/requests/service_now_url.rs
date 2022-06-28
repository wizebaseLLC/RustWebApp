use dotenv::dotenv;
use std::env;

pub fn is_prod() -> bool {
    dotenv().ok();
    let my_env = env::var("INFRAPORTAL_ENV").unwrap_or("dev".to_string());
    my_env == "prod".to_string()
}

pub fn service_now_url<'a>() -> &'a str {
    let is_prod = is_prod();

    match is_prod {
        true => "https://nb.service-now.com",
        _ => "https://neubergertest.service-now.com",
    }
}
