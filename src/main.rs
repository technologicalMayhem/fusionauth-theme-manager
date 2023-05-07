use std::{
    fs::{self, read_to_string},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, Subcommand};
use color_eyre::Result;
use data::Metadata;
use json_data::{Root, Templates, Theme};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};
use serde_json::from_str;

mod data;
mod json_data;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// The api key to use
    api_key: String,
    /// The url of the FusionAuth server
    url: String,
    /// The path where the theme should be placed or read from
    path: String,
    /// The operation to perform
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Debug, Subcommand, Clone)]
enum Operation {
    /// Retrieve theme from FusionAuth server
    Get {
        /// The id of the theme
        id: String,
    },
    /// Insert theme into FusionAuth server
    Set,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut header_map = HeaderMap::new();
    header_map.append("Authorization", HeaderValue::from_str(&args.api_key)?);
    let client = ClientBuilder::new().default_headers(header_map).build()?;

    match args.operation {
        Operation::Get { id } => get_theme(&client, &args.url, &id, &args.path),
        Operation::Set => set_theme(&client, &args.url, &args.path),
    }
}

fn get_theme(client: &Client, url: &str, id: &str, path: &str) -> Result<()> {
    let request = client.get(format!("{url}/api/theme/{id}")).build()?;
    let response = client.execute(request)?;
    let text = response.text()?;
    let root = from_str::<Root>(&text)?;
    let theme = root.theme;

    write_theme_to_dir(path, &theme)
}

fn set_theme(client: &Client, url: &str, path: &str) -> Result<()> {
    let metadata = from_str::<Metadata>(&read_to_string(format!("{path}/metadata.json"))?)?;
    let default_messages = read_to_string(format!("{path}/defaultMessages.txt"))?;

    let root = Root {
        theme: Theme {
            default_messages,
            name: metadata.name,
            templates: read_templates(&PathBuf::from_str(&format!("{path}/templates"))?)?,
            ..Default::default()
        },
    };

    let id = metadata.id;
    let serialized = serde_json::to_string(&root)?;
    let request = client
        .put(format!("{url}/api/theme/{id}"))
        .body(serialized)
        .header(CONTENT_TYPE, "application/json")
        .build()?;
    client.execute(request)?;

    Ok(())
}

fn write_theme_to_dir(path: &str, theme: &Theme) -> Result<()> {
    let path = PathBuf::from_str(path)?;
    let templates_path = path.join("templates");

    fs::create_dir_all(&templates_path)?;

    write_file(&path, "defaultMessages.txt", &theme.default_messages)?;

    let metadata = Metadata {
        id: theme.id.clone(),
        name: theme.name.clone(),
    };

    write_file(
        &path,
        "metadata.json",
        &serde_json::to_string_pretty(&metadata)?,
    )?;
    write_templates(&templates_path, &theme.templates)?;

    Ok(())
}

