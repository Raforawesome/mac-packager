mod plist;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn main() {
	let exe_name: String = get_input("Enter your executable name: ").trim().to_owned();
	let display_name: String = get_input("Enter the application's display name: ").trim().to_owned();
	let bundle_name: String = get_input("Enter your bundle name (no spaces): ").trim().to_owned();
	let bundle_id: String = get_input("Enter your bundle ID (reverse DNS notation): ").trim().to_owned();
	let sig: String = get_input("Enter your application's signature (4 characters): ").trim().to_owned();

	let pl = plist::create(bundle_name, display_name, bundle_id, sig, exe_name.clone());


	let _ = fs::create_dir_all(PathBuf::from("Generated.app/Contents"));
	let _ = fs::create_dir(PathBuf::from("Generated.app/Contents/MacOS"));
	let _ = fs::create_dir(PathBuf::from("Generated.app/Contents/Resources"));

	let _ = fs::copy(
		PathBuf::from(format!("./{}", exe_name)),
		PathBuf::from(format!("Generated.app/Contents/MacOS/{}", exe_name))
	);

	let _ = fs::write(
		PathBuf::from("Generated.app/Contents/Info.plist"),
		pl.as_bytes()
	);
}

fn get_input(m: &str) -> String {
	print!("{}", m);
	let _ = std::io::stdout().flush();
	let mut buffer: String = String::new();
	let _ = std::io::stdin().read_line(&mut buffer);
	buffer
}