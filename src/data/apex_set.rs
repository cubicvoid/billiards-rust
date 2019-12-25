#[derive(Debug)]
pub struct Metadata {
  pub count: i32,
}

#[derive(Debug)]
pub struct ApexSet {
  pub metadata: Metadata,
}

pub fn new_apex_set() -> ApexSet {
  ApexSet {
    metadata: Metadata {
      count: 100
    }
  }
}

