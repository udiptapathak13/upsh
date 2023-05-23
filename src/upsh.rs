use std::io;
use std::io::Write;
mod session;

fn main()
{
	let mut comm: String;
	let mut session: Vec<u8>;
	loop {
		session = session::get();
		io::stdout().write_all(&session).unwrap();
		io::stdout().flush().unwrap();
		comm = String::new();
		io::stdin().read_line(&mut comm).unwrap();
		comm.pop();
		if comm.eq("exit") {
			break;
		}
		print!("{}\x0a", comm);
	}
}