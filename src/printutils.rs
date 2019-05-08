use prettytable::*;
use crate::Record;
use std::string::ToString;

pub fn table_record(package: &str, record: &Record) {

        let mut table = table!(
            [FYbH2c -> "Global Package Index Record"],
            [FYb -> "Package",     Fwb -> package],
            [FYb -> "Type",   Fwb ->  record.rtype.to_string()],
            [FYbH2c -> "Sources"]
        );
        for source in &record.sources {
            //  table.add_row(
            //     row![
            //         FybH2c ->"-----------------------------------------------------------------"
            //     ]
            // );
            table.add_row(
                row![
                    Fyb -> "Uses",
                    Fwb -> source.uses
                ]
            );
            table.add_row(
                row![
                    Fyb -> "Link",
                    Fwb -> source.link
                ]
            );

            table.add_row(
                row![
                    Fyb -> "tags",
                    Fwb -> source.tags
                ]
            );
            if let Some(val) = &source.subdirectory {
                table.add_row(
                    row![
                        Fyb -> "Subdirectory",
                        Fwb -> val.as_path().to_str().unwrap()
                    ]
                );
            }
            if let Some(val) = &source.init_submodules {
                table.add_row(
                    row![
                        Fyb -> "initSubmodules",
                        Fwb -> val
                    ]
                );
            }
        }
        // FORMAT_CLEAN
        // FORMAT_NO_COLSEP
        // FORMAT_BORDERS_ONLY
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
        println!("");
        table.printstd();
        println!("");
}