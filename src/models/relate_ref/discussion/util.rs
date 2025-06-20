use uuid::Uuid;

lazy_static::lazy_static! {
    // static ref ROOT_DISCUSSION_UUID : Uuid =
    //     Uuid::parse_str("ed5baabe-7201-45d3-9ec8-e610839feb39")
    //         .expect("Set root discussion uuid failed!");

    static ref ROOT_DISCUSSION_COMMENT_UUID : Uuid =
        Uuid::parse_str("dd92b579-019b-48ed-a9e2-28131dbdd363")
            .expect("Set root discussion comment uuid failed!");
}

// /// Retund default discussion
// pub(crate) fn get_root_discussion_uuid() -> Uuid {
//     *ROOT_DISCUSSION_UUID
// }

/// Retund default discussion comment
pub(crate) fn get_root_discussion_comment_uuid() -> Uuid {
    *ROOT_DISCUSSION_COMMENT_UUID
}

