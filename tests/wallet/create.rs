use {super::*, ord::subcommand::wallet::create::Output};

#[test]
fn create() {
  let rpc_server = test_groestlcoincore_rpc::spawn();

  assert!(!rpc_server.wallets().contains("ord"));

  CommandBuilder::new("wallet create")
    .rpc_server(&rpc_server)
    .run_and_deserialize_output::<Output>();

  assert!(rpc_server.wallets().contains("ord"));
}

#[test]
fn seed_phrases_are_twelve_words_long() {
  let Output { mnemonic, .. } = CommandBuilder::new("wallet create")
    .rpc_server(&test_groestlcoincore_rpc::spawn())
    .run_and_deserialize_output();

  assert_eq!(mnemonic.word_count(), 12);
}

#[test]
fn wallet_creates_correct_mainnet_taproot_descriptor() {
  let rpc_server = test_groestlcoincore_rpc::spawn();

  CommandBuilder::new("wallet create")
    .rpc_server(&rpc_server)
    .run_and_deserialize_output::<Output>();

  assert_eq!(rpc_server.descriptors().len(), 2);
  assert_regex_match!(
    &rpc_server.descriptors()[0],
    r"tr\(\[[[:xdigit:]]{8}/86'/0'/0'\]xprv[[:alnum:]]*/0/\*\)#[[:alnum:]]{8}"
  );
  assert_regex_match!(
    &rpc_server.descriptors()[1],
    r"tr\(\[[[:xdigit:]]{8}/86'/0'/0'\]xprv[[:alnum:]]*/1/\*\)#[[:alnum:]]{8}"
  );
}

#[test]
fn wallet_creates_correct_test_network_taproot_descriptor() {
  let rpc_server = test_groestlcoincore_rpc::builder()
    .network(Network::Signet)
    .build();

  CommandBuilder::new("--chain signet wallet create")
    .rpc_server(&rpc_server)
    .run_and_deserialize_output::<Output>();

  assert_eq!(rpc_server.descriptors().len(), 2);
  assert_regex_match!(
    &rpc_server.descriptors()[0],
    r"tr\(\[[[:xdigit:]]{8}/86'/1'/0'\]tprv[[:alnum:]]*/0/\*\)#[[:alnum:]]{8}"
  );
  assert_regex_match!(
    &rpc_server.descriptors()[1],
    r"tr\(\[[[:xdigit:]]{8}/86'/1'/0'\]tprv[[:alnum:]]*/1/\*\)#[[:alnum:]]{8}"
  );
}

#[test]
fn detect_wrong_descriptors() {
  let rpc_server = test_groestlcoincore_rpc::spawn();

  CommandBuilder::new("wallet create")
    .rpc_server(&rpc_server)
    .run_and_deserialize_output::<Output>();

  rpc_server.import_descriptor("wpkh([aslfjk])#a23ad2l".to_string());

  CommandBuilder::new("wallet transactions")
    .rpc_server(&rpc_server)
    .stderr_regex(
      r#"error: wallet "ord" contains unexpected output descriptors, and does not appear to be an `ord` wallet, create a new wallet with `ord wallet create`\n"#,
    )
    .expected_exit_code(1)
    .run_and_extract_stdout();
}

#[test]
fn create_with_different_name() {
  let rpc_server = test_groestlcoincore_rpc::spawn();

  assert!(!rpc_server.wallets().contains("inscription-wallet"));

  CommandBuilder::new("wallet --name inscription-wallet create")
    .rpc_server(&rpc_server)
    .run_and_deserialize_output::<Output>();

  assert!(rpc_server.wallets().contains("inscription-wallet"));
}
