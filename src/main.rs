use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};
use tokio;

mod json_parser;
mod user_roles;

/// Handles the incoming data into the server and processes it based on the method and route.
///
/// # Arguments
///
/// * `req` - Request body data passed into the server
///
/// # Returns
/// Hyper response body
async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("this is a get")))
        },

        (&Method::POST, "/roles") => {
            let byte_stream = hyper::body::to_bytes(req).await?;
            let body = json_parser::json_parser::RequestBody::new(byte_stream);
            let token = body.content.get("token").unwrap().to_string();
            let role = body.content.get("role").unwrap().to_string();

            let result = user_roles::user_roles::UserRole::new(token, role);
            Ok(Response::new(Body::from(result.pack())))
        },

        (&Method::GET, "/roles") => {
            let byte_stream = hyper::body::to_bytes(req).await?;
            let body = json_parser::json_parser::RequestBody::new(byte_stream);
            let token = body.content.get("token").unwrap().to_string();

            let result = user_roles::user_roles::UserRole::get(token);
            Ok(Response::new(Body::from(result.pack())))
        },

        _ => {
            Ok(Response::new(Body::from("Rust Microservice caught")))
        }
    }
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
