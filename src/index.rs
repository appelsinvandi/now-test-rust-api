use http::{header, Request, Response, StatusCode};
use serde_json::{json, Value};

fn handler(request: Request<()>) -> http::Result<Response<Value>> {
	let response = Response::builder()
		.status(StatusCode::OK)
		.header(header::CONTENT_TYPE, "text/json")
		.body(json!({ "path": "/", "method": request.method().to_string() }))
		.expect("failed to render response");

	Ok(response)
}
