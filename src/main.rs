mod database; 
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::database::{database_node_management, get_data_from_key, MAX_VALUE_SIZE};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = SocketAddr::from(([127, 0, 0, 1], 2904));

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle_request(req, remote_addr))) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn handle_request(
    req: Request<Body>,
    remote_addr: SocketAddr,
) -> Result<Response<Body>, Infallible> {
    info!(
        "Received request from {}: {} {}",
        remote_addr,
        req.method(),
        req.uri()
    );

    let path = req.uri().path();
    match path {
        "/query" => {
            let query = match req.uri().query() {
                Some(q) => q,
                None => return Ok(Response::new(Body::from("Missing query parameters")))
            };
            
            let params: Vec<&str> = query.split('&').collect();
            let mut key: Option<&str> = None;
            let mut value: Option<&str> = None;

            for param in params {
                let mut split = param.split('=');
                match split.next() {
                    Some("key") => key = split.next(),
                    Some("value") => value = split.next(),
                    _ => {}
                }
            }

            if let (Some(key), Some(value)) = (key, value) {
                if !key.is_empty() {
                    let was_truncated = value.len() > MAX_VALUE_SIZE;
                    let result = database_node_management(key, value);
                    
                    if result {
                        if was_truncated {
                            return Ok(Response::new(Body::from(format!(
                                "Success (value truncated to {} bytes)", MAX_VALUE_SIZE
                            ))));
                        } else {
                            return Ok(Response::new(Body::from("Success")));
                        }
                    } else {
                        return Ok(Response::new(Body::from("Invalid key characters")));
                    }
                }
            }

            return Ok(Response::new(Body::from("Invalid parameters")));
        },
        "/value" => {
            let query = match req.uri().query() {
                Some(q) => q,
                None => return Ok(Response::new(Body::from("Missing query parameters")))
            };
            
            let params: Vec<&str> = query.split('&').collect();
            let mut key: Option<&str> = None;

            for param in params {
                let mut split = param.split('=');
                match split.next() {
                    Some("key") => key = split.next(),
                    _ => {}
                }
            }

            if let Some(key) = key {
                if !key.is_empty() {
                    match get_data_from_key(key) {
                        Ok(data) => {
                            return Ok(Response::new(Body::from(data)));
                        },
                        Err(e) => {
                            return Ok(Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from(format!("Error: {}", e)))
                                .unwrap());
                        }
                    }
                }
            }

            return Ok(Response::new(Body::from("Invalid parameters")));
        },
        "/status" => {
            return Ok(Response::new(Body::from("Server is running")));
        },
        "/info" => {
            return Ok(Response::new(Body::from("This is a sample server")));
        },
        _ => {
            return Ok(Response::new(Body::from("Endpoint nonexistent")));
        },
    }
}