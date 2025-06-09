fn main() {
    let url =
        "https://raw.githubusercontent.com/my-logger-hub/my-logger-server/refs/heads/main/proto/";
    ci_utils::sync_and_build_proto_file(url, "MyLogger.proto");
}
