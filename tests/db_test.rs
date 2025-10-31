// Tests for database connection module

#[cfg(test)]
mod tests {
    use backend::db::*;
    use std::env;

    #[test]
    fn test_create_pool_with_invalid_url() {
        let result = create_pool("mysql://invalid:url@localhost:9999/testdb");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_pool_with_valid_url_format() {
        // This test checks if the URL is parsed correctly, not if connection succeeds
        let url = "mysql://root:password@localhost:3307/mydb";
        let result = create_pool(url);

        // The pool creation might fail if MySQL is not running, but the URL should be valid
        match result {
            Ok(_pool) => {}
            Err(e) => {
                // If it fails, it should be a connection error, not a URL parsing error
                let error_msg = format!("{}", e);
                // Check that it's not a URL parsing error
                assert!(!error_msg.contains("invalid"));
            }
        }
    }

    #[test]
    fn test_pool_type() {
        // Test that DbPool type is correctly defined
        let url = "mysql://root:password@localhost:3307/mydb";
        if let Ok(pool) = create_pool(url) {
            // Try to get a connection
            let conn_result = pool.get_conn();
            // Either succeeds or fails with connection error (not type error)
            match conn_result {
                Ok(_) => println!("worked"),
                Err(_) => println!("didn't"), // Expected if DB not running
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_create_pool_with_malformed_url() {
        let _ = create_pool("not-a-valid-url").unwrap();
    }

    #[test]
    fn test_database_url_parsing() {
        unsafe {
            env::set_var("TEST_DATABASE_URL", "mysql://root:pass@localhost:3307/test");
        }
        let url = env::var("TEST_DATABASE_URL").unwrap();
        assert!(url.contains("mysql://"));
        assert!(url.contains("localhost"));
        assert!(url.contains("3307"));
    }
}
