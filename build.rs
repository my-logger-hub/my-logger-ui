fn main() {
    // The proto is a copy of my-logger-server/proto/MyLogger.proto - update it from there.
    // It is not downloaded here on purpose: ci-utils' download feature brings flurl ->
    // my-http-utils into the build dependencies, and that copy breaks the "server" feature
    // my-http-server needs from my-http-utils.
    ci_utils::compile_protos("proto/MyLogger.proto");
}
