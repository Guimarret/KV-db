use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{char, path};
use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::fs::File;
use std::fs::OpenOptions;


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
            let query = req.uri().query().expect(&format!("Failed to extract query from request: {:?}", req));        
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
                if let (Some(key_char), Some(value_char)) = (key.chars().next(), value.chars().next()) {
                    database_node_management(key_char, value_char);
                    return Ok(Response::new(Body::from("Success")));
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
            return Ok(Response::new(Body::from("Nah try again")));
        }
    }
}


fn database_node_management(key:char, value:char){
    let node_char:[char;36] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    if key.is_numeric() || node_char.contains(&key) {
        let file_path = key.to_string();
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&file_path)
            .expect("Unable to open or create file");

        file.write_all(value.to_string().as_bytes()).expect("Unable to write to file");
    } else {
        println!("Invalid key: {}", key);
    }
    return;

    // DB will ignore keys with lower or uper case
    // Value have to be obligatory a char not a float
    // The key will be a char from a to z or a number from 0 to 9
}