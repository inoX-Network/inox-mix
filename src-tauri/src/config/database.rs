// Modul: config/database — SQLite-Datenbankverbindung und Initialisierung
use log::{error, info};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Datenbank-Manager für SQLite — Thread-sicher über Mutex
pub struct Database {
    /// SQLite-Verbindung (Mutex für Thread-Sicherheit)
    pub(crate) conn: Mutex<Connection>,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database").finish()
    }
}

impl Database {
    /// Datenbank öffnen oder erstellen, Tabellen anlegen
    ///
    /// Erstellt die SQLite-Datei falls sie nicht existiert
    /// und legt die Standard-Tabellen an.
    pub fn open(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Verzeichnis erstellen falls nötig
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path).map_err(|e| {
            error!("SQLite konnte nicht geöffnet werden: {}", e);
            e
        })?;

        // WAL-Modus für bessere Performance bei gleichzeitigen Lese/Schreib-Operationen
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        info!("Datenbank geöffnet: {}", path);

        let db = Self {
            conn: Mutex::new(conn),
        };

        // Standard-Tabellen erstellen
        db.create_tables()?;

        Ok(db)
    }

    /// In-Memory Datenbank erstellen (für Tests)
    pub fn open_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.create_tables()?;
        Ok(db)
    }

    /// Standard-Tabellen erstellen (config, presets, scenes)
    pub fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS config (
                key   TEXT PRIMARY KEY NOT NULL,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS presets (
                id         TEXT PRIMARY KEY NOT NULL,
                name       TEXT NOT NULL,
                category   TEXT NOT NULL DEFAULT 'custom',
                state_json TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TABLE IF NOT EXISTS scenes (
                id         TEXT PRIMARY KEY NOT NULL,
                name       TEXT NOT NULL,
                state_json TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TABLE IF NOT EXISTS sounds (
                id         TEXT PRIMARY KEY NOT NULL,
                name       TEXT NOT NULL,
                file_path  TEXT NOT NULL,
                hotkey     TEXT,
                bus_id     TEXT NOT NULL DEFAULT 'B1',
                volume_db  REAL NOT NULL DEFAULT 0.0,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
            );

            CREATE TABLE IF NOT EXISTS profanity_words (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                word     TEXT NOT NULL UNIQUE,
                category TEXT NOT NULL DEFAULT 'custom',
                language TEXT NOT NULL DEFAULT 'de'
            );

            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY NOT NULL
            );",
        )?;

        // Schema-Version setzen falls noch nicht vorhanden
        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version) VALUES (?1)",
            params![1],
        )?;

        info!("Datenbank-Tabellen erstellt/geprüft");
        Ok(())
    }

    /// Wert aus config-Tabelle lesen
    ///
    /// Gibt None zurück wenn der Key nicht existiert.
    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
        let result = stmt.query_row(params![key], |row| row.get::<_, String>(0));

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Wert in config-Tabelle schreiben (INSERT oder UPDATE)
    pub fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;

        Ok(())
    }

    /// Wert aus config-Tabelle löschen
    pub fn delete(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let rows = conn.execute("DELETE FROM config WHERE key = ?1", params![key])?;

        Ok(rows > 0)
    }

    /// Alle Config-Einträge als Key-Value Paare lesen
    pub fn get_all(&self) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let mut stmt = conn.prepare("SELECT key, value FROM config ORDER BY key")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    /// Generische SQL-Execute-Methode (INSERT, UPDATE, DELETE)
    ///
    /// # Parameter
    /// - `sql`: SQL-Statement
    /// - `params`: Parameter für prepared statement
    ///
    /// # Rückgabe
    /// - Anzahl der betroffenen Zeilen
    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize, Box<dyn std::error::Error>>
    where
        P: rusqlite::Params,
    {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let rows = conn.execute(sql, params)?;
        Ok(rows)
    }

    /// Generische SQL-Query-Methode (SELECT)
    ///
    /// # Parameter
    /// - `sql`: SQL-Query
    /// - `params`: Parameter für prepared statement
    /// - `mapper`: Closure zum Mappen von Rows auf Result-Typ
    ///
    /// # Rückgabe
    /// - Vec mit gemappten Ergebnissen
    pub fn query<T, F, P>(
        &self,
        sql: &str,
        params: P,
        mapper: F,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        F: FnMut(&rusqlite::Row) -> Result<T, rusqlite::Error>,
        P: rusqlite::Params,
    {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, mapper)?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    /// Aktuelle Schema-Version abfragen
    pub fn schema_version(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?;

        let version: u32 =
            conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
                row.get(0)
            })?;

        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory();
        assert!(db.is_ok(), "In-Memory DB sollte sich öffnen lassen");
    }

    #[test]
    fn test_set_and_get() {
        let db = Database::open_in_memory().unwrap();

        // Wert setzen
        let result = db.set("test_key", "test_value");
        assert!(result.is_ok(), "Config setzen sollte funktionieren");

        // Wert lesen
        let value = db.get("test_key").unwrap();
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn test_get_nonexistent() {
        let db = Database::open_in_memory().unwrap();

        let value = db.get("nonexistent").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_overwrite() {
        let db = Database::open_in_memory().unwrap();

        db.set("key", "value1").unwrap();
        db.set("key", "value2").unwrap();

        let value = db.get("key").unwrap();
        assert_eq!(value, Some("value2".to_string()));
    }

    #[test]
    fn test_delete() {
        let db = Database::open_in_memory().unwrap();

        db.set("key", "value").unwrap();
        let deleted = db.delete("key").unwrap();
        assert!(deleted);

        let value = db.get("key").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_delete_nonexistent() {
        let db = Database::open_in_memory().unwrap();

        let deleted = db.delete("nonexistent").unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_get_all() {
        let db = Database::open_in_memory().unwrap();

        db.set("b_key", "b_value").unwrap();
        db.set("a_key", "a_value").unwrap();

        let all = db.get_all().unwrap();
        assert_eq!(all.len(), 2);
        // Sortiert nach Key
        assert_eq!(all[0], ("a_key".to_string(), "a_value".to_string()));
        assert_eq!(all[1], ("b_key".to_string(), "b_value".to_string()));
    }

    #[test]
    fn test_schema_version() {
        let db = Database::open_in_memory().unwrap();

        let version = db.schema_version().unwrap();
        assert_eq!(version, 1);
    }
}
