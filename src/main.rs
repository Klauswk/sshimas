extern crate regex;
extern crate clap;
extern crate rpassword;
extern crate chrono;

use std::io::Read;
use rpassword::read_password;
use std::process::exit;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, params};
use clap::{Arg, App};
use std::io::{self, Write};
use std::process::Command;
use std::env;
use std::fs::{File, OpenOptions};
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct ConnectionData {
    user: String,
    ip: String,
    password: String,
	id: u32,
}

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
	
    let connection = create_connection(".db");

    create_table(&connection);
	
	if matches.is_present("connect") {
		let con = get_connection(&connection, matches.value_of("connect").unwrap());
		
		append_history(&con);
		
		start_connection(&con);
		exit(0);
	} else if matches.is_present("list") {
		list_connections(&connection);
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

		add_connection(&connection, data);
		exit(0);
	} else if matches.is_present("remove") {
		let id = matches.value_of("remove");

		remove_connection(&connection, id.unwrap());
		exit(0);
	} else if matches.is_present("history") {
		history();
		exit(0);
	}
}

fn start_connection(connection: &ConnectionData) {

let path = env::current_dir();

if path.is_err() {
	panic!("Couldn`t find the correct path of the application");
}

let path_buff = if cfg!(target_os = "windows") {
	let mut windows_path = path.ok().unwrap();
	windows_path.push("bin\\putty.exe");
	windows_path
} else {
	let mut unix_path = path.ok().unwrap();
	unix_path.push("bin/plink");
	unix_path
};

println!("The current directory is {}", path_buff.display());

println!("{:?}", connection);

let user_ip = [connection.user.to_string(), connection.ip.to_string()].join("@");

let mut password_array = ["",""];

let output = if !connection.password.is_empty() {
	password_array[0] = "-pw";
	password_array[1] = &*connection.password;
	
	Command::new(path_buff)
            .args(&["-ssh", &*user_ip])
			.args(&password_array)
            .output()
            .expect("failed to execute process")
} else {
	Command::new(path_buff)
            .args(&["-ssh", &*user_ip])
            .output()
            .expect("failed to execute process")
};

println!("status: {}", output.status);
io::stdout().write_all(&output.stdout).unwrap();
io::stderr().write_all(&output.stderr).unwrap();
}

fn add_connection(_connection: &Connection, con: ConnectionData) {
    println!("adding connection {:?}", con);

	match _connection.execute(
		"INSERT INTO Connection(User,Ip,Password) VALUES(?1,?2,?3)",
            &[&con.user, &con.ip, &con.password],
        ) {
        Ok(_connection) => println!("Connection added"),
        Err(e) => panic!("An error occour while adding the connection: {}", e) 
    };
	
}

fn remove_connection(_connection: &Connection, id: &str) {
    println!("removing connection {}", id);

	match _connection.execute(
		"DELETE FROM Connection where Id = ?1",
            &[&id],
        ) {
        Ok(_connection) => println!("Connection removed"),
        Err(e) => panic!("An error occour while removing the connection: {}", e) 
    };
}

fn get_connection(_connection: &Connection, id: &str) -> ConnectionData {
    println!("fetching connections");

	let mut stmt = _connection
    .prepare("SELECT Id, User, Ip, Password FROM Connection where Id = ?").unwrap();

    let result = stmt.query_row(&[&id], |row| {
        Ok(ConnectionData{
			user: row.get(1)?,
    		ip: row.get(2)?,
    		password: row.get(3)?,
			id: row.get(0)?,
		})
	});
	
	match result {
		Ok(data) => data,
		Err(_err) => panic!("Not data found for the id: {} ", id)
	}
}

fn list_connections(connection: &Connection) {
    println!("listing connections");

	let mut stmt = connection
    .prepare("SELECT Id, User, Ip FROM Connection").unwrap();

    let connections = match stmt.query_map(NO_PARAMS, |row| {
        Ok(ConnectionData{
			user: row.get(1)?,
    		ip: row.get(2)?,
    		password: String::new(),
			id: row.get(0)?,
		})
	}) {
		Ok(connections) => connections,
		Err(err) => panic!("An error occour while fetching data from the database: {}", err)
	};
	
	for con in connections {		
		println!("Connections: {:?}", con);
	}
}

fn append_history(_connection: &ConnectionData) {
    let mut file = OpenOptions::new()
        .append(true)
		.create(true)
        .open(".history")
        .unwrap();

	let user_ip = [_connection.user.to_string(), _connection.ip.to_string()].join("@");

    let now: DateTime<Utc> = Utc::now();

    if let Err(e) = writeln!(file, "{}\t{}\t{}", _connection.id, now.format("%d-%m-%Y"), user_ip) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn history() {
	let mut f = File::open(".history").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

	println!("{}", s);
}

fn create_connection(db_name: &str) -> Connection {

    if db_name.is_empty()  {
        return Connection::open_in_memory().unwrap();
    }
    Connection::open(db_name).unwrap()
}

fn create_table(connection: &Connection) {
    match connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS Connection(Id INTEGER PRIMARY KEY, User TEXT NOT NULL, Ip TEXT NOT NULL, Password BLOB);
        ",
 		params![],
    ) {
        Ok(_connection) => println!("Database created"),
        Err(e) => panic!("An error occour while creating the database: {}", e) 
    };
}
