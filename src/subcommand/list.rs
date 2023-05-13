use super::*;

#[derive(Debug, Parser)]
pub(crate) struct List {
  #[clap(help = "List gros in <OUTPOINT>.")]
  outpoint: OutPoint,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub output: OutPoint,
  pub start: u64,
  pub size: u64,
  pub rarity: Rarity,
  pub name: String,
}

impl List {
  pub(crate) fn run(self, options: Options) -> Result {
    let index = Index::open(&options)?;

    index.update()?;

    match index.list(self.outpoint)? {
      Some(crate::index::List::Unspent(ranges)) => {
        let mut outputs = Vec::new();
        for (output, start, size, rarity, name) in list(self.outpoint, ranges) {
          outputs.push(Output {
            output,
            start,
            size,
            rarity,
            name,
          });
        }

        print_json(outputs)?;

        Ok(())
      }
      Some(crate::index::List::Spent) => Err(anyhow!("output spent.")),
      None => Err(anyhow!("output not found")),
    }
  }
}

fn list(outpoint: OutPoint, ranges: Vec<(u64, u64)>) -> Vec<(OutPoint, u64, u64, Rarity, String)> {
  ranges
    .into_iter()
    .map(|(start, end)| {
      let size = end - start;
      let rarity = Sat(start).rarity();
      let name = Sat(start).name();

      (outpoint, start, size, rarity, name)
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn list_ranges() {
    let outpoint =
      OutPoint::from_str("3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:5")
        .unwrap();
    let ranges = vec![
      (50 * COIN_VALUE, 55 * COIN_VALUE),
      (10, 100),
      (1050000000000000, 1150000000000000),
    ];
    assert_eq!(
      list(outpoint, ranges),
      vec![
        (
          OutPoint::from_str("3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:5")
            .unwrap(),
          50 * COIN_VALUE,
          5 * COIN_VALUE,
          Rarity::Uncommon,
          "bvivudxiizmn".to_string()
        ),
        (
          OutPoint::from_str("3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:5")
            .unwrap(),
          10,
          90,
          Rarity::Common,
          "bvivuucdvvel".to_string()
        ),
        (
          OutPoint::from_str("3ce968df58f9c8a752306c4b7264afab93149dbc578bd08a42c446caaa6628bb:5")
            .unwrap(),
          1050000000000000,
          100000000000000,
          Rarity::Uncommon,
          "bnxlspjqjdjl".to_string()
        )
      ]
    )
  }
}
