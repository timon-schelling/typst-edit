use clap::{command, Arg};
use std::fs::create_dir_all;
use std::io;
use std::path::Path;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(Arg::new("input").required(true).help("Input path"))
        .arg(Arg::new("output").help("Output path"))
        .arg(
            Arg::new("address")
                .long("address")
                .short('a')
                .default_value("0.0.0.0")
                .help("Network address"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("8080")
                .help("Network port"),
        )
        .arg(
            Arg::new("args")
                .trailing_var_arg(true)
                .num_args(0..)
                .help("Arguments to be passed on to typst"),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let output = match matches.get_one::<String>("output") {
        Some(o) => o.to_owned(),
        None => {
            input
                .strip_suffix(".typ")
                .unwrap_or(input.clone().as_str())
                .to_owned()
                + ".pdf"
        }
    };
    let address = matches
        .get_one::<String>("address")
        .map(|s| s.as_str())
        .unwrap();
    let port = matches
        .get_one::<String>("port")
        .map(|s| s.as_str())
        .unwrap();
    let args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .collect::<Vec<_>>();

    if let Some(parent) = Path::new(&output).parent() {
        create_dir_all(parent)?;
    }

    let mut typst_compile = spawn_typst_compile(input, &output, &args)?;
    typst_compile.wait().await?;

    let mut typst_live = spawn_typst_live(&output, address, port)?;
    let mut typst_watch = spawn_typst_watch(input, &output, &args)?;

    select! {
        _ = typst_live.wait() => {}
        _ = typst_watch.wait() => {}
    }

    Ok(())
}

fn spawn_typst_compile(input: &str, output: &str, args: &Vec<&String>) -> Result<Child, io::Error> {
    spawn_typst(input, output, false, args)
}

fn spawn_typst_watch(input: &str, output: &str, args: &Vec<&String>) -> Result<Child, io::Error> {
    spawn_typst(input, output, true, args)
}

fn spawn_typst(
    input: &str,
    output: &str,
    watch: bool,
    args: &Vec<&String>,
) -> Result<Child, io::Error> {
    Command::new("typst")
        .arg(if watch { "watch" } else { "compile" })
        .arg(input)
        .arg(output)
        .args(args)
        .stdout(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
}

fn spawn_typst_live(output: &str, address: &str, port: &str) -> Result<Child, io::Error> {
    Command::new("typst-live")
        .arg("--address")
        .arg(address)
        .arg("--port")
        .arg(port)
        .arg("--no-browser-tab")
        .arg("--no-recompile")
        .arg(output)
        .stdout(Stdio::null())
        .kill_on_drop(true)
        .spawn()
}
