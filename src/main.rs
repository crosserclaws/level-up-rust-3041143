use std::{path, os::unix::prelude::PermissionsExt};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        self.metadata()
            .map(|m| m.permissions().mode() & 0o400 > 0)
            .unwrap_or(false)
    }

    fn is_writeable(&self) -> bool {
        self.metadata()
            .map(|m| m.permissions().mode() & 0o200 > 0)
            .unwrap_or(false)
    }

    fn exists(&self) -> bool {
        self.try_exists().unwrap_or(false)
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
