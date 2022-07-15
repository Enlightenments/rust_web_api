use hyper::{Request, Body};
use std::collections::HashMap;


pub async fn post_param(req: Request<Body>)-> Result<HashMap<String,String>, hyper::Error>{
    let b = hyper::body::to_bytes(req).await?;
    println!("{:?}",b);
    let params = url::form_urlencoded::parse(b.as_ref()).into_owned().collect::<HashMap<String, String>>();
    println!("params:{:?}",params);
    Ok(params)
}
