extern crate chrono;
extern crate clap;
extern crate regex;
extern crate rpassword;
extern crate rusqlite;
extern crate uuid;

use rusqlite::{params, Connection, NO_PARAMS};
use std::io;
use std::result::Result;

use action::*;
use chrono::{DateTime, Utc};
use common::ConnectionData;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;
use uuid::Uuid;

#[derive(Debug)]
pub struct SqliteConnection {
	connection: rusqlite::Connection,
}

impl SqliteConnection {
	pub fn new(db_name: &str) -> Self {
		let conn = if db_name.is_empty() {
			return SqliteConnection {
				connection: Connection::open_in_memory().unwrap(),
			};
		} else {
			SqliteConnection {
				connection: Connection::open(db_name).unwrap(),
			}
		};
		match conn.connection
		    .execute(
		        "
		        CREATE TABLE IF NOT EXISTS Connection(Id CHAR(36) PRIMARY KEY, User TEXT NOT NULL, Ip TEXT NOT NULL, Password BLOB);
		        ",
		 		params![],
		    ) {
		        Ok(_connection) => println!("Database created"),
		        Err(e) => panic!("An error occour while creating the database: {}", e) 
		    };

		conn
	}
}

impl Add for SqliteConnection {
	fn add(&self, connection: &ConnectionData) -> Result<&str, String> {
		println!("adding connection {:?}", connection);

		let id = Uuid::new_v4();

		println!("UUID: {}",id);

		match self.connection.execute(
			"INSERT INTO Connection(Id, User,Ip,Password) VALUES(?1,?2,?3,?4)",
			&[&id.to_string(), &connection.user, &connection.ip, &connection.password],
		) {
			Ok(_ok) => Ok("Success"),
			Err(_e) => panic!(_e),
		}
	}
}

impl Connect for SqliteConnection {
	fn connect(&self, connection: &ConnectionData) {
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

		let mut password_array = ["", ""];

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
}

impl Remove for SqliteConnection {
	fn remove(&self, connection: &ConnectionData) -> Result<&str, String> {
		println!("removing connection {}", connection.id);

		match self
			.connection
			.execute("DELETE FROM Connection where Id = ?1", &[&connection.id])
		{
			Ok(_connection) => Ok("Success"),
			Err(_e) => Err(format!("An error occour while removing the connection with id: {}",connection.id)),
		}
	}
}

impl Get for SqliteConnection {
	fn get(&self, id: &str) -> Result<ConnectionData, String> {
		println!("fetching connections");

		let mut stmt = self
			.connection
			.prepare("SELECT Id, User, Ip, Password FROM Connection where Id like ? || '%'")
			.unwrap();

		let result = stmt.query_row(&[&id], |row| {
			Ok(ConnectionData {
				user: row.get(1)?,
				ip: row.get(2)?,
				password: row.get(3)?,
				id: row.get(0)?,
			})
		});

		match result {
			Ok(data) => Ok(data),
			Err(_err) => Err(format!("No data found for the id: {}", id)),
		}
	}
}

impl List for SqliteConnection {
	fn list(&self) -> Result<Vec<ConnectionData>, String> {
		println!("listing connections");

		let mut stmt = self
			.connection
			.prepare("SELECT Id, User, Ip FROM Connection")
			.unwrap();

		let connections = stmt.query_map(NO_PARAMS, |row| {
			Ok(ConnectionData {
				user: row.get(1)?,
				ip: row.get(2)?,
				password: String::new(),
				id: row.get(0)?,
			})
		});
		
		match connections {
			Ok(connections) => {
				let mut conns = Vec::<ConnectionData>::new();
			
				for con in connections {
					conns.push(con.unwrap());
				}
				
				Ok(conns)
			}
			Err(_err) => Err("An error occour while fetching data from the database".to_string()),
		}
	}
}

impl History for SqliteConnection {
	fn history(&self) {
		let mut f = File::open(".history").unwrap();
		let mut s = String::new();
		f.read_to_string(&mut s).unwrap();

		println!("{}", s);
	}

	fn append(&self, _connection: &ConnectionData) {
		let mut file = OpenOptions::new()
			.append(true)
			.create(true)
			.open(".history")
			.unwrap();
		let user_ip = [_connection.user.to_string(), _connection.ip.to_string()].join("@");

		let now: DateTime<Utc> = Utc::now();

		if let Err(e) = writeln!(
			file,
			"{}\t{}\t{}",
			_connection.id,
			now.format("%d-%m-%Y"),
			user_ip
		) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}
}
