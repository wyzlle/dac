use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use whois_rust::{WhoIs, WhoIsLookupOptions};

fn whois(domain: String) -> String {
    let whois = WhoIs::from_path("../servers.json").expect("Failed to load servers.json");
    let result = whois
        .lookup(WhoIsLookupOptions::from_string(domain).unwrap())
        .unwrap_or_default();
    result
}

#[post("/check")]
async fn check(req_body: String) -> impl Responder {
    let res_body = whois(req_body.clone());
    let res = !res_body.contains("Registrar IANA ID");
    println!("{} : {}", req_body, res);
    return HttpResponse::NotFound().body(res.to_string());
}

#[get("/")]
async fn ok() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ok).service(check))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
