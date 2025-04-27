pub mod schema;
mod models;

use diesel::prelude::*;
use crate::schema::user_roles;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = user_roles)]
pub struct UserRole {
    pub user_id: i64,
    pub role_id: String,
}

pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct RolePermission {
    pub role_id: String,
    pub permission_id: String,
}

pub struct Permission {
    pub id: String,
    pub resource: String,
    pub action: u16, //TODO: use bitmap for permission.
}


impl Permission {
    pub const READ:   u16 = 1<<0;
    pub const CREATE: u16 = 1<<1;
    pub const UPDATE: u16 = 1<<2;
    pub const DELETE: u16 = 1<<3;

    const W_MASK: u16 = Self::UPDATE | Self::DELETE | Self::CREATE;
    const RW_MASK: u16 = Self::W_MASK | Self::READ;

    #[inline]
    pub fn is_readonly(&self) -> bool {
        self.action & Self::RW_MASK == Self::READ
    }

    #[inline]
    pub fn is_write(&self) -> bool {
        self.action & Self::W_MASK == Self::W_MASK
    }

    #[inline]
    pub fn is_create(&self) -> bool {
        self.action & Self::CREATE == Self::CREATE
    }

    #[inline]
    pub fn is_delete(&self) -> bool {
        self.action & Self::DELETE == Self::DELETE
    }

    #[inline]
    pub fn is_update(&self) -> bool {
        self.action & Self::UPDATE == Self::UPDATE
    }

    #[inline]
    pub fn is_create_and_update(&self) -> bool {
        self.is_create() && self.is_update()
    }
}

use std::{collections::{HashMap, HashSet}, sync::RwLock};
pub struct PermissionStore {
    pub roles: RwLock<HashMap<String, Role>>,
    pub permissions: RwLock<HashMap<String, Permission>>,

    pub user_roles: RwLock<HashSet<UserRole>>,
    pub role_permissions: RwLock<HashSet<RolePermission>>,
}

impl PermissionStore {

}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use diesel::{insert_into, sql_query, Connection, RunQueryDsl, SqliteConnection};

    use crate::{schema::user_roles, UserRole};

    const DATABASE_URL:&str = "./target/test/sqlite/auth.db";

    fn establish_sqlite_connection(database_url: &str) -> SqliteConnection {
        let path = Path::new(database_url);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create database directory");
            }
        }
        SqliteConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    fn maybe_create_user_roles_table(conn: &mut SqliteConnection) {
        sql_query(
            r#"
            CREATE TABLE IF NOT EXISTS user_roles (
                user_id BIGINT PRIMARY KEY NOT NULL,
                role_id VARCHAR(32) NOT NULL
            );
            "#
        )
        .execute(conn)
        .unwrap_or_else(|_| panic!("Error creating subjects table"));
    }

    fn insert_user_roles(conn: &mut SqliteConnection, user_id: i64, role_id: &str) {
        let new_user_role = UserRole {
            user_id,
            role_id: role_id.to_string(),
        };
        insert_into(user_roles::table)
            .values(&new_user_role)
            .execute(conn)
            .unwrap_or_else(|_| panic!("Error insert zero user roles"));
    }

    #[test]
    fn test_sqlite_connection() {
        establish_sqlite_connection(DATABASE_URL);
    }

    #[test]
    fn test_insert_user_role() {
        let connection = &mut establish_sqlite_connection(DATABASE_URL);
        insert_user_roles(connection, 0, "root");
    }

    #[test]
    fn test_fetch_all_subjects() {
        use crate::schema::user_roles::dsl::*;
        let connection = &mut establish_sqlite_connection(DATABASE_URL);
        maybe_create_user_roles_table(connection);
        let results = user_roles.load::<UserRole>(connection).expect("Error loading all subjects");

        println!("=========");
        for subject in results {
            println!("User Roles {:#?}", subject);
        }
        println!(">>>>>>>>>");
    }
}
