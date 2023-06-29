use {super::*, ord::subcommand::traits::Output, ord::Rarity};

#[test]
fn traits_command_prints_sat_traits() {
  assert_eq!(
    CommandBuilder::new("traits 0").run_and_check_output::<Output>(),
    Output {
      number: 0,
      decimal: "0.0".into(),
      degree: "0°0′0″0‴".into(),
      name: "bvivuucdvvev".into(),
      height: 0,
      cycle: 0,
      epoch: 0,
      period: 0,
      offset: 0,
      rarity: Rarity::Mythic,
    }
  );
}
#[test]
fn traits_command_for_last_sat() {
  assert_eq!(
    CommandBuilder::new("traits 10499999999999999").run_and_check_output::<Output>(),
    Output {
      number: 10499999999999999,
      decimal: "46199999.0".into(),
      degree: "7°1049999′1343″0‴".into(),
      name: "a".into(),
      height: 46199999,
      cycle: 5,
      epoch: 32,
      period: 22916,
      offset: 0,
      rarity: Rarity::Uncommon,
    }
  );
}
