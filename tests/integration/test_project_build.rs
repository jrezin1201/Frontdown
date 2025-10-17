use ravensone::compile_component;

mod common;

#[test]
fn builds_all_fixture_components() {
    let fixtures = [
        ("basic/input.raven", "basic/expected.tsx"),
        ("components/button.raven", "components/button.expected.tsx"),
    ];

    for (input, expected) in fixtures {
        let source = common::read_fixture(input);
        let expected_tsx = common::read_fixture(expected);
        let output = compile_component(&source).expect("compilation succeeded");
        assert_eq!(output, expected_tsx, "fixture {} should compile", input);
    }
mod common;

#[test]
fn builds_example_project() {
    let fixture = common::fixture("components/button.raven");
    assert!(fixture.contains("button"));
}
