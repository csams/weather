use colored::*;
use term_table::{row::Row, table_cell::Alignment, table_cell::TableCell};
use term_table::{TableBuilder, TableStyle};
use textwrap;

/// render creates an ascii table from a `Doc`.
pub fn render(doc: &crate::forecast::Doc, style: TableStyle) -> String {
    let header = Row::new(vec![
        TableCell::new("Period"),
        TableCell::new_with_alignment("Temp", 1, Alignment::Right),
        TableCell::new_with_alignment("Precip", 1, Alignment::Right),
        TableCell::new("Forecast"),
        TableCell::new("Wind"),
        TableCell::new("Details"),
    ]);

    let mut rows = vec![header];

    rows.extend(doc.properties.periods.iter().map(|p| {
        let mut temp = format!("{} F", p.temperature);
        temp = if p.is_daytime {
            temp.red().to_string()
        } else {
            temp.blue().to_string()
        };

        let short = textwrap::wrap(&p.short_forecast, 60).join("\n");
        let detail = textwrap::wrap(&p.detailed_forecast, 70).join("\n");
        let wind = format!("{} {}", p.wind_direction, p.wind_speed);
        let precip = p
            .probability_of_precipitation
            .value
            .map_or("".to_string(), |v| format!("{}%", v).green().to_string());

        Row::new(vec![
            TableCell::new(&p.name),
            TableCell::new_with_alignment(temp, 1, Alignment::Right),
            TableCell::new_with_alignment(precip, 1, Alignment::Right),
            TableCell::new(short),
            TableCell::new(wind),
            TableCell::new(detail),
        ])
    }));

    TableBuilder::new().style(style).rows(rows).build().render()
}
