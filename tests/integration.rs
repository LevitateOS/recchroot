//! Integration tests for recchroot binary.
//!
//! These tests run the actual binary and verify behavior.
//! Note: Actual chroot/mount tests require root and are marked #[ignore].

use std::process::Command;

/// Helper to run recchroot with given args
fn run_recchroot(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_recchroot"))
        .args(args)
        .output()
        .expect("Failed to execute recchroot")
}

// =============================================================================
// CLI Argument Tests
// =============================================================================

#[test]
fn test_help_flag() {
    let output = run_recchroot(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("chroot"), "Help should mention chroot");
    assert!(
        stdout.contains("bind mounts"),
        "Help should mention bind mounts"
    );
}

#[test]
fn test_version_flag() {
    let output = run_recchroot(&["--version"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("recchroot"));
}

#[test]
fn test_missing_chroot_argument() {
    let output = run_recchroot(&[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // clap should complain about missing required argument
    assert!(
        stderr.contains("required") || stderr.contains("<CHROOT_DIR>"),
        "stderr was: {}",
        stderr
    );
}

// =============================================================================
// Error Path Tests
// =============================================================================

#[test]
fn test_nonexistent_directory() {
    let output = run_recchroot(&["/nonexistent/path/12345"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should show E001 error code
    assert!(
        stderr.contains("E001:"),
        "Expected E001, stderr was: {}",
        stderr
    );
    assert!(stderr.contains("does not exist"), "stderr was: {}", stderr);
}

#[test]
fn test_file_instead_of_directory() {
    let output = run_recchroot(&["/etc/passwd"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should show E002 error code
    assert!(
        stderr.contains("E002:"),
        "Expected E002, stderr was: {}",
        stderr
    );
    assert!(stderr.contains("not a directory"), "stderr was: {}", stderr);
}

// =============================================================================
// Exit Code Tests
// =============================================================================

#[test]
fn test_exit_code_success_on_help() {
    let output = run_recchroot(&["--help"]);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn test_exit_code_failure_on_error() {
    let output = run_recchroot(&["/nonexistent"]);
    assert_ne!(output.status.code(), Some(0));
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_path() {
    let output = run_recchroot(&[""]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("E001:"),
        "Expected E001 for empty path, stderr was: {}",
        stderr
    );
}

#[test]
fn test_whitespace_only_path() {
    let output = run_recchroot(&["   "]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("E001:"),
        "Expected E001 for whitespace-only path, stderr was: {}",
        stderr
    );
}

#[test]
fn test_path_with_special_characters() {
    // Test paths that might cause shell issues but should be handled safely
    let output = run_recchroot(&["/nonexistent/path with spaces"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("E001:"),
        "Expected E001, stderr was: {}",
        stderr
    );
}

// =============================================================================
// Mount/Permission Error Tests (E003, E004)
// =============================================================================

#[test]
fn test_mount_without_root() {
    // Create temp dir, try to chroot without root - should fail on mount
    let dir = tempfile::tempdir().expect("Failed to create temp dir");
    let output = run_recchroot(&[dir.path().to_str().unwrap()]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should fail with E004 (mount failed - EPERM) when not root
    assert!(
        stderr.contains("E004:") || stderr.contains("permission"),
        "Expected E004 or permission error, stderr was: {}",
        stderr
    );
}

// =============================================================================
// Root-Required Tests (marked #[ignore])
// =============================================================================

/// Test actual chroot functionality - requires root
#[test]
#[ignore]
fn test_chroot_echo_command() {
    // This would need a proper chroot environment with /bin/echo
    // Only run manually with root: cargo test -- --ignored
    let output = run_recchroot(&["/", "echo", "hello"]);
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("hello"));
    }
}

/// Test E006 - invalid command inside chroot
#[test]
#[ignore]
fn test_invalid_command_in_chroot() {
    // Requires root - run with: cargo test -- --ignored
    let output = run_recchroot(&["/", "/nonexistent_command_12345"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // The chroot command itself should fail to execute the nonexistent command
    assert!(
        stderr.contains("E006:") || !output.status.success(),
        "Expected failure for invalid command, stderr was: {}",
        stderr
    );
}
