use {super::*, ord::subcommand::find::Output};

#[test]
fn find_command_returns_satpoint_for_sat() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  assert_eq!(
    CommandBuilder::new("--index-sats find 0")
      .rpc_server(&rpc_server)
      .run_and_deserialize_output::<Output>(),
    Output {
      satpoint: "3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:0:0"
        .parse()
        .unwrap()
    }
  );
}

#[test]
fn unmined_sat() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  CommandBuilder::new("--index-sats find 5000000000")
    .rpc_server(&rpc_server)
    .expected_stderr("error: gro has not been mined as of index height\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  CommandBuilder::new("find 0")
    .rpc_server(&rpc_server)
    .expected_stderr("error: find requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
