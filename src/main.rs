mod options;
mod utils;

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use anyhow::{bail, Context, Result};
use structopt::StructOpt;

use options::Opt;

fn main() -> Result<()> {
    let opt: Arc<Opt> = Arc::new(Opt::from_args());

    create_dir_all(&opt.output).context(format!(
        "Failed to create directory `{}`",
        opt.output.display()
    ))?;
    match utils::dir_is_writable(&opt.output) {
        Ok(false) | Err(_) => {
            bail!(format!("`{}` is not writeable", opt.output.display()));
        }
        _ => {}
    }

    let listener = TcpListener::bind(("0.0.0.0", 9999))?;

    for stream in listener.incoming() {
        if stream.is_err() {
            continue;
        }
        let stream = stream.unwrap();
        let opt = Arc::clone(&opt);
        thread::spawn(move || handle_client(stream, opt));
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, settings: Arc<Opt>) {
    let mut buffer = vec![0; settings.buffer_size];

    let read = stream.read(&mut buffer).unwrap();

    let mut file = settings.output.clone();
    let name = loop {
        let name = utils::gen_name();
        file.push(&name);
        if !file.is_file() {
            break name;
        }
        file.pop();
    };
    let mut file = File::create(file).unwrap();
    file.write_all(&buffer[..read]).unwrap();

    let mut addr = if settings.https {
        String::from("https://")
    } else {
        String::from("http://")
    };
    addr.push_str(&settings.domain);
    if !settings.domain.ends_with('/') {
        addr.push('/');
    }
    addr.push_str(name.as_str());
    addr.push('\n');

    stream.write_all(addr.as_ref()).unwrap();
}
