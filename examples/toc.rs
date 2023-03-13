use std::error::Error;

use yuque_rust::Yuque;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Yuque::builder()
        .host("https://lzzzt.yuque.com/api/v2".into())
        .token("OFg2CEeldHQjwAcq6ejnID2tOzSstNPZwxg9OhG5".into())
        .build()?;

    let repo = client.repos();

    let a = repo.get("lzzzt/ssg", None).await?.data.toc;

    println!("{:#?}", a);

    Ok(())
}
