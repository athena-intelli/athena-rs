use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("src/pb");
    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&[
            "proto/athenaapis/package_service.proto",
            "proto/athenaapis/lot_service.proto",
            "proto/athenaapis/unit_service.proto",
            "proto/athenaapis/work_order.proto",
            "proto/athenaapis/work_station.proto",
            "proto/athenaapis/security.proto",
            "proto/athenaapis/object_retrieval.proto",
            "proto/athenaapis/object_storage.proto",
        ], &["proto"])
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .compile(
            &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}