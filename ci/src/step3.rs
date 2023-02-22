use dagger_sdk::HostWorkdirOpts;

pub async fn step3() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let src = client
        .host()
        .workdir_opts(HostWorkdirOpts {
            // Loading host dir but excluding a few files first
            exclude: Some(vec!["target/", ".git/"]),
            include: None,
        })
        .id()
        .await?;

    let rust = client
        .container()
        .from("rust:1.67-slim-bookworm")
        // Installing git
        .with_exec(vec!["apt-get", "update"])
        .with_exec(vec!["apt-get", "install", "git", "-y"])
        // Mounting source
        .with_mounted_directory("/src", src)
        .with_workdir("/src");

    let rg = rust
        // Building ripgrep in release mode
        .with_exec(vec!["cargo", "build", "--release", "--bin=rg"])
        // Retreive the binary
        .file("target/release/rg");

    let doc_src = client.host().directory("doc").id().await?;

    let entries = client
        .container()
        .from("debian:bookworm-slim")
        .with_file("/usr/bin/rg", rg.id().await?)
        .with_mounted_directory("/src/doc", doc_src)
        .with_workdir("/src/doc")
        .with_exec(vec!["/usr/bin/rg", "ripgrep"])
        .stdout()
        .await?;

    println!("{entries}");

    Ok(())
}
