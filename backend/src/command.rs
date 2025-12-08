use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "zero2prod")]
#[command(about = "A Rust web framework for production")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
	CreateAdmin {
		#[arg(short = 'u', long = "username", default_value = "admin")]
		username: String,
		#[arg(short = 'p', long = "password", default_value = "admin123")]
		password: String,
		#[arg(short = 'e', long = "email", default_value = "admin@admin.com")]
		email: String,
	},
	Server,
}
