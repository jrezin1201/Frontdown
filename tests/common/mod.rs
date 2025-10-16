use std::fs;
use std::path::{Path, PathBuf};

pub fn fixture_path(name: &str) -> PathBuf {
    Path::new("tests/fixtures").join(name)
}

pub fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture_path(name)).expect("fixture should exist")
}
