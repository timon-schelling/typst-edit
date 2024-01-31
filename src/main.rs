use clap::Arg;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = clap::Command::new("typst-liveedit")
        .arg(Arg::new("in")
            .required(true))
        .arg(Arg::new("out"))
        .arg(Arg::new("address")
            .short('a')
            .long("address"))
        .arg(Arg::new("port")
            .short('p')
            .long("port"))
        .get_matches();

    let input = matches.get_one::<String>("in").unwrap();
    let output = match matches.get_one::<String>("out") {
        Some(o) => o.to_owned(),
        None => {
            input.strip_suffix(".typ").unwrap_or(input.clone().as_str()).to_owned() + ".pdf"
        }
    };
    let address = matches.get_one::<String>("address").map(|s| s.as_str()).unwrap_or("0.0.0.0");
    let port = matches.get_one::<String>("port").map(|s| s.as_str()).unwrap_or("8080");

    let mut typst_live = Command::new("typst-live")
        .arg("-A")
        .arg(address)
        .arg("-P")
        .arg(port)
        .arg("-T")
        .arg("-R")
        .arg(output.clone())
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let typst_live_stdout = typst_live.stdout.take().unwrap();
    let mut typst_live_reader = BufReader::new(typst_live_stdout).lines();

    let mut typst = Command::new("typst")
        .arg("watch")
        .arg(input)
        .arg(output)
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let typst_stdout = typst.stdout.take().unwrap();
    let mut typst_reader = BufReader::new(typst_stdout).lines();

    loop {
        select! {
            _line = typst_live_reader.next_line() => {},
            _line = typst_reader.next_line() => {},
        }
    }
}
