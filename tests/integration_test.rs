use sigmaos_craft_grid::{Cell, Alignment, GridLayout};

#[test]
fn grid_layout_integration_aaa() {
    let mut g = GridLayout::new(3, 3).unwrap();
    let c = Cell { id: "c1".into(), row: 0, col: 0, row_span: 1, col_span: 1,
        h_align: Alignment::Center, v_align: Alignment::Center };
    g.add_cell(c).unwrap();
    assert_eq!(g.cell_count(), 1);
    assert!(g.get_cell("c1").is_some());
}
