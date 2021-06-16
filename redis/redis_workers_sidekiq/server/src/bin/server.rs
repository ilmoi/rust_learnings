extern crate env_logger;
extern crate sidekiq;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use sidekiq::{error_handler, Job, JobHandlerResult, panic_handler, printer_handler, retry_middleware, SidekiqServer};
use sidekiq::JobSuccessType::Success;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "example", about = "An example of Sidekiq usage.", author = "Young Wu <doomsplayer@gmail.com")]
struct Params {
    #[structopt(short = "r", long = "redis", help = "redis connection string", default_value = "redis://localhost:6379")]
    redis: String,
    #[structopt(short = "n", long = "namespace", help = "the namespace", default_value = "default")]
    namespace: String,
    #[structopt(short = "c", long = "concurrency", help = "how many workers do you want to start", default_value = "10")]
    concurrency: usize,
    #[structopt(short = "q", long = "queues", help = "the queues, in `name:weight` format, e.g. `critical:10`")]
    queues: Vec<String>,
    #[structopt(short = "t", long = "timeout", help = "the timeout when force terminated", default_value = "10")]
    timeout: usize,
}

// todo so I sort of got this working - follow the todo steps below

fn main() {
    env_logger::init();
    // let params = Params::from_args();

    //todo 1) configure this to match up with sidekiq client
    let params = Params {
        redis: "redis://localhost:6379".to_string(),
        namespace: "test".to_string(),
        concurrency: 1,
        queues: vec!["default:10".to_string()],
        timeout: 30,
    };

    let queues: Vec<_> = params.queues
        .into_iter()
        .map(|v| {
            let mut sp = v.split(':');
            let name = sp.next().unwrap();
            let weight = sp.next().unwrap().parse().unwrap();
            (name.to_string(), weight)
        })
        .collect();

    let mut server = SidekiqServer::new(&params.redis, params.concurrency).unwrap();

    // todo 2) define and attach a handler for your type of job
    // server.attach_handler("Printer", printer_handler);
    // server.attach_handler("Error", error_handler);
    server.attach_handler("MyClass", my_class_handler);

    server.attach_middleware(retry_middleware);
    for (name, weight) in queues {
        server.new_queue(&name, weight);
    }

    server.namespace = params.namespace;
    server.force_quite_timeout = params.timeout;
    start(server)
}

fn start(mut server: SidekiqServer) {
    server.start();
}

pub fn my_class_handler(_: &Job) -> JobHandlerResult {
    println!("executing MyClass job...");
    Ok(Success)
}

// todo 3)start the server with this command: RUST_LOG=sidekiq=info cargo run --bin server
// todo 4)go into client and start it with this command: cargo run --bin boss
// todo (must have redis running ofc)