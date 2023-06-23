use {
  super::*,
  ord::subcommand::wallet::{cardinals::Cardinal, outputs::Output},
};

#[test]
fn cardinals() {
  let rpc_server = test_groestlcoincore_rpc::spawn();
  create_wallet(&rpc_server);

  inscribe(&rpc_server);

  let all_outputs = CommandBuilder::new("wallet outputs")
    .rpc_server(&rpc_server)
    .run_and_check_output::<Vec<Output>>();

  let cardinal_outputs = CommandBuilder::new("wallet cardinals")
    .rpc_server(&rpc_server)
    .run_and_check_output::<Vec<Cardinal>>();

  assert_eq!(all_outputs.len() - cardinal_outputs.len(), 1);
}
