CREATE TABLE nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_name TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    port INTEGER DEFAULT 22 NOT NULL,
    username VARCHAR(200) NULL,
    password VARCHAR(200) NULL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted INTEGER DEFAULT 0 NOT NULL
);

CREATE TABLE commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    command TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted INTEGER DEFAULT 0 NOT NULL
);

CREATE TABLE node_commands (
    node_id INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    command_id INTEGER REFERENCES commands(id) ON DELETE CASCADE,
    PRIMARY KEY (node_id, command_id)
);

CREATE TABLE configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    config_name TEXT NOT NULL,
    config_key TEXT NOT NULL,
    config_value TEXT NOT NULL,
    config_type TEXT NOT NULL, -- 标记配置类型，如 'account_password' 或 'other_data'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted INTEGER DEFAULT 0 NOT NULL
);
