use colored::*;

pub struct Tabulator {
  header: Vec<String>,
  rows: Vec<Vec<String>>,
  column_widths: Vec<usize>,
}


impl Tabulator {
  pub fn new(header: Vec<String>) -> Tabulator {
    let column_widths = header.iter().map(|s| s.len()).collect();
    return Tabulator{header, column_widths, rows: Vec::new()}
  }

  pub fn append(&mut self, row: Vec<String>) {
    for i in 0..self.header.len() {
      if let Some(s) = row.get(i) {
        let len = s.len();
        if len > self.column_widths[i] {
          self.column_widths[i] = len;
        }
      }
    }
    self.rows.push(row);
  }

  pub fn display(&self) {
    let text_len: usize = self.column_widths.iter().sum();
    let margin_len = 3 * self.column_widths.len() - 1;
    let top = std::iter::repeat("_").take(text_len + margin_len).collect::<String>();
    println!("+{}+", top);
    self.display_row(&self.header);
    self.display_blank_row();
    for row in &self.rows {
      self.display_row(row);
    }
    let bottom = std::iter::repeat("-").take(text_len + margin_len).collect::<String>();
    println!("+{}+", bottom);
  }

  fn display_row(&self, row: &Vec<String>) {
    let mut result = "|".to_string();
    for (i, s) in row.iter().enumerate() {
      result.push_str(&format!(" {} ", s.cyan()));
      let padding_len = self.column_widths[i] - s.len();
      let padding = std::iter::repeat(" ").take(padding_len).collect::<String>();
      result.push_str(&padding);
      result.push_str("|");
    }
    println!("{}", result);
  }

  fn display_blank_row(&self) {
    let mut result = "|".to_string();
    for width in &self.column_widths {
      let padding_len = width + 2;
      let padding = std::iter::repeat("-").take(padding_len).collect::<String>();
      result.push_str(&padding);
      result.push_str("|");
    }
    println!("{}", result);
  }
}
