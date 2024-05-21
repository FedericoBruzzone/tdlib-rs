// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2024 - developers of the `tgt` and `tdlib-rs` projects.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Code to generate Rust's `enum`'s from TL definitions.

use crate::ignore_type;
use crate::metadata::Metadata;
use crate::rustifier;
use std::io::{self, Write};
use tdlib_rs_parser::tl::{Category, Definition, Type};

/// Writes an enumeration listing all types such as the following rust code:
///
/// ```ignore
/// pub enum Name {
///     Variant(crate::types::Name),
/// }
/// ```
fn write_enum<W: Write>(
    file: &mut W,
    ty: &Type,
    metadata: &Metadata,
    gen_bots_only_api: bool,
) -> io::Result<()> {
    writeln!(
        file,
        "    #[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]",
    )?;
    writeln!(file, "    #[serde(tag = \"@type\")]")?;
    writeln!(file, "    pub enum {} {{", rustifier::types::type_name(ty))?;
    for d in metadata.defs_with_type(ty) {
        if rustifier::definitions::is_for_bots_only(d) && !gen_bots_only_api {
            continue;
        }

        writeln!(
            file,
            "{}",
            rustifier::definitions::description(d, "        ")
        )?;
        writeln!(
            file,
            "        #[serde(rename(serialize = \"{0}\", deserialize = \"{0}\"))]",
            d.name
        )?;
        write!(file, "        {}", rustifier::definitions::variant_name(d))?;

        // Variant with no struct since it has no data and it only adds noise
        if d.params.is_empty() {
            writeln!(file, ",")?;
            continue;
        } else {
            write!(file, "(")?;
        }

        if metadata.is_recursive_def(d) {
            write!(file, "Box<")?;
        }
        write!(file, "{}", rustifier::definitions::qual_name(d))?;
        if metadata.is_recursive_def(d) {
            write!(file, ">")?;
        }

        writeln!(file, "),")?;
    }
    writeln!(file, "    }}")?;
    Ok(())
}

/// Write the entire module dedicated to enums.
pub(crate) fn write_enums_mod<W: Write>(
    mut file: &mut W,
    definitions: &[Definition],
    metadata: &Metadata,
    gen_bots_only_api: bool,
) -> io::Result<()> {
    // Begin outermost mod
    writeln!(file, "#[allow(clippy::all)]")?;
    writeln!(file, "pub mod enums {{")?;
    writeln!(file, "    use serde::{{Deserialize, Serialize}};")?;

    let mut enums: Vec<&Type> = definitions
        .iter()
        .filter(|d| d.category == Category::Types && !ignore_type(&d.ty))
        .map(|d| &d.ty)
        .collect();
    enums.dedup();

    for ty in enums {
        write_enum(&mut file, ty, metadata, gen_bots_only_api)?;
    }

    // End outermost mod
    writeln!(file, "}}")
}
