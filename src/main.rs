use xmlrpc::Request;

fn main() {

    let topics_request = Request::new("getSystemState").arg("/rostopic");

    let result = topics_request.call_url("http://192.168.1.22:11311");

    let system_state = result.unwrap();

    let mut topics_list: Vec<String> = vec![];
    let topic_types = &system_state[2];
    let publishers = &topic_types[0];
    for topic in publishers.as_array().unwrap() {
        match topic[0].as_str() {
            Some(topic_name) => topics_list.push(topic_name.to_string()),
            None => (),
        }
    }

    let subscribers = &topic_types[1];
    for topic in subscribers.as_array().unwrap() {
        match topic[0].as_str() {
            Some(topic_name) => topics_list.push(topic_name.to_string()),
            None => (),
        }
    }

    for topic in topics_list {
        println!("{}", topic);
    }
}
