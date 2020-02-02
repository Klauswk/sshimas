use std::process::exit;
use sqlite::Connection;

extern crate clap;
use clap::{Arg, App};

fn main() {
		
    /*for argument in args().skip(1) {
		let arg = argument.as_ref();
        match arg {
            "l" => println!("0"),
            "a" => println!("1"),
            "c" => println!("2"),
            "r" => remove(),
            "h" => history(),
            _ => panic!("No args selected")
        }
    }*/
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
	
	if matches.is_present("list") {
		list_connections();
		exit(0);
	}
	
	if matches.is_present("history") {
		history();
		exit(0);
	}
	
	if matches.is_present("add") {
		add_connection(matches.value_of("add"));
		exit(0);
	}
	
	if matches.is_present("remove") {
		remove_connection(matches.value_of("remove"));
		exit(0);
	}
	
    let connection = create_connection(".db");

    create_table(connection);
}

fn add_connection(connection: Option<&str>) {
    println!("adding connection {}", connection.unwrap());
}

fn remove_connection(connection: Option<&str>) {
    println!("removing connection {}", connection.unwrap());
}

fn list_connections() {
    println!("listing connections");
}

fn history() {
    println!("showing history");
}

fn create_connection(db_name: &str) -> Connection {

    if db_name.is_empty()  {
        return sqlite::open(":memory:").unwrap();
    }
    sqlite::open(db_name).unwrap()
}

fn create_table(connection: Connection) {
    match connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS Users(Id INTEGER PRIMARY KEY, User TEXT NOT NULL, Ip TEXT NOT NULL, Password BLOB);
        "
    ) {
        Ok(_connection) => println!("Database created"),
        Err(e) => panic!("An error occour while creating the database: {}", e) 
    };
}
