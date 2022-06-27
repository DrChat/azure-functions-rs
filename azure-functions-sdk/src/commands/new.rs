use crate::{
    commands::TEMPLATES,
    util::{
        create_from_template, last_segment_in_path, print_failure, print_running, print_success,
    },
};
use atty::Stream;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::Colorize;
use regex::Regex;
use serde_json::{json, Value};
use std::{
    fs::{remove_file, File},
    io::Read,
    path::Path,
};
use syn::{self, parse::Parser, parse_file, punctuated::Punctuated, Item, Token};

mod activity;
mod blob;
mod cosmos_db;
mod event_grid;
mod event_hub;
mod http;
mod orchestration;
mod queue;
mod service_bus;
mod timer;

pub use self::activity::Activity;
pub use self::blob::Blob;
pub use self::cosmos_db::CosmosDb;
pub use self::event_grid::EventGrid;
pub use self::event_hub::EventHub;
pub use self::http::Http;
pub use self::orchestration::Orchestration;
pub use self::queue::Queue;
pub use self::service_bus::ServiceBus;
pub use self::timer::Timer;

fn get_path_for_function(name: &str) -> Result<String, String> {
    if !Regex::new("^[a-zA-Z][a-zA-Z0-9_]*$")
        .unwrap()
        .is_match(name)
    {
        return Err("Function name must start with a letter and only contain letters, numbers, and underscores.".to_string());
    }

    if name.len() > 127 {
        return Err("Function names cannot exceed 127 characters.".to_string());
    }

    if !Path::new("src/functions").is_dir() {
        return Err("Directory 'src/functions' does not exist.".to_string());
    }

    let path = format!("src/functions/{}.rs", name);
    if Path::new(&path).exists() {
        return Err(format!("'{}' already exists.", path));
    }

    Ok(path)
}

fn create_function(name: &str, template: &str, data: &Value, quiet: bool) -> Result<(), String> {
    let path = get_path_for_function(name)?;

    if !quiet {
        print_running(&format!("creating {}.", path.cyan()));
    }

    create_from_template(&TEMPLATES, template, "", &path, data)
        .map(|_| {
            if !quiet {
                print_success();
            }
        })
        .map_err(|e| {
            if !quiet {
                print_failure();
            }
            e
        })?;

    if !quiet {
        print_running(&format!(
            "exporting function {} in {}.",
            name.cyan(),
            "src/functions/mod.rs".cyan()
        ));
    }

    export_function(name)
        .map(|_| {
            if !quiet {
                print_success();
            }
        })
        .map_err(|e| {
            if !quiet {
                print_failure();
            }
            remove_file(path).expect("failed to delete source file");
            e
        })?;

    Ok(())
}

fn format_path(path: &syn::Path) -> String {
    use std::fmt::Write;

    let mut formatted = String::new();
    if path.leading_colon.is_some() {
        formatted.push_str("::");
    }

    let mut first = true;
    for segment in &path.segments {
        if first {
            first = false;
        } else {
            formatted.push_str("::");
        }

        write!(formatted, "{}", segment.ident).unwrap();
    }

    formatted
}

fn export_function(name: &str) -> Result<(), String> {
    let mut file =
        File::open("src/functions/mod.rs").map_err(|_| "'src/functions/mod.rs' does not exist.")?;

    let mut source = String::new();
    file.read_to_string(&mut source)
        .map_err(|_| "failed to read 'src/functions/mod.rs'.")?;

    let ast = parse_file(&source).map_err(|_| "failed to parse 'src/functions/mod.rs'.")?;

    let mut modules = Vec::new();
    let mut exports = Vec::new();

    for item in ast.items {
        match item {
            Item::Mod(m) => {
                modules.push(m.ident.to_string());
            }
            Item::Macro(m) => {
                if last_segment_in_path(&m.mac.path).ident == "export" {
                    exports.extend(
                        Punctuated::<syn::Path, Token![,]>::parse_terminated
                            .parse2(m.mac.tokens)
                            .map_err(|e| format!("failed to parse 'export!' macro: {}", e))?
                            .into_iter()
                            .map(|p| format_path(&p)),
                    );
                }
            }
            _ => {}
        }
    }

    modules.push(name.to_string());
    modules.sort();

    exports.push(format!("{}::{}", name, name));
    exports.sort();

    create_from_template(
        &TEMPLATES,
        "functions_mod.rs",
        "",
        "src/functions/mod.rs",
        &json!({
            "modules": modules,
            "exports": exports
        }),
    )
}

pub struct New<'a> {
    quiet: bool,
    color: Option<&'a str>,
    args: &'a ArgMatches,
}

impl<'a> New<'a> {
    pub fn create_subcommand() -> App<'static> {
        SubCommand::with_name("new")
            .about("Creates a new Azure Function from a template.")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .arg(
                Arg::with_name("quiet")
                    .long("quiet")
                    .short('q')
                    .help("No output printed to stdout."),
            )
            .arg(
                Arg::with_name("color")
                    .long("color")
                    .value_name("WHEN")
                    .help("Controls when colored output is enabled.")
                    .possible_values(&["auto", "always", "never"])
                    .default_value("auto"),
            )
            .subcommand(Blob::create_subcommand())
            .subcommand(Http::create_subcommand())
            .subcommand(Queue::create_subcommand())
            .subcommand(Timer::create_subcommand())
            .subcommand(EventGrid::create_subcommand())
            .subcommand(EventHub::create_subcommand())
            .subcommand(CosmosDb::create_subcommand())
            .subcommand(ServiceBus::create_subcommand())
            .subcommand(Activity::create_subcommand())
            .subcommand(Orchestration::create_subcommand())
    }

    fn set_colorization(&self) {
        ::colored::control::set_override(match self.color {
            Some("auto") | None => ::atty::is(Stream::Stdout),
            Some("always") => true,
            Some("never") => false,
            _ => panic!("unsupported color option"),
        });
    }

    pub fn execute(&self) -> Result<(), String> {
        self.set_colorization();

        match self.args.subcommand() {
            Some(("blob", args)) => Blob::from(args).execute(self.quiet),
            Some(("http", args)) => Http::from(args).execute(self.quiet),
            Some(("queue", args)) => Queue::from(args).execute(self.quiet),
            Some(("timer", args)) => Timer::from(args).execute(self.quiet),
            Some(("event-grid", args)) => EventGrid::from(args).execute(self.quiet),
            Some(("event-hub", args)) => EventHub::from(args).execute(self.quiet),
            Some(("cosmos-db", args)) => CosmosDb::from(args).execute(self.quiet),
            Some(("service-bus", args)) => ServiceBus::from(args).execute(self.quiet),
            Some(("activity", args)) => Activity::from(args).execute(self.quiet),
            Some(("orchestration", args)) => Orchestration::from(args).execute(self.quiet),
            _ => panic!("expected a subcommand for the 'new' command."),
        }
    }
}

impl<'a> From<&'a ArgMatches> for New<'a> {
    fn from(args: &'a ArgMatches) -> Self {
        New {
            quiet: args.is_present("quiet"),
            color: args.value_of("color"),
            args,
        }
    }
}
