pub(crate) mod filter;
pub(crate) mod model;
pub(crate) mod order;

use uuid::Uuid;

/// Returns all array elements in a single [`String`], with each array element enclosed in single quotes (`'`) and enumerated with a comma (`,`).
///
/// Examles:
/// ```
/// // object_uuids -> result
/// &[A, B, C] -> String::from("'A', 'B', 'C'");
/// ```
/// After processing the `object_uuids` vec the `result` string will be returned.
/// A string with nil uuid will be returns for an empty array.
pub(crate) fn get_vec_in_string(object_uuids: &[Uuid]) -> String {
    if object_uuids.is_empty() {
        return String::from("'00000000-0000-0000-0000-000000000000'");
    }
    format!("{:?}", object_uuids)
        .replace('[', "\'")
        .replace(']', "\'")
        .replace(", ", "\', \'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuids_to_string() {
        let object_uuids = vec![
            Uuid::parse_str("0b837514-43f9-4bda-bf6c-bccacecb798c").unwrap_or_default(),
            Uuid::parse_str("54cbf609-60dc-4545-aec2-a052bceb0f46").unwrap_or_default(),
            Uuid::parse_str("62406e75-d14a-46af-87a6-6a3d840ff14c").unwrap_or_default(),
            Uuid::parse_str("2fc128a3-e234-478d-b472-faae86d28768").unwrap_or_default(),
            Uuid::parse_str("0c7410ad-2a3a-4dcb-bec5-44b3d31ba311").unwrap_or_default(),
            Uuid::parse_str("99e51255-47de-4171-8b61-4559c5d682a1").unwrap_or_default(),
            Uuid::parse_str("d8f26d8e-b7b9-4e98-a552-f7ec08d55156").unwrap_or_default(),
        ];
        let result = String::from("'0b837514-43f9-4bda-bf6c-bccacecb798c', '54cbf609-60dc-4545-aec2-a052bceb0f46', '62406e75-d14a-46af-87a6-6a3d840ff14c', '2fc128a3-e234-478d-b472-faae86d28768', '0c7410ad-2a3a-4dcb-bec5-44b3d31ba311', '99e51255-47de-4171-8b61-4559c5d682a1', 'd8f26d8e-b7b9-4e98-a552-f7ec08d55156'");
        assert_eq!(result, get_vec_in_string(&object_uuids));
    }

    #[test]
    fn empty_to_string() {
        let object_uuids = vec![];
        let result = String::from("'00000000-0000-0000-0000-000000000000'");
        assert_eq!(result, get_vec_in_string(&object_uuids));
    }
}
