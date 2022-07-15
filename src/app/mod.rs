#![deny(warnings)]

mod controller;
mod model;
mod lib;


use crate::app::controller::*;

// use std::{net::SocketAddr};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let url_path = req.uri().path();
    let url_method = req.method();
    info!("method:{},path:{}",url_method,url_path);
    //static file (src/public/html/1.html)
    if file_controller::check_static_url(url_path) {
        file_controller::main(file_controller::get_static_url(url_path)).await
    } else {
        // post or get
        match (url_method, url_path) {
            (&Method::POST, "/upload") => upload_controller::main(req,false).await,
            (&Method::POST, "/uploads") => upload_controller::main(req,true).await,
            (&Method::POST, "/test") => test_controller::main(req).await,
            (&Method::POST, "/param") => param_controller::main(req).await,
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("".to_string())).unwrap()),
        }
    }


}

pub async fn run(addr:SocketAddr) {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(make_svc);
    info!("server run at http://{}",addr);
    if let Err(e) = server.await { info!("server error: {}", e); }
}