use std::convert::Infallible;
use std::net::SocketAddr;
use serde_json::json;
use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};
use tokio;

mod json_parser;
mod user_roles;


async fn handle(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    match (_req.method(), _req.uri().path()) {

        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("this is a get")))
        },

        (&Method::POST, "/roles") => {
            let byte_stream = hyper::body::to_bytes(_req).await?;
            let body = json_parser::json_parser::RequestBody::new(byte_stream);
            let token = body.content.get("token").unwrap().to_string();
            let role = body.content.get("role").unwrap().to_string();

            let result = user_roles::user_roles::UserRole::new(token, role);

//            let response_json = json!({
//                "role": result.role,
//                "success": result.success,
//                "message": result.message
//            });
//            return Ok(Response::new(result))
//            let role: &str = &result.role;
            let pool = futures_fs::FsPool::default();
//            return Ok(Response::new(Body::from("Rust POST method executed")))
            return Ok(Response::new(Body::from(role)))
        },

        (&Method::GET, "/roles") => {
            let byte_stream = hyper::body::to_bytes(_req).await?;
            let body = json_parser::json_parser::RequestBody::new(byte_stream);
            let token = body.content.get("token").unwrap().to_string();

            let _ = user_roles::user_roles::UserRole::get(token);

            Ok(Response::new(Body::from("Rust GET method executed")))
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
