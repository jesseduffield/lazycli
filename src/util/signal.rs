// not sure if we actually need this file but keeping it around in case we do.

#[derive(Clone)]
pub struct SinSignal {
  x: f64,
  interval: f64,
  period: f64,
  scale: f64,
}

impl SinSignal {
  pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
    SinSignal {
      x: 0.0,
      interval,
      period,
      scale,
    }
  }
}

impl Iterator for SinSignal {
  type Item = (f64, f64);
  fn next(&mut self) -> Option<Self::Item> {
    let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
    self.x += self.interval;
    Some(point)
  }
}
