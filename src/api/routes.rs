//use crate::api::handlers;
use crate::utils::webhook_payload_parser;
use warp::Filter;
pub async fn register_routes(_job_queu: &mut queues::Queue<i32>) {
    let api_v1 = warp::path("api").and(warp::path("v1"));

    let handle_webhook = warp::path("webhook")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(|body: bytes::Bytes| async move {
            match serde_json::from_slice::<serde_json::Value>(&body) {
                Ok(json) => Ok(json),
                Err(_e) => Err(warp::reject::reject()),
            }
        })
        .and_then(move |content: serde_json::Value| {
            async move {
                // Process the content here
                webhook_payload_parser::parse_webhook(content).await
            }
        });
    //.and_then(handlers::handle_webhooks);

    let routes = api_v1.and(handle_webhook);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
