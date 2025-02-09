use super::*;

#[derive(Boilerplate)]
pub(crate) struct ClockSvg {
  height: Height,
  hour: f64,
  minute: f64,
  second: f64,
}

impl ClockSvg {
  pub(crate) fn new(height: Height) -> Self {
    let min = height.min(Epoch::FIRST_POST_SUBSIDY.starting_height());

    Self {
      height,
      hour: f64::from(min.n() % Epoch::FIRST_POST_SUBSIDY.starting_height().n())
        / f64::from(Epoch::FIRST_POST_SUBSIDY.starting_height().n())
        * 360.0,
      minute: f64::from(min.n() % SUBSIDY_HALVING_INTERVAL) / f64::from(SUBSIDY_HALVING_INTERVAL)
        * 360.0,
      second: f64::from(height.period_offset()) / f64::from(DIFFCHANGE_INTERVAL) * 360.0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn second() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).second, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(504)).second, 90.0);
    pretty_assert_eq!(ClockSvg::new(Height(1008)).second, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(1512)).second, 270.0);
    pretty_assert_eq!(ClockSvg::new(Height(2016)).second, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930000)).second, 180.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930504)).second, 270.0);
  }

  #[test]
  fn minute() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(52500)).minute, 18.0);
    pretty_assert_eq!(ClockSvg::new(Height(105000)).minute, 36.0);
    pretty_assert_eq!(ClockSvg::new(Height(157500)).minute, 54.0);
    pretty_assert_eq!(ClockSvg::new(Height(210000)).minute, 72.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930000)).minute, 216.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930001)).minute, 216.00034285714287);
  }

  #[test]
  fn hour() {
    pretty_assert_eq!(ClockSvg::new(Height(0)).hour, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(1732500)).hour, 18.0);
    pretty_assert_eq!(ClockSvg::new(Height(3465000)).hour, 36.0);
    pretty_assert_eq!(ClockSvg::new(Height(5197500)).hour, 54.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930000)).hour, 72.0);
    pretty_assert_eq!(ClockSvg::new(Height(6930001)).hour, 72.00001038961038);
  }

  #[test]
  fn final_subsidy_height() {
    pretty_assert_eq!(
      ClockSvg::new(Height(9078423)).second,
      1007.0 / 2016.0 * 360.0
    );
    pretty_assert_eq!(
      ClockSvg::new(Height(9078423)).minute,
      209_999.8 / 210_000.0 * 360.0
    );
    pretty_assert_eq!(
      ClockSvg::new(Height(9078423)).hour,
      9078423.0 / 9078424.0 * 360.0
    );
  }

  #[test]
  fn first_post_subsidy_height() {
    pretty_assert_eq!(ClockSvg::new(Height(34650000)).second, 240.0);
    pretty_assert_eq!(ClockSvg::new(Height(34650000)).minute, 0.0);
    pretty_assert_eq!(ClockSvg::new(Height(34650000)).hour, 0.0);
  }

  #[test]
  fn clock_svg() {
    assert_regex_match!(
      ClockSvg::new(Height(6929999)).to_string(),
      r##"<\?xml version="1.0" encoding="UTF-8"\?>
<svg.*>.*
  <text.*>6929999</text>.*
  <line y2="-9" transform="rotate\(71.9999896103896\)"><title>Subsidy</title></line>.*
  <line y2="-13" stroke-width="0.6" transform="rotate\(215.99965714285716\)"><title>Epoch</title></line>.*
  <line y2="-16" stroke="#d00505" stroke-width="0.2" transform="rotate\(179.82142857142858\)"><title>Period</title></line>.*
  <circle r="0.7" stroke="#d00505" stroke-width="0.3"/>.*
</svg>
"##,
    );
  }
}
