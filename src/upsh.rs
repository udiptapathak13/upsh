use std::io::Write;

fn main() {
	let mut comm = String::new();
	loop {
		std::io::stdout().write_all(b"pwd> ").unwrap();
		std::io::stdout().flush().unwrap();
		std::io::stdin().read_line(&mut comm).unwrap();
		comm.pop();
		if comm.eq("exit") {
			break;
		}
		print!("{}\x0a", comm);
		comm = String::new()
	}
}