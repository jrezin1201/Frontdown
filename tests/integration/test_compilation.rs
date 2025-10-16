mod common;

#[test]
fn compiles_basic_program() {
    let fixture = common::fixture("basic/input.raven");
    assert!(fixture.contains("basic"));
}
