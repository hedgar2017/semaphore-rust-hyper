//!
//! The Semaphone CI pipeline demo library.
//!

use futures::{future, prelude::*};
use hyper::{Body, Request, Response, Server};

///
/// The blocking method which runs the HTTP server.
///
pub fn run(port: u16) {
    let addr = ([0, 0, 0, 0], port).into();
    println!("Starting the HTTP server at {}", addr);

    let server = Server::bind(&addr)
        .serve(|| hyper::service::service_fn(hello))
        .map_err(|error| eprintln!("The HTTP server error: {}", error));

    hyper::rt::run(server);
}

///
/// The HTTP request handler which returns hello-world responses.
///
fn hello(_request: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    let mut response = Response::new(Body::empty());
    *response.body_mut() = Body::from("Hello, World!");
    future::ok(response)
}

#[cfg(test)]
mod tests {
    ///
    /// The default unit test.
    ///
    #[test]
    fn default() {
        assert_eq!(2 + 2, 5);
    }
}
