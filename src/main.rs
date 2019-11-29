extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
mod config;

fn main() {

    let data = r#"
        {
            "dns-name": "bestaatniet.xyz"
            "strategy": "roundrobin",
            "port": 80,
            "type": "http2"
            "hosts": [
                "10.0.0.1:5900",
                "10.0.0.2:5900",
                "10.0.0.3:5900"
            ]
        }"#;

    
    config::start_cluster();
    

    pretty_env_logger::init();
    let addr = ([127,0,0,1], 80).into();

    let server = Server::bind(&addr)
        .serve(|| {
            // Onderzoeken
            service_fn_ok(move |_: Request<Body>| {
                Response::new(Body::from("Hello World!"))
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        rt::run(server);

        
}

