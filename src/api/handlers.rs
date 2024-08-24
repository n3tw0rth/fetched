pub async fn handle_webhooks() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Updated the queue")
}
