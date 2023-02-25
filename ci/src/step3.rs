use dagger_sdk::HostWorkdirOpts;
use tokio::task::JoinSet;

#[allow(dead_code)]
pub async fn step3() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let src = client.host().workdir_opts(HostWorkdirOpts {
        // Loading host dir but excluding a few files first
        exclude: Some(vec!["target/", ".git/"]),
        include: None,
    });

    let mut tasks = JoinSet::new();
    ["1.65", "1.66", "1.67"].iter().for_each(|v| {
        let v = v.clone();
        let client = client.clone();
        let src = src.clone();

        tasks.spawn(async move {
            let version = client
                .container()
                .from(format!("rust:{v}-slim-bullseye"))
                // Installing git
                .with_exec(vec!["apt-get", "update"])
                .with_exec(vec!["apt-get", "install", "git", "-y"])
                // Mounting source
                .with_workdir("/src/")
                .with_mounted_directory("/src/", src.id().await?)
                // Building ripgrep in release mode
                .with_exec(vec!["cargo", "build", "--release", "--bin=rg"])
                // Getting rg version
                .with_exec(vec!["target/release/rg", "--version"])
                .stdout()
                .await;

            return version;
        });
    });

    while let Some(Ok(version)) = tasks.join_next().await {
        println!("{}", version?);
    }

    Ok(())
}
