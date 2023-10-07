use std::os::unix::fs::PermissionsExt;

pub struct Permission(u32);

impl Permission {
    fn readable(&self) -> bool {
        self.0 & 0o4 > 0
    }

    fn writable(self) -> bool {
        self.0 & 0o2 > 0
    }

    fn executable(self) -> bool {
        self.0 & 0o1 > 0
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
