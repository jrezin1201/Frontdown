mod common;

#[test]
fn builds_example_project() {
    let fixture = common::fixture("components/button.raven");
    assert!(fixture.contains("button"));
}
