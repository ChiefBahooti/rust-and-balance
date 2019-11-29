extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {

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

