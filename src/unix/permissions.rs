use std::{ops::BitAnd, os::unix::fs::PermissionsExt};

enum PermissionFlag {
    READ = 0o4,
    WRITE = 0o2,
    EXECUTE = 0o1,
}

impl BitAnd<PermissionFlag> for u32 {
    type Output = u32;

    fn bitand(self, rhs: PermissionFlag) -> Self::Output {
        self & rhs as u32
    }
}

pub struct Permission(u32);

impl Permission {
    pub fn readable(&self) -> bool {
        self.0 & PermissionFlag::READ > 0
    }

    pub fn writable(&self) -> bool {
        self.0 & PermissionFlag::WRITE > 0
    }

    pub fn executable(&self) -> bool {
        self.0 & PermissionFlag::EXECUTE > 0
    }
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.readable() { 'r' } else { '-' },
            if self.writable() { 'w' } else { '-' },
            if self.executable() { 'x' } else { '-' }
        )
    }
}

pub trait UnixPermissions {
    fn owner(&self) -> Permission;
    fn group(&self) -> Permission;
    fn other(&self) -> Permission;
}

impl UnixPermissions for std::fs::Permissions {
    fn owner(&self) -> Permission {
        Permission((self.mode() & 0o700) >> 6)
    }

    fn group(&self) -> Permission {
        Permission((self.mode() & 0o70) >> 3)
    }

    fn other(&self) -> Permission {
        Permission(self.mode() & 0o7)
    }
}
