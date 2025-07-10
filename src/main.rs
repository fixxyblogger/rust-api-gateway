use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use reqwest::Client;
use reqwest::header::{HeaderMap as ReqwestHeaderMap, HeaderName, HeaderValue};

async fn proxy(req: HttpRequest, body: web::Bytes, client: web::Data<Client>) -> impl Responder {
    let path = req
        .uri()
        .path_and_query()
        .map(|p| p.as_str())
        .unwrap_or("/");

    let main_res = "Response from API Gateway: ";

    // Expected format: /api/{service}/{...}
    let path_segments: Vec<&str> = path.trim_start_matches("/api/").split('/').collect();

    if path_segments.is_empty() || path_segments[0].is_empty() {
        return HttpResponse::BadRequest().body(format!("{}Invalid API path", main_res));
    }

    let service = path_segments[0];
    let remaining_path = &path_segments[1..].join("/");

    // Map service to base URL
    let target_base = match service {
        "srp" => "http://localhost:3000",
        _ => return HttpResponse::BadGateway().body(format!("{}Unknown service", main_res)),
    };

    let target_url = format!("{}/{}", target_base, remaining_path);

    // Convert Actix headers to Reqwest headers
    let mut headers = ReqwestHeaderMap::new();
    for (name, value) in req.headers().iter() {
        if let Ok(header_name) = HeaderName::from_bytes(name.as_str().as_bytes()) {
            if header_name != HeaderName::from_static("host") {
                if let Ok(header_value) = HeaderValue::from_bytes(value.as_bytes()) {
                    headers.insert(header_name, header_value);
                }
            }
        }
    }

    // Build and send the request
    let forwarded_req = client
        .request(req.method().clone(), &target_url)
        .headers(headers)
        .body(body)
        .send()
        .await;

    // Handle the response
    match forwarded_req {
        Ok(res) => {
            let mut client_resp = HttpResponse::build(res.status());

            for (key, value) in res.headers() {
                client_resp.append_header((key.clone(), value.clone()));
            }

            match res.bytes().await {
                Ok(bytes) => client_resp.body(bytes),
                Err(_) => HttpResponse::InternalServerError()
                    .body(format!("{}Failed to read upstream body", main_res)),
            }
        }
        Err(err) => {
            eprintln!("Gateway error: {}", err);
            HttpResponse::InternalServerError().body(format!("{}Upstream request failed", main_res))
        }
    }
}

#[get("/hc")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("API Gateway is healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new();

    println!("🚀 API Gateway running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(Cors::permissive()) // Allow all CORS for now
            .service(health_check) // Health check endpoint
            .default_service(web::to(proxy)) // Proxy all requests
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