// This is a bit of mess but it's dirty quick code so who cares. 
// Down the line I could make a macro that merges the creation of the write and read macro via a macro.
#[allow(clippy::too_many_lines)]
fn write_templates(templates_path: &Path, templates: &Templates) -> Result<()> {
    let templates = vec![
        ("account_index.ftl", &templates.account_index),
        ("account_edit.ftl", &templates.account_edit),
        (
            "account_two_factor_disable.ftl",
            &templates.account_two_factor_disable,
        ),
        (
            "account_two_factor_enable.ftl",
            &templates.account_two_factor_enable,
        ),
        (
            "account_two_factor_index.ftl",
            &templates.account_two_factor_index,
        ),
        (
            "account_web_authn_add.ftl",
            &templates.account_web_authn_add,
        ),
        (
            "account_web_authn_delete.ftl",
            &templates.account_web_authn_delete,
        ),
        (
            "account_web_authn_index.ftl",
            &templates.account_web_authn_index,
        ),
        ("email_complete.ftl", &templates.email_complete),
        ("email_send.ftl", &templates.email_send),
        ("email_sent.ftl", &templates.email_sent),
        (
            "email_verification_required.ftl",
            &templates.email_verification_required,
        ),
        ("email_verify.ftl", &templates.email_verify),
        ("helpers.ftl", &templates.helpers),
        ("index.ftl", &templates.index),
        ("oauth2_authorize.ftl", &templates.oauth2_authorize),
        (
            "oauth2_authorized_not_registered.ftl",
            &templates.oauth2_authorized_not_registered,
        ),
        (
            "oauth2_child_registration_not_allowed.ftl",
            &templates.oauth2_child_registration_not_allowed,
        ),
        (
            "oauth2_child_registration_not_allowed_complete.ftl",
            &templates.oauth2_child_registration_not_allowed_complete,
        ),
        (
            "oauth2_complete_registration.ftl",
            &templates.oauth2_complete_registration,
        ),
        ("oauth2_device.ftl", &templates.oauth2_device),
        (
            "oauth2_device_complete.ftl",
            &templates.oauth2_device_complete,
        ),
        ("oauth2_error.ftl", &templates.oauth2_error),
        ("oauth2_logout.ftl", &templates.oauth2_logout),
        ("oauth2_passwordless.ftl", &templates.oauth2_passwordless),
        ("oauth2_register.ftl", &templates.oauth2_register),
        (
            "oauth2_start_id_plink.ftl",
            &templates.oauth2_start_id_plink,
        ),
        ("oauth2_two_factor.ftl", &templates.oauth2_two_factor),
        (
            "oauth2_two_factor_enable.ftl",
            &templates.oauth2_two_factor_enable,
        ),
        (
            "oauth2_two_factor_enable_complete.ftl",
            &templates.oauth2_two_factor_enable_complete,
        ),
        (
            "oauth2_two_factor_methods.ftl",
            &templates.oauth2_two_factor_methods,
        ),
        ("oauth2_wait.ftl", &templates.oauth2_wait),
        ("oauth2_web_authn.ftl", &templates.oauth2_web_authn),
        (
            "oauth2_web_authn_reauth.ftl",
            &templates.oauth2_web_authn_reauth,
        ),
        (
            "oauth2_web_authn_reauth_enable.ftl",
            &templates.oauth2_web_authn_reauth_enable,
        ),
        ("password_change.ftl", &templates.password_change),
        ("password_complete.ftl", &templates.password_complete),
        ("password_forgot.ftl", &templates.password_forgot),
        ("password_sent.ftl", &templates.password_sent),
        (
            "registration_complete.ftl",
            &templates.registration_complete,
        ),
        ("registration_send.ftl", &templates.registration_send),
        ("registration_sent.ftl", &templates.registration_sent),
        (
            "registration_verification_required.ftl",
            &templates.registration_verification_required,
        ),
        ("registration_verify.ftl", &templates.registration_verify),
        ("samlv2_logout.ftl", &templates.samlv2_logout),
        ("unauthorized.ftl", &templates.unauthorized),
    ];

    for (filename, contents) in templates {
        write_file(templates_path, filename, contents)?;
    }

    Ok(())
}

