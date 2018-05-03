use sample::*;
use sample_grpc::{Greeter, create_greeter};
use grpcio::{EnvBuilder, RpcContext, ServerBuilder, UnarySink, Server};
use std::sync::Arc;

#[derive(Clone)]
struct SampleService;
impl Greeter for SampleService {
    fn hello(&self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloResponse>) {

    }
}

pub fn start_server() -> Server {
    let env = Arc::new(EnvBuilder::new().build());
    let sample_service = create_greeter(SampleService);
    let mut server = ServerBuilder::new(env)
        .register_service(sample_service)
        .bind("127.0.0.1", 12354)
        .build()
        .expect("build failed");
    server.start();
    server
}