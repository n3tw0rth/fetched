use serde_json::Value;
use warp::http::StatusCode;

pub async fn parse_webhook(payload: serde_json::Value) -> Result<BuildConfig, warp::Rejection> {
    let platform =
        detect_git_platform(payload.clone()).unwrap_or("Unrecognized platform".to_string());
    match platform.as_str() {
        "github" => parse_github(payload),
        _ => Err(warp::reject::not_found()),
    }
}

fn detect_git_platform(payload: serde_json::Value) -> Result<String, warp::Rejection> {
    if payload.to_string().find("api.bitbucket").is_some() {
        Ok("bitbucket".to_string())
    } else if payload.to_string().find("api.github").is_some() {
        Ok("github".to_string())
    } else {
        Err(warp::reject::not_found())
    }
}

#[derive(Debug, serde_derive::Serialize)]
pub struct BuildConfig {
    pub git_platform: String,
    pub repo_url: Value,
    pub events: Value,
}

impl warp::Reply for BuildConfig {
    fn into_response(self) -> warp::reply::Response {
        // Convert the struct to JSON
        let json = serde_json::to_string(&self).unwrap();

        // Create a response with the JSON body
        let mut res = warp::reply::Response::new(json.into());

        // Set the status code
        *res.status_mut() = StatusCode::from_u16(200).unwrap_or(StatusCode::OK);

        // Set the content type to JSON
        res.headers_mut().insert(
            "Content-Type",
            warp::http::header::HeaderValue::from_static("application/json"),
        );

        res
    }
}

fn parse_github(payload: serde_json::Value) -> Result<BuildConfig, warp::Rejection> {
    Ok(BuildConfig {
        git_platform: "github".to_string(),
        events: payload["hook"]["events"].clone(),
        repo_url: payload["repository"]["clone_url"].clone(),
    })
}
