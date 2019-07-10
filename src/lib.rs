use std::{thread::sleep, time::Duration};
use notify_rust::Notification;
use std::process::{Command, Stdio};
use std::env;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const AUDIOPATH: &str = "/home/kim/.assets/open-ended.mp3";

#[derive(Debug)]
pub struct Args {
    time: u64,
    msg: String,
}

impl Args {
    pub fn new() -> Result<Args> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("not enough arguments".into())
        }

        let time: u64 = args[1].parse()?;
        let msg: String = args[2..].join(" ");

        let a = Args {
            time,
            msg,
        };

        Ok(a)
    }
}

pub fn run(args: Args) -> Result<()> {
    let plural = if args.time > 1 { "s" } else { "" };
    let summary = format!("Reminder in {} minute{}", args.time, plural);
    let message = format!("Note: {}", args.msg);
    let time = Duration::from_secs(args.time);

    notify(&summary, &message)?;
    sleep(time * 60);
    notify("Reminder", &args.msg)?;

    Command::new("ffplay")
        .args(&[AUDIOPATH, "-nodisp", "-autoexit"])
        .stdout(Stdio::null())
        .output()?;

    Ok(())
}

fn notify(summary: &str, message: &str) -> Result<()> {
    Notification::new()
	.summary(summary)
	.body(message)
	.icon("time")
        .appname("RemindMeIn")
        .show()?;
    Ok(())
}
