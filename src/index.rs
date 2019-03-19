use http::{header, Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
	let response = Response::builder()
		.status(StatusCode::OK)
		.header(header::CONTENT_TYPE, "text")
		.body(format!("{} - /", request.method()))
		.expect("failed to render response");

	Ok(response)
}
