use {super::*, ord::subcommand::subsidy::Output};

#[test]
fn genesis() {
  assert_eq!(
    CommandBuilder::new("subsidy 0").output::<Output>(),
    Output {
      first: 0,
      subsidy: 5000000000,
      name: "nvtdijuwxlp".into(),
    }
  );
}

#[test]
fn second_block() {
  assert_eq!(
    CommandBuilder::new("subsidy 1").output::<Output>(),
    Output {
      first: 5000000000,
      subsidy: 5000000000,
      name: "nvtcsezkbth".into(),
    }
  );
}

#[test]
fn second_to_last_block_with_subsidy() {
  assert_eq!(
    CommandBuilder::new("subsidy 46199998").output::<Output>(),
    Output {
      first: 10499999999999998,
      subsidy: 1,
      name: "b".into(),
    }
  );
}

#[test]
fn last_block_with_subsidy() {
  assert_eq!(
    CommandBuilder::new("subsidy 46199999").output::<Output>(),
    Output {
      first: 10499999999999999,
      subsidy: 1,
      name: "a".into(),
    }
  );
}

#[test]
fn first_block_without_subsidy() {
  CommandBuilder::new("subsidy 6930000")
    .expected_stderr("error: block 6930000 has no subsidy\n")
    .expected_exit_code(1)
    .run();
}
