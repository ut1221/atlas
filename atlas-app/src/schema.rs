// @generated automatically by Diesel CLI.

diesel::table! {
    commands (id) {
        id -> Nullable<Integer>,
        name -> Text,
        command_text -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

diesel::table! {
    configs (id) {
        id -> Nullable<Integer>,
        config_name -> Text,
        config_key -> Text,
        config_value -> Text,
        config_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

diesel::table! {
    node_commands (node_id, command_id) {
        node_id -> Nullable<Integer>,
        command_id -> Nullable<Integer>,
    }
}

diesel::table! {
    nodes (id) {
        id -> Nullable<Integer>,
        server_name -> Text,
        ip_address -> Text,
        port -> Integer,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

diesel::joinable!(node_commands -> commands (command_id));
diesel::joinable!(node_commands -> nodes (node_id));

diesel::allow_tables_to_appear_in_same_query!(
    commands,
    configs,
    node_commands,
    nodes,
);
