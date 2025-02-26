use std::{
    error::Error,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
};

use crate::{config::Config, server::sock_path};

pub fn refresh(
    _conf: Config,
    cache_path: &Path,
    accounts: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let sock_path = sock_path(cache_path);
    let mut errs = Vec::new();
    for act_name in accounts {
        let mut stream = UnixStream::connect(&sock_path)
            .map_err(|_| "pizauth authenticator not running or not responding")?;
        stream
            .write_all(format!("refresh {act_name:}").as_bytes())
            .map_err(|_| "Socket not writeable")?;
        stream.shutdown(Shutdown::Write)?;

        let mut rtn = String::new();
        stream.read_to_string(&mut rtn)?;
        match rtn.splitn(2, ':').collect::<Vec<_>>()[..] {
            ["ok", ""] => (),
            ["error", cause] => errs.push(format!("{act_name}:{cause:}")),
            ["pending", ""] => errs.push(format!(
                "{act_name:}: Token unavailable until authentication complete"
            )),
            _ => errs.push(format!("{act_name:}: Malformed response '{rtn:}'")),
        }
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(errs.join("\n").into())
    }
}

pub fn reload(_conf: Config, conf_path: PathBuf, cache_path: &Path) -> Result<(), Box<dyn Error>> {
    let sock_path = sock_path(cache_path);
    let mut stream = UnixStream::connect(&sock_path)
        .map_err(|_| "pizauth authenticator not running or not responding")?;
    stream
        .write_all(
            format!(
                "reload {}",
                conf_path
                    .as_os_str()
                    .to_str()
                    .ok_or("Unencodable file name")?
            )
            .as_bytes(),
        )
        .map_err(|_| "Socket not writeable")?;
    stream.shutdown(Shutdown::Write)?;

    let mut rtn = String::new();
    stream.read_to_string(&mut rtn)?;
    match rtn.splitn(2, ':').collect::<Vec<_>>()[..] {
        ["ok", ""] => Ok(()),
        ["error", cause] => Err(cause.into()),
        _ => Err(format!("Malformed response '{rtn:}'").into()),
    }
}

pub fn show_token(_conf: Config, cache_path: &Path, account: &str) -> Result<(), Box<dyn Error>> {
    let sock_path = sock_path(cache_path);
    let mut stream = UnixStream::connect(&sock_path)
        .map_err(|_| "pizauth authenticator not running or not responding")?;
    stream
        .write_all(format!("showtoken {account:}").as_bytes())
        .map_err(|_| "Socket not writeable")?;
    stream.shutdown(Shutdown::Write)?;

    let mut rtn = String::new();
    stream.read_to_string(&mut rtn)?;
    match rtn.splitn(2, ':').collect::<Vec<_>>()[..] {
        ["access_token", x] => {
            println!("{x:}");
            Ok(())
        }
        ["pending", ""] => Err("Token unavailable until authentication complete".into()),
        ["error", cause] => Err(cause.into()),
        _ => Err(format!("Malformed response '{rtn:}'").into()),
    }
}

pub fn shutdown(
    _conf: Config,
    _conf_path: PathBuf,
    cache_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let sock_path = sock_path(cache_path);
    let mut stream = UnixStream::connect(&sock_path)
        .map_err(|_| "pizauth authenticator not running or not responding")?;
    stream
        .write_all(b"shutdown")
        .map_err(|_| "Socket not writeable")?;
    Ok(())
}
