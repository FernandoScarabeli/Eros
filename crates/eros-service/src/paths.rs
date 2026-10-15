use std::ffi::{CStr, OsStr};
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::{env, fs, io, path::PathBuf};

pub fn home_dir() -> io::Result<PathBuf> {
    if let Some(home) = env::var_os("HOME") {
        if let Ok(home) = fs::canonicalize(home) {
            if home.is_dir() {
                return Ok(home);
            }
        }
    }

    let mut capacity = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
    if capacity < 1024 {
        capacity = 16 * 1024;
    }
    let mut buffer = vec![0_u8; capacity as usize];
    loop {
        let mut account = MaybeUninit::<libc::passwd>::uninit();
        let mut result = std::ptr::null_mut();
        let status = unsafe {
            libc::getpwuid_r(
                libc::geteuid(),
                account.as_mut_ptr(),
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                &mut result,
            )
        };
        if status == libc::ERANGE {
            if buffer.len() >= 1024 * 1024 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "The user account record is too large",
                ));
            }
            buffer.resize(buffer.len() * 2, 0);
            continue;
        }
        if status != 0 {
            return Err(io::Error::from_raw_os_error(status));
        }
        if result.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The current user account was not found",
            ));
        }
        let account = unsafe { account.assume_init() };
        if account.pw_dir.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The current user has no home directory",
            ));
        }
        let home = unsafe { CStr::from_ptr(account.pw_dir) };
        let home = PathBuf::from(OsStr::from_bytes(home.to_bytes()));
        let home = fs::canonicalize(home)?;
        if !home.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The current user's home path is not a directory",
            ));
        }
        return Ok(home);
    }
}

pub fn private_dir(path: PathBuf) -> io::Result<PathBuf> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
    }
    let meta = fs::symlink_metadata(&path)?;
    if !meta.is_dir()
        || meta.file_type().is_symlink()
        || meta.uid() != unsafe { libc::geteuid() }
        || meta.mode() & 0o777 != 0o700
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Eros requires a private user-owned directory (0700)",
        ));
    }
    Ok(path)
}

pub fn data_dir() -> io::Result<PathBuf> {
    let path = env::var_os("EROS_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env::var_os("HOME").unwrap_or_default()).join(".local/share")
                })
                .join("eros")
        });
    private_dir(path)
}

pub fn runtime_dir() -> io::Result<PathBuf> {
    let path = env::var_os("EROS_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            env::var_os("XDG_RUNTIME_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(format!("/tmp/eros-{}", unsafe { libc::geteuid() }))
                })
                .join("eros")
        });
    private_dir(path)
}

pub fn socket_path() -> io::Result<PathBuf> {
    Ok(runtime_dir()?.join("service.sock"))
}
