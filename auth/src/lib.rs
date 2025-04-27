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
    pub action: u16, //TODO: use bitmap for permission.
}

impl Permission {
    pub const READ:   u16 = 1<<0;
    pub const CREATE: u16 = 1<<1;
    pub const UPDATE: u16 = 1<<2;
    pub const DELETE: u16 = 1<<3;

    const W_MASK: u16 = Self::UPDATE | Self::DELETE | Self::CREATE;
    const RW_MASK: u16 = Self::W_MASK | Self::READ;

    pub fn is_readonly(&self) -> bool {
        self.action & Self::RW_MASK == Self::READ
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
