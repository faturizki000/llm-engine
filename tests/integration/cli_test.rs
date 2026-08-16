use assert_cmd::Command;

#[test]
fn single_binary_supports_generate_command() {
    let mut cmd = Command::cargo_bin("llm-engine").unwrap();
    cmd.args(["generate", "--prompt", "hello", "--max-tokens", "4"]);
    let assert = cmd.assert();
    assert.success();
}
