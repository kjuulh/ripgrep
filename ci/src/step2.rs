use dagger_sdk::HostWorkdirOpts;

pub async fn step2() -> eyre::Result<()> {
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

    let version = client
        .container()
        .from("rust:1.67-slim-bookworm")
        // Installing git
        .with_exec(vec!["apt-get", "update"])
        .with_exec(vec!["apt-get", "install", "git", "-y"])
        // Mounting source
        .with_workdir("/src/")
        .with_mounted_directory("/src/", src)
        // Building ripgrep in release mode
        .with_exec(vec!["cargo", "build", "--release", "--bin=rg"])
        // Getting rg version
        .with_exec(vec!["target/release/rg", "--version"])
        .stdout()
        .await?;

    println!("{version}");

    Ok(())
}
