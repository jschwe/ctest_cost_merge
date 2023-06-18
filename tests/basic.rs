use std::process::Command;

#[test]
fn basic_cost_merge_123() {
    let args = [
        "-u",
        "update1.txt",
        "-u",
        "update2.txt",
        "-u",
        "update3.txt",
    ];
    basic_cost_merge_inner(&args, "123");
}

#[test]
fn basic_cost_merge_132() {
    let args = [
        "-u",
        "update1.txt",
        "-u",
        "update3.txt",
        "-u",
        "update2.txt",
    ];
    basic_cost_merge_inner(&args, "132");
}

#[test]
fn basic_cost_merge_312() {
    let args = [
        "-u",
        "update3.txt",
        "-u",
        "update1.txt",
        "-u",
        "update2.txt",
    ];
    basic_cost_merge_inner(&args, "312");
}

#[test]
fn basic_cost_merge_321() {
    let args = [
        "-u",
        "update3.txt",
        "-u",
        "update2.txt",
        "-u",
        "update1.txt",
    ];
    basic_cost_merge_inner(&args, "321");
}

#[test]
fn basic_cost_merge_213() {
    let args = [
        "-u",
        "update2.txt",
        "-u",
        "update1.txt",
        "-u",
        "update3.txt",
    ];
    basic_cost_merge_inner(&args, "213");
}

#[test]
fn basic_cost_merge_231() {
    let args = [
        "-u",
        "update2.txt",
        "-u",
        "update3.txt",
        "-u",
        "update1.txt",
    ];
    basic_cost_merge_inner(&args, "231");
}

fn basic_cost_merge_inner(update_args: &[&str], id: &str) {
    let bin = env!("CARGO_BIN_EXE_ctest_cost_merge");
    let tmp_dir = env!("CARGO_TARGET_TMPDIR");
    let mf_dir = env!("CARGO_MANIFEST_DIR");
    let output_file = format!("{tmp_dir}/new_cost_{id}.txt");
    let cost_file_dir = format!("{mf_dir}/tests/cost_files");
    Command::new(bin)
        .args(["base.txt", "-o", output_file.as_str()])
        .args(update_args)
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
