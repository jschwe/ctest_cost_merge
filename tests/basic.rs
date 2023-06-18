use std::process::Command;

#[test]
fn basic_cost_merge() {
    let bin = env!("CARGO_BIN_EXE_ctest_cost_merge");
    let tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let mf_dir = env!("CARGO_MANIFEST_DIR");
    let output_file = format!("{tmp_dir}/new_cost.txt");
    let cost_file_dir = format!("{mf_dir}/tests/cost_files");
    // CARGO_MANIFEST_DIR
    Command::new(bin)
        .args([
            "base.txt",
            "-u",
            "update1.txt",
            "-u",
            "update2.txt",
            "-u",
            "update3.txt",
            "-o",
            output_file.as_str(),
        ])
        .current_dir(&cost_file_dir)
        .output()
        .expect("Failed");

    let actual = std::fs::read_to_string(output_file).unwrap();
    let expected = std::fs::read_to_string(format!("{cost_file_dir}/expected_new.txt")).unwrap();
    assert_eq!(
        expected.trim(),
        actual.trim(),
        "Expected and actual merged content diverged"
    );
}
