//!
//! The CI pipeline demo binary.
//!

#[derive(Debug)]
pub enum Error {}

///
/// The main function gets the HTTP port from the command line arguments
/// and starts the Hyper HTTP server using the library method.
///
fn main() -> Result<(), Error> {
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
                .required(true),
        )
        .get_matches();

    let port = args.value_of("port").expect("Unreachable");
    let port: u16 = port.parse().expect("Unreachable");

    semaphore_rust_hyper::run(port);

    Ok(())
}
