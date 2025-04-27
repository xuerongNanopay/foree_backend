diesel::table! {
    user_roles (user_id) {
        user_id -> BigInt,
        role_id -> Text,
        status -> Text,
    }
}
