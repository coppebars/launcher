use {
	launchers::distros::mojang::Mojang,
	std::path::Path,
};

fn main() {
	let distro = Mojang::try_from_file(Path::new("./minecraft/versions/1.20.1/1.20.1.json")).unwrap();

	let mut process = distro.try_into_process().unwrap();

	process.cwd = "./minecraft".into();

	process.launch_blocking();
}
