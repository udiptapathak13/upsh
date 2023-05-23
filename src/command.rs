use std::process::Command;

pub fn run(comm: String) -> Vec<u8>
{
	let args: Vec<&str> = comm.split(" ").collect();
	let mut process = Command::new(args[0]);
	for i in 1..args.len() {
		process.arg(args[i]);
	}
	return match process.output() {
	Ok(out) => if out.stdout.len() != 0 {out.stdout}
		else {out.stderr}
	Err(_) => {
		let mut invalid = String::from(args[0]);
		invalid.push_str(" is not a valid command\x0a");
		invalid.as_bytes().to_vec()
	}
	};
}
