#![crate_id = "genllvmdeps"]
#![comment = "LLVM Dependencies generator"]
#![crate_type = "bin"]

#![deny(non_uppercase_statics, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, unused_must_use)]

use std::os::args;
use std::io::fs::File;
use std::io::{IoError, Command, LineBufferedWriter};
use std::iter::FromIterator;
use std::path::Path;
use std::fmt::Show;
use std::from_str::from_str;
static LLCONFIG:&'static str = "llvm-config";
static FILE_ERR:&'static str = "Could not write to file";
fn attempt<T, E:Show>(res:Result<T, E>, err:&'static str) -> T {
	match res {
		Ok(v) => v,
		Err(e) => fail!("{} ({})", err, e)
	}
}
fn get_output(command:&'static str, args: &[&'static str]) -> String {
	let mut command = Command::new(command);
	for arg in args.iter() {
		command.arg(*arg);
	}
	let process = attempt(command.spawn(), "Could not create process");
	match process.stdout.clone() {
		Some(mut v) => attempt(v.read_to_str(), "Could not read from process"),
		None => fail!("")
	}
}
pub fn main() {
	let os_args = args();
	let (file, enable_static) = match os_args.as_slice().slice_from(1) {
		[ref file] => (file, false),
		[ref file, ref flag] if flag.as_slice() == "--static" => (file, true),
		args => fail!("Expected single argument to genllvmdeps, but got {}", args.concat())
	};
	let path = Path::new(file.as_slice());
	let mut file = LineBufferedWriter::new(File::create(&path));
	attempt(file.write_line("// This is a generated file. Please see the generator at src/genllvmdeps/gen.rs if you want to change this"), "Could not write to file");
	let target = get_output(LLCONFIG, ["--host-target"]);
	let (arch, os) = {
		let (mut arch, mut os) = (None, None);
		let mut splits = target.as_slice().splitn('-', 1);
		for split in splits {
			if arch.is_none() {
				arch = Some(split)
			} else {
				os = Some(split)
			}
		}
		(arch.unwrap(), os.unwrap())
	};
	let arch = match arch {
		"i686" | "i386" => "x86",
		_ => arch
	};
	let os = match os {
		_ if os.contains("darwin") =>
			"macos",
		_ if os.contains("linux") =>
			"linux",
		_ if os.contains("freebsd") =>
			"freebsd",
		_ if os.contains("android") =>
			"android",
		_ if os.contains("mingw") || os.contains("win") =>
			"win32",
		_ => os
	};
	attempt(file.write_line(format!("\\#[cfg( target_arch = \"{}\", target_os = \"{}\")]", arch, os).as_slice()), "Could not write to file");
	let mut version_string = get_output(LLCONFIG, ["--version"]);
	version_string = version_string.as_slice().trim().into_string();
	let version : Vec<int> = FromIterator::from_iter(version_string.as_slice().splitn('.', 2).map(|vt|from_str::<int>(vt).unwrap()));
	let (major_version, minor_version) = (*version.get(0), *version.get(1));
	let args: Vec<&'static str> = if major_version > 3 || (major_version == 3 && minor_version >= 5) {
		vec!("--libs", "engine")
	} else {
		vec!("--libs", "--system-libs")
	};
	let out = get_output(LLCONFIG, args.as_slice());
	println!("Output from {} = {}", args, out);
	for lib in out.as_slice().trim().replace("\n", " ").as_slice().split(' ').filter(|l| l.len() > 2 && l.starts_with("-l")) {
		let lib = lib.trim().slice_from(2);
		attempt::<(), IoError>((|| {
			try!(file.write_str(format!("\\#[link(name = \"{}\"", lib).as_slice()));
			if lib.contains("LLVM") {
				try!(file.write_str(", kind = \"static\""));
			}
			file.write_line(")]")
		})(), FILE_ERR);
	}
	if major_version <= 3 && minor_version <= 5 && os == "win32" {
		attempt(file.write_line("#[link(name = \"imagehlp\")]"), FILE_ERR);
	}
	if os == "linux" {
		attempt(file.write_line("#[link(name = \"ncurses\")]"), FILE_ERR);
	}
	let out = get_output(LLCONFIG, ["--ldflags"]);
	for lib in out.as_slice().trim().split(' ') {
		if lib.as_slice().slice_to(2) == "-l" {
			attempt(file.write_line(format!("\\#[link(name = \"{}\")]", lib.slice_from(2)).as_slice()), FILE_ERR);
		}
	}
	let out = get_output(LLCONFIG, ["--cxxflags"]);
	match (enable_static, out.as_slice().contains("stdlib=libc++")) {
		(true, true) =>
			fail!("'stdlib=libc++' should not be in output for static"),
		(true, false) =>
			attempt(file.write_line("#[link(name = \"stdc++\", kind = \"static\")]"), FILE_ERR),
		(false, true) =>
			attempt(file.write_line("#[link(name = \"c++\")]"), FILE_ERR),
		(false, false) =>
			attempt(file.write_line("#[link(name = \"stdc++\")]"), FILE_ERR)
	}
	attempt(file.write_line("extern {}"), FILE_ERR);
}