use actix_web::{get, web};
use actix_files as fs;


pub fn router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(fs::Files::new("/web", "."))
        // .service(web::resource("/node/.*").to(node_gateway))
        .service(index)
        .default_service(web::route().to(not_found));
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, permission!"
}

async fn not_found() -> &'static str {
    "Not Found"
}

// async fn node_gateway() -> &'static str {
//     "Node Gateway"
// }
```