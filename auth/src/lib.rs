#[allow(unused)]

pub struct Subject {
    pub id: u64,
    pub subject_type: String,
}

pub struct Role {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

pub struct Permission {
    pub id: u64,
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

    pub fn is_write(&self) -> bool {
        self.action & Self::W_MASK == Self::W_MASK
    }
}

pub struct SubjectRole {
    pub subject_id: u64,
    pub role_id: u64,
}

pub struct RolePermission {
    pub role_id: u64,
    pub permission_id: u64,
}

pub struct PermissionStore {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
