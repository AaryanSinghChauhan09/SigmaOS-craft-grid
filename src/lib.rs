#[derive(Debug, Clone)]
pub struct GridSystem { pub cols: u32, pub gutter: f32 }
impl GridSystem {
    pub fn new(cols: u32, gutter: f32) -> Self { Self { cols, gutter } }
    pub fn col_width(&self, total_width: f32) -> f32 {
        let usable = total_width - (self.cols - 1) as f32 * self.gutter;
        usable / self.cols as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_grid() {
        let g = GridSystem::new(12, 10.0);
        assert!(g.col_width(1200.0) > 0.0);
    }
}
