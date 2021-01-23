use xmlrpc::{Request, Value};

fn main() {

    let topics_request = Request::new("getSystemState").arg("/rostopic");

    let result = topics_request.call_url("http://192.168.1.22:11311");

    println!("Topics: {:#?}", result.unwrap()[2]);
}
