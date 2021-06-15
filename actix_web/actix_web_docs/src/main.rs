use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{HttpServer, Responder, HttpResponse, get, App, web, Result, guard, HttpRequest};
use std::sync::Mutex;
use serde::Deserialize;
use std::path::PathBuf;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let our_app = AppData {app_name: "yay".into()};
    //web::Data::new() is creating an Arc(), which is letting us use .clone() below
    let our_mut_app = web::Data::new(MutableAppData {counter: Mutex::new(0)});

    //building an app:
    // scope = common prefix
    // service = when we want more detailed config over just .route, or when we want to register a flask-like handler
    // resource = grouping of stuff related to a single route
    // route = actual route - can take str and web obj or just web obj
    // guard = can further specify inside the route what should/not match

    HttpServer::new(move || { //automatically starts a # of http workers equal to # of logical cpu cores, but can be overwritten w/ .workers(x)
        //each worker receives a separate app instance w/o shared state - hence app factories must be S+S
        //to share state between worker threads use data / Arc as below
        //each worker processes req sequentially - so any IO bound ops within the same worker should themselves be async to speed it up
        App::new()
            .service(
                web::scope("/app") //how we create scope for routes
                    .service(hello)
                    .service(mut_hello)
                    .service( //detailed route configuration
                        web::resource("/string")
                            .route(web::get().to(hello_str1))
                            .route(web::post().to(hello_str2))
                            .route(
                                web::route()
                                    .guard(guard::Put())
                                    .to(hello_str3)
                            )
                    )
                    .service(index)
                    .service(index2)
                    .service(get_urlz)
                    .service(
                        web::resource("/test/{one}/{two}/{three}")
                            .name("foo")
                            .route(web::get().to(|| HttpResponse::Ok()))
                    )
            )
            .route("/files/{filename:.*}", web::get().to(file_handler))//serve single file from root
            .service(fs::Files::new("/actix_web", ".").show_files_listing())
            .data(our_app.clone())
            .app_data(our_mut_app.clone()) //app_data = Arc(), which is why 1) we're not passing it again, 2)we're using a mutex
    })
        .bind("127.0.0.1:5001")?
        .run() //returns instance of a server type, which we can .pause(), .resume(), .stop()
        .await
}

// this is called a "response handler" - takes 0+ params that can be extracted from a request
// happens in 2 stages: 1) responder obj is called > 2) respond_to is called
#[get("/")]
async fn hello(data: web::Data<AppData>) -> impl Responder {
    println!("{:?}", &data);
    HttpResponse::Ok().body("hey there!")
}

#[get("/mut")]
async fn mut_hello(data: web::Data<MutableAppData>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    println!("{}", counter);
    HttpResponse::Ok().body("hey there!")
}

#[derive(Debug, Clone)] //can't impl copy here, coz String inside the type is not Copy
struct AppData {
    app_name: String,
}

#[derive(Debug)]
struct MutableAppData {
    counter: Mutex<i32>
}

//by default Responder is implemented for both types of strings
// #[get("/str1")]
async fn hello_str1() -> impl Responder {
    "hello world! ref"
}

// #[get("/str2")]
async fn hello_str2() -> impl Responder {
    String::from("hello world! owned")
}

// #[get("/str3")]
async fn hello_str3() -> impl Responder {
    String::from("hello world! PUT")
}

//extractors = type-safe request information access
//actix supports up to 12 extractors, eg Path (info about the url), Query (query params), Json (deserialization of req body), Form (url encoded pararms) and other
//actix also has NOT type safe info access = .match_info()
#[get("/users/{user_id}/{other_id}")]
async fn index(web::Path((user_id, other_id)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("user_id: {}, other_id: {}", user_id, other_id))
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    other_id: String,
}

#[get("/users2/{user_id}/{other_id}")]
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!("user_id: {}, other_id: {}", info.user_id, info.other_id))
}


#[get("/get_url")]
async fn get_urlz(req: HttpRequest) -> impl Responder {
    let url = req.url_for("foo", &["1", "2", "3"]).unwrap();
    println!("{}", url);
    String::from(url)
}

// curl http://localhost:5001/files/Cargo.toml will read the file and give me back its contents over http
async fn file_handler(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)

}