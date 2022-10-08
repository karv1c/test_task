use crate::handler::handler;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::sync::Arc;
use std::{env, error::Error, path::Path};

mod handler;
mod parameter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let img_path: Arc<String> = Arc::new(args[1].to_owned());
        if Path::new(&*img_path.clone()).exists() {
            let make_svc = make_service_fn(move |_conn| {
                let img_path_clone = img_path.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req| handler(req, img_path_clone.clone())))
                }
            });
            let addr = ([127, 0, 0, 1], 8080).into();
    
            let server = Server::bind(&addr).serve(make_svc);
    
            println!("Listening on http://{}", addr);
    
            server.await?;
        } else {
            println!("File doesn't exist")
        }
        
        
    } else {
        println!("Program accepts only 1 argument: image file path");
    }
    Ok(())
}
