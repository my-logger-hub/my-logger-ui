fn main() {
    ci_utils::sync_and_build_proto_file_from_private_github_repo(
        "my-logger-hub",
        "my-logger-server",
        "proto/MyLogger.proto",
    );

    ci_utils::tonic_build::compile_protos("proto/MyLogger.proto").unwrap();
}
