use hyper::{header::CONTENT_TYPE, Body, Request, Response};
use multer::{Multipart};
use std::path::Path;
use std::fs::{DirBuilder, File};
use std::io::Write;
use futures::stream;
use serde::{Serialize};
use crate::app::lib::json_res::JsonRes;
use chrono::Local;

pub async fn main(req: Request<Body>,is_mult:bool) -> Result<Response<Body>, hyper::Error> {
    let boundary = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok());
    if boundary.is_none() {
        JsonRes::new(-1, "failed multipart/form-data supports only!".to_string(), "")
    } else {    // parse the multipart from request's body
        let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
        if is_mult {
            if full_body.len() > 20 * 1024 * 1024 + 400 {
                JsonRes::new(-2, "all upload files limit 20M".to_string(), "")
            } else {
                let body_vec: Vec<Result<Vec<u8>, std::io::Error>> = vec![Ok(full_body.to_vec())];

                let mut multipart = Multipart::new(stream::iter(body_vec), boundary.unwrap());

                let mut result: Vec<FileObject> = vec![];

                while let Some(mut field) = multipart.next_field().await.expect("multipart error") {
                    let name = field.name().unwrap().to_owned();
                    let filename = field.file_name().unwrap().to_owned();

                    info!("{:?}:{:?}", name, filename);

                    let mut content = Vec::new();
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        content.extend(chunk.to_vec());
                    }

                    info!("file size = {:?}", content.len());

                    let mut file = FileObject::new(filename, name, "".to_owned());
                    match file.save_file(&content) {
                        Ok(_) => {
                            info!("{:?} save done!", file.uri);
                            result.push(file);
                        }
                        Err(e) => {
                            info!("something went wrong: {:?}!", e.to_string());
                            file.uri = e.to_string();
                            result.push(file);
                        }
                    }
                }
                JsonRes::new(0, "success".to_string(), Some(result))
            }
        }else{
            if full_body.len() > 2 * 1024 * 1024 + 400 {
                JsonRes::new(-2, "all upload files limit 2M".to_string(), "")
            } else {
                let body_vec: Vec<Result<Vec<u8>, std::io::Error>> = vec![Ok(full_body.to_vec())];

                let mut multipart = Multipart::new(stream::iter(body_vec), boundary.unwrap());


                 if let Some(mut field) = multipart.next_field().await.expect("multipart error") {
                    let name = field.name().unwrap().to_owned();
                    let filename = field.file_name().unwrap().to_owned();

                    info!("{:?}:{:?}", name, filename);

                    let mut content = Vec::new();
                    while let Some(chunk) = field.chunk().await.unwrap() {
                        content.extend(chunk.to_vec());
                    }

                    info!("file size = {:?}", content.len());

                    let mut file = FileObject::new(filename, name, "".to_owned());
                    match file.save_file(&content) {
                        Ok(_) => {
                            info!("{:?} save done!", file.uri);
                            JsonRes::new(0, "success".to_string(), file)
                        }
                        Err(e) => {
                            info!("something went wrong: {:?}!", e.to_string());
                            JsonRes::new(-1, e.to_string(), "")
                        }
                    }
                }else {
                     JsonRes::new(-1, "failed".to_string(), "")
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
struct FileObject {
    filename: String,
    name: String,
    uri: String,
}

impl FileObject {
    fn new(filename: String, name: String, uri: String) -> Self {
        Self {
            filename,
            name,
            uri,
        }
    }


    fn save_file(&mut self, content: &[u8]) -> Result<(), std::io::Error> {
        let file_ext = Path::new(&self.filename).extension().unwrap().to_str().unwrap().to_lowercase();
        let file_dir = Local::now().format("%Y-%m-%d").to_string();
        let dir_path = Path::new("./src/app/public/upload").join(&file_dir);
        let filename = format!("{}.{}", Local::now().timestamp_millis(), file_ext);
        DirBuilder::new().recursive(true).create(&dir_path).unwrap();
        self.uri = format!("{}/{}/{}", "/public/upload", file_dir, &filename);
        let path = dir_path.join(&filename);
        if path.exists() {
            return Ok(());
        }
        match File::create(&path) {
            Ok(mut f) => {
                f.write_all(content)?;
                Ok(())
            }
            Err(e) => {
                let err = std::io::Error::new(std::io::ErrorKind::Other, e.to_string());
                info!("{:?}", err);
                return Err(err);
            }
        }
    }
}
