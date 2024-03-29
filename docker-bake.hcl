group "default" {
    targets = ["app"]
}

target "docker-metadata-action" {}

target "app" {
    inherits = ["docker-metadata-action"]
    platforms = ["linux/amd64", "linux/arm64"]
    args = {
        BUILD_DEPS="musl-dev pkgconfig perl build-base openssl openssl-dev git"
        RUN_DEPS="ca-certificates openssl libgcc"
        BIN_NAME="rust_tool_template"
    }
}
