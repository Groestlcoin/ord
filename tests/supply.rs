use {super::*, ord::subcommand::supply::Output};

#[test]
fn genesis() {
  assert_eq!(
    CommandBuilder::new("supply").run_and_check_output::<Output>(),
    Output {
      supply: 10500000000000000,
      first: 0,
      last: 10500000000000000,
      last_mined_in_block: 46199999
    }
  );
}
