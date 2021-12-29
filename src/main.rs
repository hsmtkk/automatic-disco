mod line;

fn main() {
    let channel_access_token = std::env::var("CHANNEL_ACCESS_TOKEN").expect("CHANNEL_ACCESS_TOKEN environment variable must be defined");
    let client = line::Client::new(&channel_access_token);
    client.broadcast(vec!["charlie", "delta"]).expect("broadcast");
}
