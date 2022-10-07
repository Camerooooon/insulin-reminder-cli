use structopt::StructOpt;

#[derive(StructOpt)]
enum InsulinCommand {
    #[structopt(name = "put", alias = "p")]
    Put {},

    #[structopt(name = "get", alias = "g")]
    Get {
        #[structopt(name = "key", alias = "k")]
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = InsulinCommand::from_args();
    match args {
        InsulinCommand::Put {} => todo!(),
        InsulinCommand::Get { key } => {
            let client = reqwest::Client::new();
            let resp = client
                .get("http://localhost:8000/lastdose")
                .header("x-api-key", key)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;
            println!("{:#?}", resp);
            Ok(())
        }
    }
}
