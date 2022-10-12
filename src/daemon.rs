use core::time;
use std::thread;

use crate::insulin;

pub async fn init_daemon(
    key: String,
    ip: String,
    port: i16,
    delay: u64,
    secure: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Insulin daemon has started!");
    notifica::notify("Insulin daemon has started", "")?;
    let client = reqwest::Client::new();
    let mut last_time: i64 = 0;
    loop {
        thread::sleep(time::Duration::from_millis(delay));
        println!("Daemon is checking!");
        let resp = client
            .get(format!(
                "http{}://{}:{}/lastdose",
                {
                    if secure {
                        "s"
                    } else {
                        ""
                    }
                },
                ip,
                port
            ))
            .header("x-api-key", &key)
            .send()
            .await?
            .json::<insulin::DoseResponse>()
            .await?;
        match resp.time_until {
            Some(t) => {
                notify_time(t, last_time)?;
                last_time = t;
            }
            None => println!("None"),
        }
    }
}

fn notify_time(time: i64, last_time: i64) -> Result<(), Box<dyn std::error::Error>> {
    println!("current time: {}, previous time: {}", time, last_time);

    if time < 0 && last_time > 0 {
        notifica::notify("Insulin time", "Time to take your insulin")?;
    }

    if time < -(60 * 60) && last_time > -(60 * 60) {
        notifica::notify("Insulin time", "You are an hour late")?;
    }

    if time < -(60 * 60 * 2) && last_time > -(60 * 60 * 2) {
        notifica::notify("Insulin time", "You are 2 hours late")?;
    }

    Ok(())
}
