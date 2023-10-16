pub fn create_template() -> String {
    format!(
        r#"# Bristol format for NAND gate
# Number of gates: 1
# Number of inputs: 2
# Number of outputs: 1

1 2 1 0 NAND"#
    )
}
