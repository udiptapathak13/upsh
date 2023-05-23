use std::process::Command;
use std::ffi::OsStr;
use std::str;

pub fn simplified_pwd(uname: &Vec<u8>) -> Vec<u8> {
	let pwd = Command::new("pwd")
		.output()
		.unwrap()
		.stdout;
	let home = Command::new("grep")
		.arg(OsStr::new(str::from_utf8(&uname)
			.unwrap()))
		.arg("/etc/passwd")
		.output()
		.unwrap()
		.stdout;
	let mut iter = 0;
	let mut iter2 = 0;
	while home[iter2] != b'/' {
		iter2 += 1;
	}
	while pwd[iter] == home[iter2] {
		iter += 1;
		iter2 += 1;
	}
	let pwd_len = pwd.len();
	let mut res = Vec::<u8>::new();
	if uname.len() != 4
	|| uname[0] != b'r'
	|| uname[1] != b'o'
	|| uname[2] != b'o'
	|| uname[3] != b't' {
		res.push(b'~');
	} else {
		res.push(b'/');
	}
	while iter < pwd_len {
		res.push(pwd[iter]);
		iter += 1;
	}
	return res;
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
	let mut pwd = simplified_pwd(&uname);
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
