use {super::*, ord::subcommand::subsidy::Output};

#[test]
fn genesis() {
  assert_eq!(
    CommandBuilder::new("subsidy 0").run_and_deserialize_output::<Output>(),
    Output {
      first: 0,
      subsidy: 0,
      name: "bvivuucdvvev".into(),
    }
  );
}

#[test]
fn second_block() {
  assert_eq!(
    CommandBuilder::new("subsidy 1").run_and_deserialize_output::<Output>(),
    Output {
      first: 0,
      subsidy: 24064000000000,
      name: "bvivudxiizmn".into(),
    }
  );
}

#[test]
fn second_to_last_block_with_subsidy() {
  assert_eq!(
    CommandBuilder::new("subsidy 9078422").run_and_deserialize_output::<Output>(),
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
    CommandBuilder::new("subsidy 9078423").run_and_deserialize_output::<Output>(),
    Output {
      first: 10499999999999999,
      subsidy: 1,
      name: "a".into(),
    }
  );
}

#[test]
fn first_block_without_subsidy() {
  CommandBuilder::new("subsidy 9078424")
    .expected_stderr("error: block 9078424 has no subsidy\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
