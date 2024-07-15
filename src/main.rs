use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::warn;
use std::char;
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, Level, Value};
use tracing_subscriber::FmtSubscriber;

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
    Ok(Response::new(Body::from("Ok!")))
}

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
#[allow(dead_code)] 

fn database_node_management(key:char, value:char){
    let node_char:[char;36] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // DB will ignore keys with lower or uper case
    // Value have to be obligatory a char not a float
    
}