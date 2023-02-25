#[allow(dead_code)]
pub async fn step1() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let version = client
        .container()
        .from("rust:1.67-slim-bookworm")
        .with_exec(vec!["cargo", "--version"])
        .with_exec(vec!["rustc", "--version"])
        .stdout()
        .await?;

    println!("{version}");

    Ok(())
}
