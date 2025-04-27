#[allow(unused)]

pub enum UserStatus {
    CreatePending,
    UpdatePending,
    Suspend,
    Disabled,
    Active,
    Deleted,
}
pub struct BasicUser {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
}

pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct Permission {
    pub resource: String,
    pub action: u32, //TODO: use bitmap for permission.
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
