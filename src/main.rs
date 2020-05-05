use clap::{App, AppSettings};

fn main() {
    let matches = App::new("time-splitter")
        .bin_name("tsplit")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1")
        .about("Manage time, maintain motivation!")
        .author("Andrew Halle <ahalle@berkeley.edu>")
        .subcommand(App::new("practice").about("Quick practice"))
        .subcommand(App::new("project").about("A longer project"))
        .subcommand(App::new("dcp").about("Pick a language for Daily Coding Problem"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("practice") {
        println!("prctice");
    } else if let Some(ref matches) = matches.subcommand_matches("project") {
        println!("prject");
    } else if let Some(ref matches) = matches.subcommand_matches("dcp") {
        println!("cp");
    } else {
    }
}
