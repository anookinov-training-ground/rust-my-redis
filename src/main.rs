use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);

    let mut buffer = File::create("bar.txt").await?;

    buffer.write_all(b"some bytes").await?;

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foobar.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
