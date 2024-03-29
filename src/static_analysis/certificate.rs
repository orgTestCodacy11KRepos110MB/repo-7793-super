//! This module performs the static analysis of the certificate of the
//! application.

use crate::{
    criticality::Criticality,
    print_vulnerability, print_warning,
    results::{Results, Vulnerability},
    Config,
};
use anyhow::{bail, Context, Error};
use chrono::{Datelike, Local};
use colored::Colorize;
use std::{fs, process::Command};

/// Parses the given month string.
///
/// It will convert it to an integer representing the month number in the year.
fn parse_month<S: AsRef<str>>(month_str: S) -> u32 {
    match month_str.as_ref() {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => 0,
    }
}

/// Performs the certificate analysis.
///
/// *Note: This requires OpenSSL.*
pub fn analysis<S: AsRef<str>>(
    config: &Config,
    package: S,
    results: &mut Results,
) -> Result<(), Error> {
    if config.is_verbose() {
        println!("Reading and analyzing the certificates.")
    }

    // Gets the path to the certificate files.
    let path = config
        .dist_folder()
        .join(package.as_ref())
        .join("original")
        .join("META-INF");
    let dir_iter = fs::read_dir(&path)?;

    for f in dir_iter {
        let f = match f {
            Ok(f) => f,
            Err(e) => {
                print_warning(format!(
                    "An error occurred when reading the {} dir searching certificates. \
                     Certificate analysis will be skipped. More info: {}",
                    path.display(),
                    e
                ));
                break;
            }
        };

        let path_file = f
            .path()
            .file_name()
            .map(|n| n.to_os_string().into_string().unwrap())
            .unwrap_or_default();

        let mut is_cert = false;
        if let Some(e) = f.path().extension() {
            if e.to_string_lossy() == "RSA" || e.to_string_lossy() == "DSA" {
                is_cert = true;
            }
        }

        // We found a certificate, let's get its information.
        if is_cert {
            let output = Command::new("openssl")
                .arg("pkcs7")
                .arg("-inform")
                .arg("DER")
                .arg("-in")
                .arg(f.path().to_str().unwrap())
                .arg("-noout")
                .arg("-print_certs")
                .arg("-text")
                .output()
                .context(
                    "there was an error when executing the openssl command to check the \
                     certificate",
                )?;

            if !output.status.success() {
                bail!(
                    "the openssl command returned an error. More info: {}",
                    String::from_utf8_lossy(&output.stderr[..])
                );
            };

            let cmd = String::from_utf8_lossy(&output.stdout);
            if config.is_verbose() {
                println!(
                    "The application is signed with the following certificate: {}",
                    path_file.bold()
                );

                println!("{}", cmd);
            }

            // Get the information we need.
            let mut issuer = String::new();
            let mut subject = String::new();
            let mut after = String::new();
            for line in cmd.lines() {
                if line.contains("Issuer:") {
                    issuer = line.to_owned();
                }
                if line.contains("Subject:") {
                    subject = line.to_owned();
                }
                if line.contains("Not After :") {
                    after = line.to_owned();
                }
            }

            results.set_certificate(cmd);

            let mut issuer = issuer.split(": ");
            let mut subject = subject.split(": ");
            let mut after = after.split(": ");

            // Detect Android debug certificate.
            if issuer.nth(1).unwrap().contains("Android Debug") {
                let criticality = Criticality::Critical;
                let description = "The application is signed with the Android Debug Certificate. \
                                   This certificate should never be used for publishing an app.";

                let vulnerability = Vulnerability::new(
                    criticality,
                    "Android Debug Certificate",
                    description,
                    None::<String>,
                    None,
                    None,
                    None::<String>,
                );
                results.add_vulnerability(vulnerability);
                print_vulnerability(description, criticality);
            }
            if issuer.nth(1) == subject.nth(1) {
                // TODO: This means it is self signed. Should we do something?
            }

            // Check certificate expiration.
            let now = Local::now();
            let year = now.year();
            let month = now.month();
            let day = now.day();

            let after = after.nth(1).unwrap();
            let cert_year = after[16..20].parse::<i32>().unwrap();
            let cert_month = parse_month(&after[0..3]);
            let cert_day = match after[4..6].parse::<u32>() {
                // if day<10 parse 1 number
                Ok(n) => n,
                Err(_) => after[5..6].parse::<u32>().unwrap(),
            };

            if year > cert_year
                || (year == cert_year && month > cert_month)
                || (year == cert_year && month == cert_month && day > cert_day)
            {
                let criticality = Criticality::High;
                let description = "The certificate of the application has expired. You should not \
                                   use applications with expired certificates since the app is \
                                   not secure anymore.";

                let vulnerability = Vulnerability::new(
                    criticality,
                    "Expired certificate",
                    description,
                    None::<String>,
                    None,
                    None,
                    None::<String>,
                );
                results.add_vulnerability(vulnerability);
                print_vulnerability(description, criticality);
            }
        }
    }

    if config.is_verbose() {
        println!();
        println!("{}", "The certificates were analyzed correctly!".green());
        println!();
    } else if !config.is_quiet() {
        println!("Certificates analyzed.");
    }
    Ok(())
}
