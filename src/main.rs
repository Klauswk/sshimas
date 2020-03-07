extern crate clap;
extern crate rpassword;

use common::ConnectionData;

#[cfg(feature = "sqlite_connection")]
use sqlite_connection::SqliteConnection;

use action::*;
use clap::{App, Arg, SubCommand};
use rpassword::read_password;
use std::env;
use std::process::exit;

fn main() {
	let version = format!(
		"{}.{}.{}{}",
		env!("CARGO_PKG_VERSION_MAJOR"),
		env!("CARGO_PKG_VERSION_MINOR"),
		env!("CARGO_PKG_VERSION_PATCH"),
		option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
	);

	let matches = App::new("Shhimas")
		.version(&*version)
		.author("Klaus Klein")
		.about("A database for ssh connections")
		.arg(Arg::with_name("ID").index(1))
		.arg(
			Arg::with_name("add")
				.short("a")
				.long("add")
				.value_name("connection")
				.help("Add a new connection")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("connect")
				.short("c")
				.long("connect")
				.value_name("connection")
				.help("Connect to ssh")
				.takes_value(true),
		)
		.subcommand(
			SubCommand::with_name("edit")
				.arg(
					Arg::with_name("password")
						.short("p")
						.long("password")
						.value_name("password")
						.help("Change the password")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("user")
						.short("u")
						.long("user")
						.value_name("user")
						.help("Change the user")
						.takes_value(true),
				)
				.arg(
					Arg::with_name("ip")
						.short("i")
						.long("ip")
						.value_name("ip")
						.help("Change the ip")
						.takes_value(true),
				)
				.alias("-e"),
		)
		.arg(
			Arg::with_name("remove")
				.short("r")
				.long("remove")
				.value_name("connection")
				.help("Remove a new connection")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("list")
				.short("l")
				.long("list")
				.help("List the connections"),
		)
		.arg(
			Arg::with_name("history")
				.short("h")
				.long("history")
				.help("Show the history of connections"),
		)
		.get_matches();

	if matches.args.is_empty() {
		println!("{}", matches.usage());
		exit(0);
	}

	let sqlite_connection = SqliteConnection::new(".db");

	if matches.is_present("edit") {
		let subcommand_match = matches.subcommand_matches("edit").unwrap();

		let id = matches.value_of("ID").unwrap();

		let con = sqlite_connection.get(id).unwrap();
		if subcommand_match.is_present("password") {
			println!("Editando senha");
			let password = String::from(subcommand_match.value_of("password").unwrap());

			let new_connection = ConnectionData { password, ..con };

			sqlite_connection.edit(&new_connection).expect("A error occour while updating password");
		} else if subcommand_match.is_present("user") {
			println!("Editando user");
			let user = String::from(subcommand_match.value_of("user").unwrap());

			let new_connection = ConnectionData { user, ..con };

			sqlite_connection.edit(&new_connection).expect("A error occour while updating user");
		} else if subcommand_match.is_present("ip") {
			println!("Editando ip");
			let ip = String::from(subcommand_match.value_of("ip").unwrap());

			let new_connection = ConnectionData { ip, ..con };

			sqlite_connection.edit(&new_connection).expect("A error occour while updating ip");
		}

		exit(0);
	} else if matches.is_present("ID") {
		let con = sqlite_connection
			.get(matches.value_of("ID").unwrap())
			.unwrap();
		sqlite_connection.append(&con);
		sqlite_connection.connect(&con);
		exit(0);
	}

	if matches.is_present("connect") {
		let con = sqlite_connection
			.get(matches.value_of("connect").unwrap())
			.unwrap();
		sqlite_connection.append(&con);
		sqlite_connection.connect(&con);
		exit(0);
	} else if matches.is_present("list") {
		let result = sqlite_connection.list();

		println!("{}\t{}\t{}","id", "user", "ip");
		
		for r in result.unwrap() {
			println!("{}\t{}\t{}", r.id, r.user, r.ip);
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
			id: String::new(),
		};

		println!("{}", sqlite_connection.add(&data).unwrap());
		exit(0);
	} else if matches.is_present("remove") {
		let id = matches.value_of("remove").unwrap();

		let result = sqlite_connection.remove(&ConnectionData {
			id: id.parse::<String>().unwrap(),
			password: String::new(),
			ip: String::new(),
			user: String::new(),
		});

		println!("{}", result.unwrap());
		exit(0);
	} else if matches.is_present("history") {
		sqlite_connection.history();
		exit(0);
	}
}
