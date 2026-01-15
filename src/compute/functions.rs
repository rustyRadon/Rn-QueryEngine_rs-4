use crate::storage::column::ColumnVault;
use crate::util::bitmask::BitMask;

pub fn crunch_float_sum(vault: &ColumnVault, mask: &BitMask) -> f64 {
    if let ColumnVault::MoneyAndScores(data) = vault {
        data.iter()
            .enumerate()
            .filter(|(i, _)| mask.get(*i))
            .map(|(_, &val)| val)
            .sum()
    } else {
        0.0
    }
}

#[allow(dead_code)]
pub fn screen_for_matches<F>(vault: &ColumnVault, mut rule: F) -> BitMask 
where 
    F: FnMut(i32) -> bool 
{
    if let ColumnVault::WholeNumbers(data) = vault {
        let mut mask = BitMask::new(data.len());
        for (i, &val) in data.iter().enumerate() {
            if rule(val) {
                mask.set(i);
            }
        }
        mask
    } else {
        BitMask::new(0)
    }
}

#[allow(dead_code)]
pub fn calculate_average(vault: &ColumnVault, mask: &BitMask) -> f64 {
    let count = mask.count_active();
    if count == 0 {
        return 0.0;
    }
    let total = crunch_float_sum(vault, mask);
    total / (count as f64)
}