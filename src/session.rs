use std::process::Command;
use std::ffi::OsStr;
use std::str;

pub fn pwd(uname: &Vec<u8>) -> Vec<u8> {
	let pwd = Command::new("pwd")
		.output()
		.unwrap()
		.stdout;
	return pwd;
}

pub fn get() -> Vec<u8>
{
	let mut res = Vec::<u8>::new();
	let mut uname = Command::new("whoami")
		.output()
		.unwrap()
		.stdout;
	uname.pop();
	let mut hname = Command::new("cat")
		.arg("/proc/sys/kernel/hostname")
		.output()
		.unwrap()
		.stdout;
	let mut pwd = pwd(&uname);
	res.append(&mut uname);
	res.push(b'@');
	res.append(&mut hname);
	res.pop();
	res.push(b' ');
	res.append(&mut pwd);
	res.pop();
	res.push(b' ');
	return res;
}
