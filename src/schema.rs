table! {
    cans (can_id) {
        can_id -> Integer,
        user_id -> BigInt,
        user -> Text,
        date -> Timestamp,
    }
}

table! {
    quotes (quote_id) {
        quote_id -> Integer,
        message -> Text,
        quote_author_id -> BigInt,
        quote_author -> Text,
        date -> Timestamp,
    }
}

table! {
    suggestions (suggestion_id) {
        suggestion_id -> Integer,
        suggestion_text -> Text,
        suggestion_date -> Timestamp,
        suggestion_author_id -> BigInt,
        suggestion_message_id -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(
    cans,
    quotes,
    suggestions,
);
