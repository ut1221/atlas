#[cfg(test)]
mod tests {
    use diesel::dsl::insert_into;
    use diesel::prelude::*;
    use chrono::Utc;
    use crate::{establish_connection_pool, DbConn};
    use crate::models::nodes::Node;

    #[test]
    fn test_node_query() {
        use crate::schema::nodes::dsl::*;

        let pool = establish_connection_pool();
        let mut conn : DbConn = pool.get().map_err(|e| e.to_string()).unwrap();


        // 插入测试数据
        insert_into(nodes)
            .values((
                server_name.eq("Test Server"),
                ip_address.eq("192.168.1.2"),
                port.eq(22),
                username.eq(Some("admin")),
                password.eq(Some("password")),
                status.eq("active"),
                created_at.eq(Utc::now().naive_utc()),
                updated_at.eq(Utc::now().naive_utc()),
                deleted.eq(false),
            ))
            .execute(&mut conn)
            .unwrap();

        // 查询节点
        let results = nodes
            .filter(server_name.eq("Test Server"))
            .load::<Node>(&mut conn)
            .expect("Error loading nodes");

        // 检查查询结果
        assert_eq!(results.len(), 1);

        let node = &results[0];
        assert_eq!(node.server_name, "Test Server");
        assert_eq!(node.ip_address, "192.168.1.2");
        assert_eq!(node.port, 22);
        assert_eq!(node.username.as_deref(), Some("admin"));
        assert_eq!(node.password.as_deref(), Some("password"));
        assert_eq!(node.status, "active");
        assert!(!node.deleted);
    }
}
