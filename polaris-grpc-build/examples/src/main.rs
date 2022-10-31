mod rust_grpc;

fn main() {
    let req = rust_grpc::hello::HelloReq {
        id: 1,
        name: "polaris-grpc-build demo".to_string(),
    };

    println!("id: {},name:{}", req.id, req.name);
}
