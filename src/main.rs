use std::{env, process::exit};

use rarreg::rarreg;

macro_rules! get_help {
    () => {
        eprint!(
            r#"Usage:
    rarreg "Name" "Licence Type"

e.g.
    rarreg "GitHub, Inc." "5000 PC usage licence"

Then save the output in a file named 'rarreg.key'.
"#
        );
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        get_help!();
        exit(-1);
    }

    println!(
        "{}",
        rarreg::RegInfo::new(args[1].to_owned(), args[2].to_owned())
    );
}
