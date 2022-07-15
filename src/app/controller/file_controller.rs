
// use crate::app::lib::json_res::JsonRes;
use hyper::{Body, Response, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn main(_url:&str) -> Result<Response<Body>, hyper::Error> {
    let url = format!("{}{}","src/app/public/",_url.to_string());
    info!("{}",url);
    if let Ok(file) = File::open(url).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        Ok(Response::new(body))
    }else{
        Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("".to_string())).unwrap())
    }
}

pub fn check_static_url(path:&str)->bool{
    let file_vec :Vec<&str> = path.split("/public/").collect();
    let end_name_vec :Vec<&str> = path.split(".").collect();
    if file_vec.len()>=2  && end_name_vec.len()>1 {
        true
    }else{
        false
    }
}

pub fn get_static_url(path:&str)->&str{
    let file_vec :Vec<&str> = path.split("/public/").collect();
    file_vec[file_vec.len()-1]
}
