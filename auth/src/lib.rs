#[allow(unused)]

pub struct Subject {
    pub id: u64,
    pub subject_type: String,
}

pub struct SubjectRole {
    pub subject_id: u64,
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

pub struct PermissionStore {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
