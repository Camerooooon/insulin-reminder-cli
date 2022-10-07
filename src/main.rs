use insulin::DoseRequest;
use structopt::StructOpt;

mod insulin;

#[derive(StructOpt)]
enum InsulinCommand {
    #[structopt(name = "put", alias = "p")]
    Put {
        #[structopt(name = "key", alias = "k")]
        key: String,
    },

    #[structopt(name = "get", alias = "g")]
    Get {
        #[structopt(name = "key", alias = "k")]
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = InsulinCommand::from_args();
    let client = reqwest::Client::new();
    match args {
        InsulinCommand::Put { key } => {
            let resp = client
                .post("http://localhost:8000/dose")
                .header("x-api-key", key)
                .json(&DoseRequest { dose: 16 })
                .send()
                .await?
                .text()
                .await?;
            println!("{}", resp);
            Ok(())
        }
        InsulinCommand::Get { key } => {
            let resp = client
                .get("http://localhost:8000/lastdose")
                .header("x-api-key", key)
                .send()
                .await?
                .json::<insulin::DoseResponse>()
                .await?;
            match resp.time_until {
                Some(t) => println!("{:.2}hrs", (t as f64 / 60.0 / 60.0)),
                None => println!("None"),
            }
            Ok(())
        }
    }
}
