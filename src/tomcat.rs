use crate::error::{Response, Result};
use crate::host_config::HostConfig;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Context {
    pub context_path: String,
    pub status: String,
    pub alive_session: u32,
    pub context_directory: String,
    pub context_version: Option<String>,
}

impl Context {
    fn new(
        context_path: String,
        status: String,
        alive_session: u32,
        context_directory: String,
        context_version: Option<String>,
    ) -> Self {
        Self {
            context_path,
            status,
            alive_session,
            context_directory,
            context_version,
        }
    }
}

static SPLIT_BY_NEWLINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\r\n)|(\r)").expect("Invalid regex"));

static CONTEXT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(/[a-zA-Z0-9-_%]*):(running|stopped):(\d+):([a-zA-Z0-9-_%]+)(##([0-9A-Za-z]+))?")
        .expect("Invalid regex")
});

pub fn list(config: &HostConfig) -> Result<Response> {
    let contexts = get_contexts(config)?;

    println!(
        "{0: <20} | {1: <7} | {2: <7} | {3: <20} | {4: <10}",
        "context path", "status", "session", "directory", "version"
    );
    for it in &contexts {
        let version = match &it.context_version {
            Some(v) => v,
            None => "N/A",
        };
        println!(
            "{0: <20} | {1: <7} | {2: >7} | {3: <20} | {4: <10}",
            it.context_path, it.status, it.alive_session, it.context_directory, version
        );
    }

    Ok(crate::error::Response::Ok(None))
}

fn get_contexts(config: &HostConfig) -> Result<Vec<Context>> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(config.host.join("/manager/text/list")?)
        .basic_auth(&config.user_name, Some(&config.password))
        .send()?;

    let body = response.text()?;
    let lines = SPLIT_BY_NEWLINE.split(&body).skip(1);
    let mut result = Vec::new();

    for it in lines {
        if !it.is_empty() {
            let caps = CONTEXT
                .captures(it)
                .expect("Probably regex pattern is wrong.");
            let d = Context::new(
                caps[1].to_string(),
                caps[2].to_string(),
                caps[3]
                    .parse::<u32>()
                    .expect("Probably regex pattern is wrong."),
                caps[4].to_string(),
                caps.get(6).map(|i| i.as_str().to_string()),
            );
            result.push(d);
        }
    }

    Ok(result)
}

pub fn deploy(config: &HostConfig, context_path: &str, war_file: &Path) -> Result<Response> {
    let file = File::open(war_file)?;
    let client = reqwest::blocking::Client::new();

    let response = client
        .put(config.host.join("/manager/text/deploy")?)
        .basic_auth(&config.user_name, Some(&config.password))
        .query(&[("path", context_path)])
        .body(file)
        .send()?;

    let body = response.text()?;
    Ok(handle_response(&body))
}

pub fn undeploy(config: &HostConfig, context_path: &str) -> Result<Response> {
    tomcat_generic_command("/manager/text/undeploy", config, context_path)
}

pub fn reload(config: &HostConfig, context_path: &str) -> Result<Response> {
    tomcat_generic_command("/manager/text/reload", config, context_path)
}

pub fn start(config: &HostConfig, context_path: &str) -> Result<Response> {
    tomcat_generic_command("/manager/text/start", config, context_path)
}

pub fn stop(config: &HostConfig, context_path: &str) -> Result<Response> {
    tomcat_generic_command("/manager/text/stop", config, context_path)
}

fn tomcat_generic_command(
    command: &str,
    config: &HostConfig,
    context_path: &str,
) -> Result<Response> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(config.host.join(command)?)
        .basic_auth(&config.user_name, Some(&config.password))
        .query(&[("path", context_path)])
        .send()?;

    let body = response.text()?;
    Ok(handle_response(&body))
}

fn handle_response(response: &str) -> Response {
    if response.starts_with("OK - ") {
        Response::Ok(
            response
                .strip_prefix("OK - ")
                .and_then(|s| s.strip_suffix("\r\n"))
                .map(|s| s.to_string()),
        )
    } else {
        Response::Fail(
            response
                .strip_prefix("FAIL - ")
                .and_then(|s| s.strip_suffix("\r\n"))
                .map(|s| s.to_string()),
        )
    }
}
