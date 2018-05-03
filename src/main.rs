extern crate protobuf;
extern crate grpcio;
extern crate futures;

mod sample;
mod sample_grpc;
mod server;

use std::{
    thread,
    time::Duration,
    sync::Arc,
};
use grpcio::{ChannelBuilder, Environment, CallOption};
use sample::HelloRequest;
use sample_grpc::GreeterClient;
// use server::SampleServer;

fn main() {
    thread::spawn(move || {
        let mut server = server::start_server();
        println!("{:?}", server);
    });
    thread::sleep(Duration::from_millis(5000));

    let env = Arc::new(Environment::new(1));
    let mut ch = ChannelBuilder::new(env.clone()).connect("127.0.0.1:12354");
    let mut client = GreeterClient::new(ch);
    let mut req = HelloRequest::new();
    let res = client.hello_opt(&req, CallOption::default());

    println!("{:?}", res);
}
