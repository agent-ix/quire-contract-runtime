//! Point-only view of the kernel's admitted-charge records for schedule assertions.

use quire_exact::{ChargePoint, Meter};

pub trait ChargePoints {
    fn admitted_charge_points(&self) -> Vec<ChargePoint>;
}

impl ChargePoints for Meter {
    fn admitted_charge_points(&self) -> Vec<ChargePoint> {
        self.admitted_charges()
            .iter()
            .map(|charge| charge.point)
            .collect()
    }
}
