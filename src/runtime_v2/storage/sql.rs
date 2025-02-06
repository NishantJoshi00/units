use super::{DriverInfo, DriverStorage, PathInfo, Program, ProgramStorage, Resolver};
use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::SqlitePool;
use tonic::async_trait;
use wasmtime::component::Component;

#[derive(Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    // Helper method for JSON serialization
    fn serialize<T: Serialize>(value: &T) -> Option<String> {
        serde_json::to_string(value).ok()
    }

    // Helper method for JSON deserialization
    fn deserialize<T: DeserializeOwned>(json: &str) -> Option<T> {
        serde_json::from_str(json).ok()
    }

    // Helper method for component serialization
    async fn serialize_component(component: &Component) -> Result<Vec<u8>> {
        component
            .serialize()
            .context("Failed to serialize component")
    }

    // Helper method for component deserialization
    fn deserialize_component(bytes: &[u8], engine: &wasmtime::Engine) -> Result<Component> {
        unsafe { Component::deserialize(engine, bytes).context("Failed to deserialize component") }
    }
}

#[async_trait]
impl Resolver for SqliteStorage {
    async fn remove(&self, path: &str) -> Option<PathInfo> {
        let result = sqlx::query!(
            "DELETE FROM Resolver WHERE path = ? RETURNING path_info",
            path
        )
        .fetch_optional(&self.pool)
        .await
        .ok()?;

        result.and_then(|row| Self::deserialize(&row.path_info))
    }

    async fn list(&self) -> Vec<(String, PathInfo)> {
        sqlx::query!("SELECT path, path_info FROM Resolver")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter_map(|row| Self::deserialize(&row.path_info).map(|info| (row.path, info)))
            .collect()
    }

    async fn get(&self, path: &str) -> Option<PathInfo> {
        let result = sqlx::query!("SELECT path_info FROM Resolver WHERE path = ?", path)
            .fetch_optional(&self.pool)
            .await
            .ok()?;

        result.and_then(|row| Self::deserialize(&row.path_info))
    }

    async fn insert(&self, path: String, path_info: PathInfo) -> Option<()> {
        let path_info_json = Self::serialize(&path_info)?;

        sqlx::query!(
            "INSERT OR REPLACE INTO Resolver (path, path_info) VALUES (?, ?)",
            path,
            path_info_json
        )
        .execute(&self.pool)
        .await
        .ok()
        .map(|_| ())
    }
}

#[async_trait]
impl ProgramStorage for SqliteStorage {
    async fn insert(&self, id: &str, program: Program) -> Result<()> {
        let component_bytes = Self::serialize_component(&program.component).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO Program (id, name, version, component) VALUES (?, ?, ?, ?)",
            id,
            program.name,
            program.version,
            component_bytes
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert program")?;

        Ok(())
    }

    async fn get(&self, id: &str, engine: wasmtime::Engine) -> Result<Option<Program>> {
        let result = sqlx::query!(
            "SELECT name, version, component FROM Program WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        result
            .map(|row| {
                Ok(Program {
                    component: Self::deserialize_component(&row.component, &engine)?,
                    name: row.name,
                    version: row.version,
                })
            })
            .transpose()
    }

    async fn list(&self, engine: wasmtime::Engine) -> Result<Vec<(String, Program)>> {
        let rows = sqlx::query!("SELECT id, name, version, component FROM Program")
            .fetch_all(&self.pool)
            .await?;

        let mut programs = Vec::with_capacity(rows.len());
        for row in rows {
            let program = Program {
                component: Self::deserialize_component(&row.component, &engine)?,
                name: row.name,
                version: row.version,
            };
            programs.push((row.id, program));
        }

        Ok(programs)
    }
}

#[async_trait]
impl DriverStorage for SqliteStorage {
    async fn insert(&self, driver_info: DriverInfo, module: Component) -> Result<()> {
        let component_bytes = Self::serialize_component(&module).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO Driver (name, version, component) VALUES (?, ?, ?)",
            driver_info.name,
            driver_info.version,
            component_bytes
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert driver")?;

        Ok(())
    }

    async fn get(
        &self,
        driver_info: &DriverInfo,
        engine: wasmtime::Engine,
    ) -> Result<Option<Component>> {
        let result = sqlx::query!(
            "SELECT component FROM Driver WHERE name = ? AND version = ?",
            driver_info.name,
            driver_info.version
        )
        .fetch_optional(&self.pool)
        .await?;

        result
            .map(|row| Self::deserialize_component(&row.component, &engine))
            .transpose()
    }

    async fn list(&self, engine: wasmtime::Engine) -> Result<Vec<(DriverInfo, Component)>> {
        let rows = sqlx::query!("SELECT name, version, component FROM Driver")
            .fetch_all(&self.pool)
            .await?;

        let mut drivers = Vec::with_capacity(rows.len());
        for row in rows {
            let driver_info = DriverInfo {
                name: row.name,
                version: row.version,
            };
            let component = Self::deserialize_component(&row.component, &engine)?;
            drivers.push((driver_info, component));
        }

        Ok(drivers)
    }

    async fn remove(&self, driver_info: &DriverInfo) -> Result<()> {
        sqlx::query!(
            "DELETE FROM Driver WHERE name = ? AND version = ?",
            driver_info.name,
            driver_info.version
        )
        .execute(&self.pool)
        .await
        .context("Failed to remove driver")?;

        Ok(())
    }
}
