use http::{
	self,
	Request,
	Response,
	StatusCode
};

fn handler(req: Request<()>) -> http::Result<Response<&'static str>> {
	Response::builder()
		.header("Content-Type", "text/plain")
		.status(StatusCode::OK)
		.body("Hello!")
}
