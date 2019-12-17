extern crate clap;
use std::process::{Command, Stdio};
use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = arguments();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let mut in_args: Vec<&str> = input.split(" ").collect();
    let mut out_args: Vec<&str> = output.split(" ").collect();

    let in_command = in_args.remove(0);
    let out_command = out_args.remove(0);

    let in_command = Command::new(&in_command[..])
        .args(&in_args)
        .output()
        .expect("Error in the input command");

    let input = String::from_utf8_lossy(&in_command.stdout);

    let out_command = Command::new( out_command )
        .arg(&input[..])
        .args(out_args)
        .stdin(Stdio::piped())
        .output()
        .expect("Error in the output command");

    println!("{}", String::from_utf8_lossy(&out_command.stdout));
}

fn arguments() -> ArgMatches<'static> {

    let arguments = App::new("mypipe")
        .version("1.0.0")
        .about("A Unix like command pipe in RUST")
        .author("Thomas BRAZE <thomas.braze@myges.fr>")
        .arg( Arg::with_name("input")
                  .short("in")
                  .long("input")
                  .value_name("input")
                  .help("input command")
                  .takes_value(true)
                  .required(true)
        ).arg( Arg::with_name("output")
        .short("out")
        .long("output")
        .value_name("output")
        .help("output command")
        .takes_value(true)
        .required(true)
    )
        .get_matches();

    return  arguments;
}
