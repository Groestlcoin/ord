use {super::*, ord::subcommand::list::Output};

#[test]
fn output_found() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  let output = CommandBuilder::new(
    "--index-sats list 3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:0",
  )
  .rpc_server(&rpc_server)
  .run_and_check_output::<Vec<Output>>();

  assert_eq!(
    output,
    vec![]
  );
}

#[test]
fn output_not_found() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  CommandBuilder::new(
    "--index-sats list 0000000000000000000000000000000000000000000000000000000000000000:0",
  )
  .rpc_server(&rpc_server)
  .expected_exit_code(1)
  .expected_stderr("error: output not found\n")
  .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  CommandBuilder::new("list 3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:0")
    .rpc_server(&rpc_server)
    .expected_stderr("error: list requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
