use serde::Deserialize;

#[derive(Deserialize)]
struct SchemaField {
    name: String,
}

#[derive(Deserialize)]
struct Schema {
    name: String,
    fields: Vec<SchemaField>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let schema_path = &args[1];
    let out_path = &args[2];

    // generate rust structs
    let paths = std::fs::read_dir(&schema_path).unwrap();

    let mut modules = Vec::new();

    for sheet in paths {
        if !sheet.as_ref().unwrap().metadata().unwrap().is_file() {
            continue;
        }

        let schema_sheet = std::fs::read_to_string(&sheet.unwrap().path()).unwrap();
        let schema: Schema = serde_yaml_ng::from_str(&schema_sheet).unwrap();

        let mut output_str = String::default();

        // quiet rust up
        output_str.push_str("#![allow(warnings)]\n");

        output_str.push_str("/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.\n");
        output_str.push_str("use physis::{gamedata::GameData, exd::{EXD, ColumnData, ExcelRowKind}, exh::{EXH, ExcelColumnDefinition}, common::Language};\n");

        // sheet struct
        output_str.push_str(&format!("pub struct {} {{\n", schema.name));
        output_str.push_str("exd: EXD,\n");
        output_str.push_str("exh: EXH,\n");
        output_str.push_str("}\n");

        // sheet struct impl
        output_str.push_str(&format!("impl {} {{\n", schema.name));

        // read function
        output_str
            .push_str("pub fn read_from(game_data: &mut GameData, language: Language) -> Self {\n");

        output_str.push_str(&format!(
            "let exh = game_data.read_excel_sheet_header(\"{}\").unwrap();",
            schema.name
        ));
        output_str.push_str(&format!(
            "let exd = game_data.read_excel_sheet(\"{}\", &exh, language, 0).unwrap();",
            schema.name
        ));

        output_str.push_str("Self {\n");
        output_str.push_str("exh,\n");
        output_str.push_str("exd,\n");
        output_str.push_str("}\n");
        output_str.push_str("}\n");

        // get row function
        output_str.push_str(&format!(
            "pub fn get_row(&self, id: u32) -> {}Row {{",
            schema.name
        ));

        // TODO: only supports a single row for now
        output_str.push_str("let ExcelRowKind::SingleRow(row) = &self.exd.get_row(id).unwrap() else { panic!(\"Expected a single row!\"); };\n");

        // EXDSchema's fields are sorted by column offset. so we have to re-sort it to match
        output_str.push_str("let column_defs = &self.exh.column_definitions;\n");
        output_str.push_str("let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();\n");
        output_str.push_str("zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));\n");
        output_str.push_str("let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();\n");
        output_str.push_str(&format!(
            "{}Row {{ columns }}\n",
            schema.name
        ));

        output_str.push_str("}\n");
        output_str.push_str("}\n");

        // row struct
        output_str.push_str(&format!("pub struct {}Row {{\n", schema.name));
        output_str.push_str("columns: Vec<ColumnData>,\n");
        output_str.push_str("}\n");

        // row struct impl
        output_str.push_str(&format!("impl {}Row {{\n", schema.name));

        let mut i = 0;
        for field in schema.fields {
            // function
            output_str.push_str(&format!("pub fn {}(&self) -> &ColumnData {{\n", field.name));
            output_str.push_str(&format!("&self.columns[{}]\n", i));
            output_str.push_str("}\n");

            i += 1;
        }

        output_str.push_str("}\n");

        std::fs::write(&format!("{}/src/{}.rs", out_path, schema.name), output_str)
            .expect("Failed to write opcodes file!");
        modules.push(schema.name);
    }

    // generate mod file
    {
        let mut output_str = String::default();

        // rust will HATE us!
        output_str.push_str("#![allow(warnings)]\n");

        for module in &modules {
            output_str.push_str(&format!("#[cfg(feature = \"{}\")]\n", module));
            output_str.push_str(&format!("pub mod {};\n", module));
        }

        std::fs::write(&format!("{}/src/lib.rs", out_path), output_str)
            .expect("Failed to write opcodes file!");
    }

    // generate Cargo.toml
    {
        let mut output_str = String::default();

        output_str.push_str("[package]\n");
        output_str.push_str("name = \"physis-sheets\"\n");
        output_str.push_str("edition = \"2024\"\n");
        output_str.push_str("[features]\n");
        output_str.push_str(&format!("default = [{}]\n", modules.iter().map(|x| format!("\"{}\"", x)).collect::<Vec<String>>().join(",")));

        for module in modules {
            output_str.push_str(&format!("{} = []\n", module));
        }

        output_str.push_str("[dependencies]\n");
        output_str.push_str("physis = { git = \"https://github.com/redstrate/physis\" }\n");

        std::fs::write(&format!("{}/Cargo.toml", out_path), output_str)
        .expect("Failed to write opcodes file!");
    }
}
