use capnpc::CompilerCommand;

fn main() {
    CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/artist.capnp")
        .file("schema/option.capnp")
        .file("schema/user.capnp")
        .output_path("src")
        .run()
        .expect("schema compiler failed");
}
