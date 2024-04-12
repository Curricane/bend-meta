use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use tempfile::TempDir;

pub(crate) struct GlobalSledDb {
    /// When opening a db on a temp dir, the temp dir guard must be held.
    pub(crate) temp_dir: Option<TempDir>,
    pub(crate) path: String,
    pub(crate) db: sled::Db,
}

impl GlobalSledDb {
    pub(crate) fn new_temp(temp_dir: TempDir) -> Self {
        let temp_path = temp_dir.path().to_str().unwrap().to_string();

        GlobalSledDb {
            temp_dir: Some(temp_dir),
            path: temp_path.clone(),
            db: sled::open(temp_path.clone())
                .unwrap_or_else(|e| panic!("open global sled::Db(path: {}): {}", temp_path, e)),
        }
    }

    pub(crate) fn new(path: String) -> Self {
        GlobalSledDb {
            temp_dir: None,
            path: path.clone(),
            db: sled::open(path.clone())
                .unwrap_or_else(|e| panic!("open global sled::Db(path: {}): {}", path, e)),
        }
    }
}

static GLOBAL_SLED: LazyLock<Arc<Mutex<Option<GlobalSledDb>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

/// Open a db at a temp dir. For test purpose only.
pub fn init_temp_sled_db(temp_dir: TempDir) {
    let temp_path = temp_dir.path().to_str().unwrap().to_string();

    let (inited_as_temp, curr_path) = {
        let mut g = GLOBAL_SLED.as_ref().lock().unwrap();
        if let Some(gdb) = g.as_ref() {
            (gdb.temp_dir.is_some(), gdb.path.clone())
        } else {
            *g = Some(GlobalSledDb::new_temp(temp_dir));
            return;
        }
    };

    if !inited_as_temp {
        panic!(
            "sled db is already initialized with specified path: {}, can not re-init with temp path {}",
            curr_path, temp_path
        );
    }
}

pub fn init_sled_db(path: String) {
    let (inited_as_temp, curr_path) = {
        let mut g = GLOBAL_SLED.as_ref().lock().unwrap();
        if let Some(gdb) = g.as_ref() {
            (gdb.temp_dir.is_some(), gdb.path.clone())
        } else {
            *g = Some(GlobalSledDb::new(path));
            return;
        }
    };

    if inited_as_temp {
        panic!(
            "sled db is already initialized with temp dir: {}, can not re-init with path {}",
            curr_path, path
        );
    }
}

pub fn get_sled_db() -> sled::Db {
    {
        let guard = GLOBAL_SLED.as_ref().lock().unwrap();
        let glb_opt = guard.as_ref();
        match glb_opt {
            None => {}
            Some(g) => return g.db.clone(),
        }
    }

    panic!("init_sled_db() or init_temp_sled_db() has to be called before using get_sled_db()");
}
