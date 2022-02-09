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

allow_tables_to_appear_in_same_query!(
    cans,
    quotes,
);
