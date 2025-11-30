extern crate embed_resource;

fn main() {
    let _ = embed_resource::compile("tray-icons.rc", embed_resource::NONE);
}
