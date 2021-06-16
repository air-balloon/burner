table! {
    use diesel::sql_types::*;
    use crate::models::enums::*;

    users (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        account_status -> AccountStatusMapping,
        timezone -> Nullable<Varchar>,
        first_log_in_at -> Nullable<Timestamp>,
        last_log_in_at -> Nullable<Timestamp>,
        language -> Nullable<Varchar>,
    }
}
