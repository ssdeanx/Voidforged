//! Save database — SQLite-backed persistence for character profiles.
//! Uses a thin relational table around a Bincode-serialized BLOB column.

use bevy::prelude::*;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use crate::components::CharacterClass;
use crate::resources::PlayerProfile;

/// Wrapper around the SQLite connection. Uses Mutex for Send+Sync.
#[derive(Resource)]
pub struct SaveDatabase {
    conn: std::sync::Mutex<Connection>,
}

impl SaveDatabase {
    pub fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::db_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        let mut db = Self { conn: std::sync::Mutex::new(conn) };
        db.migrate()?;
        Ok(db)
    }

    fn db_path() -> PathBuf {
        let base = dirs_data_dir();
        base.join("voidforged").join("saves.db")
    }

    fn migrate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.get_mut().unwrap();
        let current: u32 = conn.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if current < 1 {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS profiles (
                    id TEXT PRIMARY KEY, name TEXT NOT NULL, class TEXT NOT NULL DEFAULT '',
                    schema_version INTEGER NOT NULL DEFAULT 1, level INTEGER NOT NULL DEFAULT 1,
                    data BLOB NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
                    play_time_secs INTEGER NOT NULL DEFAULT 0
                );
                CREATE INDEX IF NOT EXISTS idx_profiles_updated ON profiles(updated_at DESC);"
            )?;
            conn.pragma_update(None, "user_version", 1)?;
        }
        Ok(())
    }

    pub fn save_profile(&self, profile: &PlayerProfile) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(profile)?;
        let now = iso_now();
        let class_str = format!("{:?}", profile.class);
        let id_str = format!("char_{}", profile.id);
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO profiles (id, name, class, schema_version, level, data, created_at, updated_at, play_time_secs)
             VALUES (?1, ?2, ?3, 1, ?4, ?5, COALESCE((SELECT created_at FROM profiles WHERE id=?1), ?6), ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, class=excluded.class,
                level=excluded.level, data=excluded.data, updated_at=excluded.updated_at,
                play_time_secs=excluded.play_time_secs",
            params![id_str, profile.name, class_str, profile.level, data, now, profile.play_time as i64],
        )?;
        Ok(())
    }

    pub fn load_profile(&self, id: u32) -> Result<Option<PlayerProfile>, Box<dyn std::error::Error>> {
        let id_str = format!("char_{}", id);
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT data FROM profiles WHERE id = ?1")?;
        let mut rows = stmt.query(params![id_str])?;
        match rows.next()? {
            Some(row) => {
                let data: Vec<u8> = row.get(0)?;
                let profile: PlayerProfile = bincode::deserialize(&data)?;
                Ok(Some(profile))
            }
            None => Ok(None),
        }
    }

    pub fn list_profiles(&self) -> Result<Vec<ProfileSummary>, Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, class, level, play_time_secs, updated_at FROM profiles ORDER BY updated_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let id = id_str.strip_prefix("char_").and_then(|s| s.parse().ok()).unwrap_or(0);
            Ok(ProfileSummary {
                id,
                name: row.get(1)?,
                class: row.get::<_, String>(2)?.parse().unwrap_or_default(),
                level: row.get(3)?,
                play_time_secs: row.get(4)?,
                last_played: row.get(5)?,
            })
        })?;
        let mut summaries = Vec::new();
        for row in rows { summaries.push(row?); }
        Ok(summaries)
    }

    pub fn delete_profile(&self, id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let id_str = format!("char_{}", id);
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM profiles WHERE id = ?1", params![id_str])?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProfileSummary {
    pub id: u32,
    pub name: String,
    pub class: CharacterClass,
    pub level: u32,
    pub play_time_secs: i64,
    pub last_played: String,
}

fn dirs_data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(dir)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local").join("share")
    } else {
        PathBuf::from(".")
    }
}

fn iso_now() -> String {
    let dur = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs();
    let (y, m, d, h, min, s) = seconds_to_datetime(secs);
    format!("{y:04}-{m:02}-{d:02}T{h:02}:{min:02}:{s:02}Z")
}

fn seconds_to_datetime(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let days = secs / 86400;
    let r = secs % 86400;
    let (h, min, s) = (r / 3600, (r % 3600) / 60, r % 60);
    let mut y = 1970i64;
    let mut d = days as i64;
    loop {
        let diy = if is_leap(y) { 366 } else { 365 };
        if d < diy { break; }
        d -= diy; y += 1;
    }
    let md = if is_leap(y) { [31,29,31,30,31,30,31,31,30,31,30,31] } else { [31,28,31,30,31,30,31,31,30,31,30,31] };
    let mut m = 0u64;
    for (i, &days) in md.iter().enumerate() {
        if d < days { m = (i + 1) as u64; break; }
        d -= days;
    }
    if m == 0 { m = 12; }
    (y as u64, m, (d + 1) as u64, h, min, s)
}

fn is_leap(y: i64) -> bool { (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 }

/// Startup — initializes the save database.
pub fn init_save_db(mut commands: Commands) {
    match SaveDatabase::open() {
        Ok(db) => { info!("Save DB opened"); commands.insert_resource(db); }
        Err(e) => { error!("Failed to open save DB: {e}"); }
    }
}

/// Auto-save every 30s.
pub fn auto_save(
    time: Res<Time>,
    mut timer: Local<f32>,
    db: Option<Res<SaveDatabase>>,
    profile_query: Query<&PlayerProfile, With<crate::components::Player>>,
) {
    let Some(db) = db else { return };
    *timer += time.delta_secs();
    if *timer < 30.0 { return; }
    *timer = 0.0;
    let Ok(profile) = profile_query.get_single() else { return };
    if let Err(e) = db.save_profile(profile) {
        error!("Auto-save failed: {e}");
    }
}

/// Save on quit.
pub fn save_on_quit(
    db: Option<Res<SaveDatabase>>,
    profile_query: Query<&PlayerProfile, With<crate::components::Player>>,
) {
    let Some(db) = db else { return };
    let Ok(profile) = profile_query.get_single() else { return };
    if let Err(e) = db.save_profile(profile) {
        error!("Save-on-quit failed: {e}");
    }
}
