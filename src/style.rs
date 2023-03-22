use term_table::TableStyle;

/// simple exposes `TableStyle::simple()`.
pub fn simple() -> TableStyle {
    TableStyle::simple()
}

/// extended exposes `TableStyle::extended()`.
pub fn extended() -> TableStyle {
    TableStyle::extended()
}

/// elegant_style is smoother than `TableStyle::simple` and less cluttered than
/// `TableStyle::extended`.
pub fn elegant() -> TableStyle {
    TableStyle {
        top_left_corner: '┌',
        top_right_corner: '┐',
        bottom_left_corner: '└',
        bottom_right_corner: '┘',
        outer_left_vertical: '├',
        outer_right_vertical: '┤',
        outer_bottom_horizontal: '┴',
        outer_top_horizontal: '┬',
        intersection: '┼',
        vertical: '│',
        horizontal: '─',
    }
}
