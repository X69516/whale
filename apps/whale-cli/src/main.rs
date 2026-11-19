use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--version") | Some("-V") => {
            println!("whale {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }

        Some("--help") | Some("-h") | None => {
            println!(
                "Whale - Security-oriented HTTP server\n\
                 \n\
                 USAGE:\n\
                 \twhale-cli [OPTIONS]\n\
                 \n\
                 OPTIONS:\n\
                 \t-h, --help       Print help\n\
                 \t-V, --version    Print version"
            );

            ExitCode::SUCCESS
        }

        Some(command) => {
            eprintln!("Unknown argument: {command}");
            ExitCode::FAILURE
        }
    }
}
