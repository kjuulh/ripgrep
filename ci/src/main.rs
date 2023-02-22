mod step1;
mod step2;
mod step3;
mod step4;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    //step1::step1().await?;
    //step2::step2().await?;
    //step3::step3().await?;
    step4::step4().await?;

    Ok(())
}
