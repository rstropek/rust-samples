use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut file = File::create("foo.svg").await.unwrap();
    let buffer = fractal_tree::tree_svg("Hello World!", 13usize, 100f64);
    file.write_all(buffer.as_bytes()).await.unwrap();
}
