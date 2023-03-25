fn main() {
    #[cfg(windows)]
    windres::Build::new()
        .compile("resources/resources.rc")
        .unwrap();
}
