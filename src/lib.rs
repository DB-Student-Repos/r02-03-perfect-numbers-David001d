 #[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    
    let aliquot_sum: u64 = (1..num).filter(|&x| num % x == 0).sum();
    
    match aliquot_sum {
        sum if sum == num => Some(Classification::Perfect),
        sum if sum > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
}