fn read_templates(templates_path: &Path) -> Result<Templates> {
    let templates = Templates {
        account_index: read_to_string(templates_path.join("account_index.ftl"))?,
        account_edit: read_to_string(templates_path.join("account_edit.ftl"))?,
        account_two_factor_disable: read_to_string(
            templates_path.join("account_two_factor_disable.ftl"),
        )?,
        account_two_factor_enable: read_to_string(
            templates_path.join("account_two_factor_enable.ftl"),
        )?,
        account_two_factor_index: read_to_string(
            templates_path.join("account_two_factor_index.ftl"),
        )?,
        account_web_authn_add: read_to_string(templates_path.join("account_web_authn_add.ftl"))?,
        account_web_authn_delete: read_to_string(
            templates_path.join("account_web_authn_delete.ftl"),
        )?,
        account_web_authn_index: read_to_string(
            templates_path.join("account_web_authn_index.ftl"),
        )?,
        email_complete: read_to_string(templates_path.join("email_complete.ftl"))?,
        email_send: read_to_string(templates_path.join("email_send.ftl"))?,
        email_sent: read_to_string(templates_path.join("email_sent.ftl"))?,
        email_verification_required: read_to_string(
            templates_path.join("email_verification_required.ftl"),
        )?,
        email_verify: read_to_string(templates_path.join("email_verify.ftl"))?,
        helpers: read_to_string(templates_path.join("helpers.ftl"))?,
        index: read_to_string(templates_path.join("index.ftl"))?,
        oauth2_authorize: read_to_string(templates_path.join("oauth2_authorize.ftl"))?,
        oauth2_authorized_not_registered: read_to_string(
            templates_path.join("oauth2_authorized_not_registered.ftl"),
        )?,
        oauth2_child_registration_not_allowed: read_to_string(
            templates_path.join("oauth2_child_registration_not_allowed.ftl"),
        )?,
        oauth2_child_registration_not_allowed_complete: read_to_string(
            templates_path.join("oauth2_child_registration_not_allowed_complete.ftl"),
        )?,
        oauth2_complete_registration: read_to_string(
            templates_path.join("oauth2_complete_registration.ftl"),
        )?,
        oauth2_device: read_to_string(templates_path.join("oauth2_device.ftl"))?,
        oauth2_device_complete: read_to_string(templates_path.join("oauth2_device_complete.ftl"))?,
        oauth2_error: read_to_string(templates_path.join("oauth2_error.ftl"))?,
        oauth2_logout: read_to_string(templates_path.join("oauth2_logout.ftl"))?,
        oauth2_passwordless: read_to_string(templates_path.join("oauth2_passwordless.ftl"))?,
        oauth2_register: read_to_string(templates_path.join("oauth2_register.ftl"))?,
        oauth2_start_id_plink: read_to_string(templates_path.join("oauth2_start_id_plink.ftl"))?,
        oauth2_two_factor: read_to_string(templates_path.join("oauth2_two_factor.ftl"))?,
        oauth2_two_factor_enable: read_to_string(
            templates_path.join("oauth2_two_factor_enable.ftl"),
        )?,
        oauth2_two_factor_enable_complete: read_to_string(
            templates_path.join("oauth2_two_factor_enable_complete.ftl"),
        )?,
        oauth2_two_factor_methods: read_to_string(
            templates_path.join("oauth2_two_factor_methods.ftl"),
        )?,
        oauth2_wait: read_to_string(templates_path.join("oauth2_wait.ftl"))?,
        oauth2_web_authn: read_to_string(templates_path.join("oauth2_web_authn.ftl"))?,
        oauth2_web_authn_reauth: read_to_string(
            templates_path.join("oauth2_web_authn_reauth.ftl"),
        )?,
        oauth2_web_authn_reauth_enable: read_to_string(
            templates_path.join("oauth2_web_authn_reauth_enable.ftl"),
        )?,
        password_change: read_to_string(templates_path.join("password_change.ftl"))?,
        password_complete: read_to_string(templates_path.join("password_complete.ftl"))?,
        password_forgot: read_to_string(templates_path.join("password_forgot.ftl"))?,
        password_sent: read_to_string(templates_path.join("password_sent.ftl"))?,
        registration_complete: read_to_string(templates_path.join("registration_complete.ftl"))?,
        registration_send: read_to_string(templates_path.join("registration_send.ftl"))?,
        registration_sent: read_to_string(templates_path.join("registration_sent.ftl"))?,
        registration_verification_required: read_to_string(
            templates_path.join("registration_verification_required.ftl"),
        )?,
        registration_verify: read_to_string(templates_path.join("registration_verify.ftl"))?,
        samlv2_logout: read_to_string(templates_path.join("samlv2_logout.ftl"))?,
        unauthorized: read_to_string(templates_path.join("unauthorized.ftl"))?,
    };

    Ok(templates)
}

fn write_file(path: &Path, filename: &str, contents: &str) -> Result<()> {
    fs::File::create(path.join(filename))?.write_all(contents.as_bytes())?;
    Ok(())
}
