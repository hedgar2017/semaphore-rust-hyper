//!
//! The CI pipeline demo binary.
//!

///
/// The main function gets the HTTP port from the command line arguments
/// and starts the Hyper HTTP server using the library method.
///
fn main() -> Result<(), ()> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("port")
                .help("The HTTP server port")
                .short("p")
                .long("port")
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("8080"),
        )
        .get_matches();

    let port = args.value_of("port").expect("Unreachable");
    let _port: u16 = port.parse().expect("Unreachable");

    Ok(())
}
