use std::net::Ipv4Addr;

use insulin::DoseRequest;
use structopt::StructOpt;

mod insulin;

#[derive(StructOpt)]
enum InsulinCommand {
    #[structopt(name = "put", alias = "p")]
    Put {
        #[structopt(name = "key", alias = "k")]
        key: String,

        #[structopt(name = "ip", alias = "i")]
        ip: Ipv4Addr,

        #[structopt(name = "port", short = "p", default_value = "443")]
        port: i16,
    },

    #[structopt(name = "get", alias = "g")]
    Get {
        #[structopt(name = "key", alias = "k")]
        key: String,

        #[structopt(name = "ip", alias = "i")]
        ip: Ipv4Addr,

        #[structopt(name = "port", short = "p", default_value = "443")]
        port: i16,
    },
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => {}
        Err(_) => {
            println!("error")
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = InsulinCommand::from_args();
    let client = reqwest::Client::new();
    match args {
        InsulinCommand::Put { key, ip, port } => {
            let resp = client
                .post(format!("http://{}:{}/dose", ip, port))
                .header("x-api-key", key)
                .json(&DoseRequest { dose: 16 })
                .send()
                .await?
                .text()
                .await?;
            println!("{}", resp);
            Ok(())
        }
        InsulinCommand::Get { key, ip, port } => {
            let resp = client
                .get(format!("http://{}:{}/lastdose", ip, port))
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
