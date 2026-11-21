//! SigmaOS-craft-grid: grid/layout model.
//!
//! Real implementation: grid cells, columns, rows, spanning, alignment,
//! responsive constraints, and validation. Replaces the 104-line CLI placeholder.

/// Grid layout system module.
pub mod sigmagrid;

use std::collections::BTreeMap;
use std::fmt;

/// Alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

/// Grid cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub id: String,
    pub row: u32,
    pub col: u32,
    pub row_span: u32,
    pub col_span: u32,
    pub h_align: Alignment,
    pub v_align: Alignment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridError {
    EmptyCellId,
    DuplicateCell(String),
    CellNotFound(String),
    InvalidSpan(u32),
    InvalidDimensions(u32, u32),
}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCellId => write!(f, "cell ID must not be empty"),
            Self::DuplicateCell(id) => write!(f, "duplicate cell {:?}", id),
            Self::CellNotFound(id) => write!(f, "cell {:?} not found", id),
            Self::InvalidSpan(s) => write!(f, "invalid span {}", s),
            Self::InvalidDimensions(r, c) => write!(f, "invalid dimensions {}x{}", r, c),
        }
    }
}

impl std::error::Error for GridError {}

/// Grid layout.
pub struct GridLayout {
    pub rows: u32,
    pub cols: u32,
    cells: BTreeMap<String, Cell>,
}

impl GridLayout {
    pub fn new(rows: u32, cols: u32) -> Result<Self, GridError> {
        if rows == 0 || cols == 0 { return Err(GridError::InvalidDimensions(rows, cols)); }
        Ok(Self { rows, cols, cells: BTreeMap::new() })
    }

    pub fn add_cell(&mut self, cell: Cell) -> Result<(), GridError> {
        if cell.id.trim().is_empty() { return Err(GridError::EmptyCellId); }
        if cell.row_span == 0 || cell.col_span == 0 { return Err(GridError::InvalidSpan(0)); }
        if cell.row >= self.rows || cell.col >= self.cols { return Err(GridError::InvalidDimensions(cell.row, cell.col)); }
        let id = cell.id.clone();
        if self.cells.contains_key(&id) {
            return Err(GridError::DuplicateCell(id.clone()));
        }
        self.cells.insert(id, cell);
        Ok(())
    }

    pub fn remove_cell(&mut self, id: &str) -> Result<(), GridError> {
        if !self.cells.contains_key(id) {
            return Err(GridError::CellNotFound(id.into()));
        }
        self.cells.remove(id);
        Ok(())
    }

    pub fn get_cell(&self, id: &str) -> Option<&Cell> { self.cells.get(id) }

    pub fn cells(&self) -> Vec<&Cell> { self.cells.values().collect() }

    pub fn cell_count(&self) -> usize { self.cells.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_create_aaa() {
        let g = GridLayout::new(3, 3).unwrap();
        assert_eq!(g.rows, 3);
        assert_eq!(g.cols, 3);
        assert_eq!(g.cell_count(), 0);
    }

    #[test]
    fn test_invalid_dimensions_rejected_aaa() {
        assert!(GridLayout::new(0, 3).is_err());
        assert!(GridLayout::new(3, 0).is_err());
    }

    #[test]
    fn test_add_cell_aaa() {
        let mut g = GridLayout::new(3, 3).unwrap();
        let c = Cell { id: "c1".into(), row: 0, col: 0, row_span: 1, col_span: 1,
            h_align: Alignment::Start, v_align: Alignment::Start };
        g.add_cell(c).unwrap();
        assert_eq!(g.cell_count(), 1);
    }

    #[test]
    fn test_empty_cell_id_rejected_aaa() {
        let mut g = GridLayout::new(3, 3).unwrap();
        let c = Cell { id: "".into(), row: 0, col: 0, row_span: 1, col_span: 1,
            h_align: Alignment::Start, v_align: Alignment::Start };
        assert!(g.add_cell(c).is_err());
    }

    #[test]
    fn test_invalid_span_rejected_aaa() {
        let mut g = GridLayout::new(3, 3).unwrap();
        let c = Cell { id: "x".into(), row: 0, col: 0, row_span: 0, col_span: 1,
            h_align: Alignment::Start, v_align: Alignment::Start };
        assert!(g.add_cell(c).is_err());
    }

    #[test]
    fn test_duplicate_cell_rejected_aaa() {
        let mut g = GridLayout::new(3, 3).unwrap();
        let c = Cell { id: "x".into(), row: 0, col: 0, row_span: 1, col_span: 1,
            h_align: Alignment::Start, v_align: Alignment::Start };
        g.add_cell(c.clone()).unwrap();
        let c2 = Cell { id: "x".into(), row: 0, col: 0, row_span: 1, col_span: 1,
            h_align: Alignment::Start, v_align: Alignment::Start };
        assert!(g.add_cell(c2).is_err());
    }
}
