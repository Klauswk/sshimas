extern crate regex;
extern crate clap;
extern crate rpassword;
extern crate chrono;

use action::*;
use rpassword::read_password;
use std::process::{exit};
use clap::{Arg, App};
use std::env;

use common::ConnectionData;
use sqlite_connection::SqliteConnection;

fn main() {
	let version = format!("{}.{}.{}{}",
                      env!("CARGO_PKG_VERSION_MAJOR"),
                      env!("CARGO_PKG_VERSION_MINOR"),
                      env!("CARGO_PKG_VERSION_PATCH"),
                      option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

	let matches = App::new("Shhimas")
                          .version(&*version)
                          .author("Klaus Klein")
                          .about("A database for ssh connections")
                          .arg(Arg::with_name("add")
                               .short("a")
                               .long("add")
                               .value_name("connection")
                               .help("Add a new connection")
                               .takes_value(true))
						  .arg(Arg::with_name("connect")
                               .short("c")
                               .long("connect")
                               .value_name("connection")
                               .help("Connect to ssh")
                               .takes_value(true))
                          .arg(Arg::with_name("remove")
                               .short("r")
                               .long("remove")
                               .value_name("connection")
                               .help("Remove a new connection")
                               .takes_value(true))
                          .arg(Arg::with_name("list")
                               .short("l")
							   .long("list")
                               .help("List the connections"))
                          .arg(Arg::with_name("history")
                               .short("h")
							   .long("history")
                               .help("Show the history of connections"))
                          .get_matches();
	
	let sqlite_connection = SqliteConnection::new(".db");
		
	if matches.is_present("connect") {
		let con = sqlite_connection.get(matches.value_of("connect").unwrap()).unwrap();
		
		sqlite_connection.append(&con);
		
		sqlite_connection.connect(&con);
		
		exit(0);
	} else if matches.is_present("list") {
		let result = sqlite_connection.list();
		
		for r in result.unwrap() {
			println!("Connections: {:?}", r);
		}
		exit(0);
	} else if matches.is_present("add") {
		//let re = Regex::new(r"^[A-Za-z][A-Za-z0-9_]*\\@[A-Za-z][A-Za-z0-9_\.]*\\:(\\/[A-Za-z][A-Za-z0-9_]*)*$").unwrap();
		
		let new_connection = matches.value_of("add").unwrap();
				
		println!("\nType your password or ENTER to leave blank\n");
		let password = read_password().unwrap();
		
		let user_and_ip: Vec<&str> = new_connection.split('@').collect();
		
		let data = ConnectionData {
			user: user_and_ip[0].to_string(),
    		ip: user_and_ip[1].to_string(),
    		password,
			id: 0,
		};

		println!("{}",sqlite_connection.add(&data).unwrap());
		exit(0);
	} else if matches.is_present("remove") {
		let id = matches.value_of("remove").unwrap();

		let result = sqlite_connection.remove(&ConnectionData{
			id: id.parse::<u32>().unwrap(),
			password: String::new(),
			ip: String::new(),
			user: String::new()
		});
		
		println!("{}",result.unwrap());
		
		exit(0);
	} else if matches.is_present("history") {
		sqlite_connection.history();
		exit(0);
	}
}
