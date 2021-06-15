// https://medium.com/swlh/tutorial-getting-started-with-rust-and-redis-69041dd38279

use std::collections::BTreeMap;
use std::env;

use dotenv::dotenv;
use rand;
use rand::Rng;
use redis::{Commands, Connection, FromRedisValue, RedisError, RedisResult, Value};
use serde_redis::RedisDeserialize;

pub fn connect() -> redis::Connection {
    dotenv().ok();

    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);

    println!("{}", redis_conn_url);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn basics(conn: &mut Connection) {
    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(conn)
        .expect("failed to execute SET for 'foo'");

    let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);

    let _: () = conn
        .incr("counter", 2)
        .expect("failed to execute INCR for 'counter'");

    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");
    println!("counter = {}", val);
}

fn hash(conn: &mut Connection) {
    let mut driver: BTreeMap<String, String> = BTreeMap::new();
    let prefix = "redis-driver";
    driver.insert(String::from("name"), String::from("redis-rs"));
    driver.insert(String::from("version"), String::from("0.19.0"));
    driver.insert(
        String::from("repo"),
        String::from("https://github.com/mitsuhiko/redis-rs"),
    );

    let _: () = redis::cmd("HSET")
        .arg(format!("{}:{}", prefix, "rust"))
        .arg(driver)
        .query(conn)
        .expect("failed to execute HSET");

    let info: BTreeMap<String, String> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, "rust"))
        .query(conn)
        .expect("failed to execute HGETALL");
    println!("info for rust redis driver: {:?}", info);

    let _: () = conn
        .hset_multiple(
            format!("{}:{}", prefix, "go"),
            &[
                ("name", "go-redis"),
                ("version", "8.4.6"),
                ("repo", "https://github.com/go-redis/redis"),
            ],
        )
        .expect("failed to execute HSET");

    let repo_name: String = conn
        .hget(format!("{}:{}", prefix, "go"), "repo")
        .expect("HGET failed");
    println!("go redis driver repo name: {:?}", repo_name);
}

// low and high level
fn list(conn: &mut Connection) {
    let list_name = "items";

    // todo low level api
    let _: () = redis::cmd("LPUSH")
        .arg(list_name) //todo arg accepts a wide range of types
        .arg("a")
        .query(conn) //todo query allows conversion to desired type
        .expect("failed to execute LPUSH for 'items'");

    let _: () = redis::cmd("LPUSH")
        .arg(list_name)
        .arg("b")
        .query(conn)
        .expect("failed to execute LPUSH for 'items'");

    // todo high level api
    let item: String = conn
        .lpop(list_name)
        .expect("failed to execute LPOP for 'items'");
    println!("first item: {}", item);

    let _: () = conn.rpush(list_name, "c").expect("RPUSH failed");
    let _: () = conn.rpush(list_name, "d").expect("RPUSH failed");

    let len: isize = conn
        .llen(list_name)
        .expect("failed to execute LLEN for 'items'");
    println!("no. of items in list = {}", len);

    let items: Vec<String> = conn
        .lrange(list_name, 0, len - 1)
        .expect("failed to execute LRANGE for 'items'");
    println!("listing items in list");
    for item in items {
        println!("item: {}", item)
    }
}

fn set(conn: &mut Connection) {
    let set_name = "users";
    let _: () = conn
        .sadd(set_name, "user1")
        .expect("failed to execute SADD for 'users'");
    let _: () = conn
        .sadd(set_name, "user2")
        .expect("failed to execute SADD for 'users'");

    let ismember: bool = redis::cmd("SISMEMBER")
        .arg(set_name)
        .arg("user1")
        .query(conn)
        .expect("failed to execute SISMEMBER for 'users'");
    println!("does user1 exist in the set? {}", ismember);

    let users: Vec<String> = conn.smembers(set_name).expect("failed to execute SMEMBERS");
    println!("listing users in set");
    for user in users {
        println!("user: {}", user)
    }
}

pub fn sorted_set(conn: &mut Connection) {
    let sorted_set = "leaderboard";
    let _: () = redis::cmd("ZADD")
        .arg(sorted_set)
        .arg(rand::thread_rng().gen_range(1..10))
        .arg("player-1")
        .query(conn)
        .expect("failed to execute ZADD for 'leaderboard'");

    for num in 2..=5 {
        let _: () = conn
            .zadd(
                sorted_set,
                String::from("player-") + &num.to_string(),
                rand::thread_rng().gen_range(1..10),
            )
            .expect("failed to execute ZADD for 'leaderboard'");
    }

    let count: isize = conn
        .zcard(sorted_set)
        .expect("failed to execute ZCARD for 'leaderboard'");

    let leaderboard: Vec<(String, isize)> = conn
        .zrange_withscores(sorted_set, 0, count - 1)
        .expect("ZRANGE failed");
    println!("listing players and scores in ascending order");
    for item in leaderboard {
        println!("{} = {}", item.0, item.1)
    }
}

// ----------------------------------------------------------------------------- my own stuff

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub id: u32,
    pub desc: String,
    pub processing: Processing,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Processing {
    Type1,
    Type2,
}

impl Processing {
    pub fn get_processor(&self) -> fn(String) {
        match self {
            Processing::Type1 => process_type1,
            Processing::Type2 => process_type2,
        }
    }
}

pub fn process_type1(s: String) {
    println!("Task description is {}", s);
    println!("Processing ONE way!");
}

pub fn process_type2(s: String) {
    println!("Task description is {}", s);
    println!("Processing ANOTHER way!");
}

//todo manually implement FromRedisValue

// impl FromRedisValue for Task {
//     fn from_redis_value(v: &Value) -> RedisResult<Self> {
//         let t = Task {
//             id: v.id,
//             desc: v.desc,
//         };
//         RedisResult::Ok(t)
//     }
//
//     fn from_redis_values(items: &[Value]) -> RedisResult<Vec<Self>> {
//         let tasks = items
//             .into_iter()
//             .map(|v| Task {
//                 id: v.id,
//                 desc: v.desc,
//             })
//             .collect();
//         RedisResult::Ok(tasks)
//     }
// }
