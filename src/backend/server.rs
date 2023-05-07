use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web, http::header::ContentType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PlanQuery {
    start_lat: f64,
    start_lon: f64,
    goal_lat: f64,
    goal_lon: f64,
}

#[post("/map")]
async fn map(req: web::Bytes) -> HttpResponse {
    let result: Result<PlanQuery, serde_json::Error> = serde_json::from_slice(&req);
    println!("{:?}", result);

    if result.is_ok() {
        HttpResponse::Accepted()
            .body("Okay")
    } else {
        HttpResponse::NotAcceptable()
            .body("Not acceptable")
    }
}

#[post("/plan")]
async fn plan() -> HttpResponse {
    println!("Start Planning");

    if true {
        HttpResponse::Accepted()
            .body("Okay")
    } else {
        HttpResponse::NotAcceptable()
            .body("Not acceptable")
    }
}

#[post("/test")]
async fn test(req: web::Bytes) -> HttpResponse {
    let result: PlanQuery = serde_json::from_slice(&req).unwrap();
    println!("{:?}", result);

    let resp = PlanQuery { start_lat: 1.0, start_lon: 2.0, goal_lat: 3.0, goal_lon: 4.0};
    HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(resp)
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Received POST on /");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn launch() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(test)
            .service(map)
            .service(plan)
    })
    .bind(("127.0.0.2", 8080))?
    .run()
    .await
}