use chrono::DateTime;
use colored::*;
use term_table::{row::Row, table_cell::Alignment, table_cell::TableCell};
use term_table::{TableBuilder, TableStyle};
use textwrap;

/// render creates an ascii table from a `Doc`.
pub fn render(doc: &crate::forecast::Doc, style: TableStyle) -> String {
    let footer = Row::new(vec![
        TableCell::new("Date"),
        TableCell::new("Day"),
        TableCell::new("Time"),
        TableCell::new_with_alignment("Temp", 1, Alignment::Right),
        TableCell::new_with_alignment("Precip", 1, Alignment::Right),
        TableCell::new("Wind"),
        TableCell::new("Forecast"),
    ]);

    let mut rows: Vec<Row> = Vec::new();

    rows.extend(doc.properties.periods.iter().rev().map(|p| {
        let parts = p
            .start_time
            .split_once("T")
            .unwrap_or(("Unknown", "Unknown"));

        let date_default = parts.0.to_owned();
        let day_default = "Unknown".to_owned();
        let time_default = parts.1.to_owned();

        let parsed_date = DateTime::parse_from_rfc3339(&p.start_time);

        let date = parsed_date.map_or(date_default, |d| d.format("%m/%d").to_string());
        let day = parsed_date.map_or(day_default, |d| d.format("%A").to_string());
        let time = parsed_date.map_or(time_default, |d| d.format("%I %p").to_string());

        let mut temperature = format!("{} F", p.temperature);
        temperature = if p.is_daytime {
            temperature.red().to_string()
        } else {
            temperature.blue().to_string()
        };

        let short = textwrap::wrap(&p.short_forecast, 60).join("\n");
        let wind = format!("{} {}", p.wind_direction, p.wind_speed);
        let precip = p
            .probability_of_precipitation
            .value
            .map_or("".to_string(), |v| format!("{}%", v).green().to_string());

        Row::new(vec![
            TableCell::new(date),
            TableCell::new(day),
            TableCell::new(time),
            TableCell::new_with_alignment(temperature, 1, Alignment::Right),
            TableCell::new_with_alignment(precip, 1, Alignment::Right),
            TableCell::new(wind),
            TableCell::new(short),
        ])
    }));

    rows.push(footer);

    TableBuilder::new().style(style).rows(rows).build().render()
}
