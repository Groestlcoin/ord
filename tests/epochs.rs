use {super::*, ord::subcommand::epochs::Output, ord::Sat};

#[test]
fn empty() {
  assert_eq!(
    CommandBuilder::new("epochs").run_and_deserialize_output::<Output>(),
    Output {
      starting_sats: vec![
        Sat(0),
        Sat(1050000000000000 * 5),
        Sat(1575000000000000 * 5),
        Sat(1837500000000000 * 5),
        Sat(1968750000000000 * 5),
        Sat(2034375000000000 * 5),
        Sat(2067187500000000 * 5),
        Sat(2083593750000000 * 5),
        Sat(2091796875000000 * 5),
        Sat(2095898437500000 * 5),
        Sat(2097949218750000 * 5),
        Sat(2098974609270000 * 5),
        Sat(2099487304530000 * 5),
        Sat(2099743652160000 * 5),
        Sat(2099871825870000 * 5),
        Sat(2099935912620000 * 5),
        Sat(2099967955890000 * 5),
        Sat(2099983977420000 * 5),
        Sat(2099991988080000 * 5),
        Sat(2099995993410000 * 5),
        Sat(2099997995970000 * 5),
        Sat(2099998997250000 * 5),
        Sat(2099999497890000 * 5),
        Sat(2099999748210000 * 5),
        Sat(2099999873370000 * 5),
        Sat(2099999935950000 * 5),
        Sat(2099999967240000 * 5),
        Sat(2099999982780000 * 5),
        Sat(2099999990550000 * 5),
        Sat(2099999994330000 * 5),
        Sat(2099999996220000 * 5),
        Sat(2099999997060000 * 5),
        Sat(2099999997480000 * 5),
        Sat(2100000000000000 * 5)
      ]
    }
  );
}
