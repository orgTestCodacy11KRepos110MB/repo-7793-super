//! Command line interface module.
//!
//! This module contains the `generate()` function, that generates the complete
//! command line for the SUPER launcher. It's also used to generate command line
//! completion scripts in the `build.rs` file.

use clap::{crate_version, App, Arg};

/// Generates the command line interface.
#[allow(clippy::too_many_lines)]
pub fn generate() -> App<'static, 'static> {
    App::new("SUPER Android Analyzer")
        .version(crate_version!())
        .author("SUPER Team <superanalyzer@pm.me>")
        .about("Audits Android apps (.apk files) for vulnerabilities")
        .arg(
            Arg::with_name("package")
                .help("The package string of the application to test")
                .value_name("package")
                .required_unless("test-all")
                .conflicts_with("test-all")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test-all")
                .short("a")
                .long("test-all")
                .conflicts_with("package")
                .conflicts_with("open")
                .help("Test all .apk files in the downloads directory"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .conflicts_with("quiet")
                .help("If you'd like the auditor to talk more than necessary"),
        )
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("If you'd like to force the auditor to do everything from the beginning"),
        )
        .arg(
            Arg::with_name("bench")
                .long("bench")
                .help("Show benchmarks for the analysis"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .conflicts_with("verbose")
                .help("If you'd like a zen auditor that won't output anything in stdout"),
        )
        .arg(
            Arg::with_name("open")
                .long("open")
                .conflicts_with("test-all")
                .help("Open the report in a browser once it is complete"),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("Generates the results in JSON format"),
        )
        .arg(
            Arg::with_name("html")
                .long("html")
                .help("Generates the results in HTML format"),
        )
        .arg(
            Arg::with_name("min_criticality")
                .long("min-criticality")
                .help("Set a minimum criticality to analyze (Critical, High, Medium, Low)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .help(
                    "Number of threads to use, by default it will use one thread per logical CPU \
                     core",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("downloads")
                .long("downloads")
                .help("Folder where the downloads are stored")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dist")
                .long("dist")
                .help("Folder where distribution files will be extracted")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("results")
                .long("results")
                .help("Folder where to store the results")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dex2jar")
                .long("dex2jar")
                .help("Where to store the jar files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("jd-cmd")
                .long("jd-cmd")
                .help("Path to the jd-cmd file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("template")
                .long("template")
                .help("Path to a results template file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rules")
                .long("rules")
                .help("Path to a JSON rules file")
                .takes_value(true),
        )
}
