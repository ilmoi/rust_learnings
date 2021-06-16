use sidekiq::{ClientOpts, create_redis_pool, Client, Job};

fn main() {
    println!("Hello, world!");

    let ns = "test";
    let client_opts = ClientOpts {
        namespace: Some(ns.to_string()),
        ..Default::default()
    };

    let pool = create_redis_pool().unwrap();
    let client = Client::new(pool, client_opts);

    let class = "MyClass".to_string();
    let job = Job::new(class, vec![sidekiq::Value::Null], Default::default());

    println!("new job id: {}", job.jid);
    println!("new job args: {:?}", job.args);
    println!("new job q: {}", job.queue);
    println!("new job created_at: {}", job.created_at);
    println!("new job enqueued_at: {}", job.enqueued_at);

    match client.push(job) {
        Ok(_) => println!("job pushed!"),
        Err(e) => println!("push failed: {}", e),
    }

}
