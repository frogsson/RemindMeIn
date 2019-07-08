extern crate remindmein;

fn main() {
    let args = remindmein::Args::new().unwrap_or_else(|err|{
        eprintln!("{}", err);

        std::process::exit(1)
    });

    if let Err(e) = remindmein::run(args) {
        eprintln!("{}", e);

        std::process::exit(1)
    }
}
