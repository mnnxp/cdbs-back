// pub(crate) fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
//     if s.len() % 2 == 0 {
//         (0..s.len())
//             .step_by(2)
//             .map(|i| s.get(i..i + 2)
//                       .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
//             .collect()
//     } else {
//         None
//     }
// }


// let caps = Regex::new(r"\.\w*$").unwrap().captures(&filename).unwrap();
// let value_ext_str = caps.get(1).map_or("", |m| m.as_str()).to_owned();

// debug!("value_ext_str: {:?}", &value_ext_str);

// let conn = &db_connection(&pool)?;
// use crate::schema::extension_ref::dsl::*;

// let id_ext = extension_ref
//     .filter(extension.eq(value_ext_str))
//     .select(id)
//     .first(conn).unwrap_or(1); // this to fix after
