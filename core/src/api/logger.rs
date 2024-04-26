//! OneLauncher log management

use std::io::Read;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use serde::Serialize;
use futures::TryFutureExt;
use crate::data::{Credentials, Directories, MinecraftCredentials};
use crate::prelude::ClusterPath;
use crate::utils::io::{self, IOError};

// TODO: put this in the global store
/// Core logging state and reader for OneLauncher.
#[derive(Serialize, Debug)]
pub struct LogManager {
    /// The log file to read as a [`String`]
    pub log_file: String,
    /// The parsed and censored output of the logfile.
    pub output: Option<LogOutput>
}

impl LogManager {
    /// Initialize a new [`LogManager`].
    async fn initialize(cluster: &ClusterPath, log_file: String, clear: Option<bool>) -> crate::Result<Self> {
        Ok(Self {
            output: if clear.unwrap_or(false) {
                None
            } else {
                Some(get_output_by_file(cluster, &log_file).await?) 
            },
            log_file,
        })
    }
}

/// Get a [`Vec<LogManager>`] for a specific [`ClusterPath`].
#[tracing::instrument]
pub async fn get_logs(cluster_path: ClusterPath, clear: Option<bool>) -> crate::Result<Vec<LogManager>> {
    let cluster_path = if let Some(c) = crate::cluster::get(&cluster_path, None).await? {
        c.cluster_path()
    } else {
        return Err(anyhow::anyhow!("cluster {} isn't managed by onelauncher!", cluster_path.to_string()).into());
    };

    let logs_folder = Directories::cluster_logs_dir(&cluster_path).await?;
    let mut logs = Vec::new();
    if logs_folder.exists() {
        for entry in std::fs::read_dir(&logs_folder).map_err(|e| IOError::with_path(e, &logs_folder))? {
            let entry: std::fs::DirEntry = entry.map_err(|e| IOError::with_path(e, &logs_folder))?;
            let path = entry.path();
            if !path.is_file() { continue; }
            if let Some(name) = path.file_name() {
                let name = name.to_string_lossy().to_string();
                logs.push(LogManager::initialize(&cluster_path, name, clear).await);
            }
        }
    }

    let mut logs = logs.into_iter().collect::<crate::Result<Vec<LogManager>>>()?;
    logs.sort_by_key(|x| x.log_file.clone());
    Ok(logs)
}

/// Delete all stored logs from a specific [`ClusterPath`].
#[tracing::instrument]
pub async fn delete_logs(cluster_path: ClusterPath) -> crate::Result<()> {
    let cluster_path = if let Some(c) = crate::cluster::get(&cluster_path, None).await? {
        c.cluster_path()
    } else {
        return Err(anyhow::anyhow!("cluster {} isn't managed by onelauncher!", cluster_path.to_string()).into());
    };

    let logs_folder = Directories::cluster_logs_dir(&cluster_path).await?;
    for entry in std::fs::read_dir(&logs_folder).map_err(|e| IOError::with_path(e, &logs_folder))? {
        let entry = entry.map_err(|e| IOError::with_path(e, &logs_folder))?;
        let path = entry.path();
        if path.is_dir() { io::remove_dir_all(&path).await? }
    }

    Ok(())
}

/// Get the [`LogManager`] for a specific [`ClusterPath`] log file.
#[tracing::instrument]
pub async fn get_logs_by_file(cluster_path: ClusterPath, log_file: String) -> crate::Result<LogManager> {
    let cluster_path = if let Some(c) = crate::cluster::get(&cluster_path, None).await? {
        c.cluster_path()
    } else {
        return Err(anyhow::anyhow!("cluster {} isn't managed by onelauncher!", cluster_path.to_string()).into());
    };

    Ok(LogManager {
        output: Some(get_output_by_file(&cluster_path, &log_file).await?),
        log_file,
    })
}

/// Get the default [`LogCursor`] for a [`ClusterPath`].
#[tracing::instrument]
pub async fn get_log_cursor(cluster_path: ClusterPath, cursor: u64) -> crate::Result<LogCursor> {
    get_live_log_cursor(cluster_path, "latest.log", cursor).await
}

/// Get a live [`LogCursor`] for a [`ClusterPath`]'s log file.
#[tracing::instrument]
pub async fn get_live_log_cursor(cluster_path: ClusterPath, log_file: &str, mut cursor: u64) -> crate::Result<LogCursor> {
    let cluster_path = if let Some(c) = crate::cluster::get(&cluster_path, None).await? {
        c.cluster_path()
    } else {
        return Err(anyhow::anyhow!("cluster {} isn't managed by onelauncher!", cluster_path.to_string()).into());
    };

    let state = crate::State::get().await?;
    let logs_folder = Directories::cluster_logs_dir(&cluster_path).await?;
    let path = logs_folder.join(log_file);
    if !path.exists() { return Ok(LogCursor { cursor: 0, new: false, output: LogOutput("".to_string()) }); }

    let mut file = tokio::fs::File::open(&path).await.map_err(|e| IOError::with_path(e, &path))?;
    let metadata = file.metadata().await.map_err(|e| IOError::with_path(e, &path))?;
    let mut new = false;

    if cursor > metadata.len() {
        cursor = 0;
        new = true;
    }

    let mut buf = Vec::new();
    file.seek(std::io::SeekFrom::Start(cursor)).map_err(|e| IOError::with_path(e, &path)).await?;
    let bytes_read = file.read_to_end(&mut buf).map_err(|e| IOError::with_path(e, &path)).await?;
    let output = String::from_utf8_lossy(&buf).to_string();
    let cursor = cursor + bytes_read as u64;
    let creds: Vec<MinecraftCredentials> = state.users.read().await.users.clone().into_values().collect();
    let output = LogOutput::censor_secrets(output, &creds, None);
    Ok(LogCursor { cursor, new, output })
}

/// Delete a specific minecraft log file.
#[tracing::instrument]
pub async fn delete_logs_by_file(cluster_path: ClusterPath, log_file: &str) -> crate::Result<()> {
    let cluster_path = if let Some(c) = crate::cluster::get(&cluster_path, None).await? {
        c.cluster_path()
    } else {
        return Err(anyhow::anyhow!("cluster {} isn't managed by onelauncher!", cluster_path.to_string()).into());
    };

    let logs_folder = Directories::cluster_logs_dir(&cluster_path).await?;
    let path = logs_folder.join(log_file);
    io::remove_dir_all(&path).await?;
    Ok(())
}

/// Get a specific [`ClusterPath`] log file's [`LogOutput`].
#[tracing::instrument]
pub async fn get_output_by_file(cluster_path: &ClusterPath, log_file: &str) -> crate::Result<LogOutput> {
    let state = crate::State::get().await?;
    let logs_folder = Directories::cluster_logs_dir(cluster_path).await?;
    let path = logs_folder.join(log_file);
    let credentials: Vec<MinecraftCredentials> = state.users.read().await.users.clone().into_values().collect();

    // todo: make this a utility function
    if let Some(ext) = path.extension() {
        let mut file = std::fs::File::open(&path).map_err(|e| IOError::with_path(e, &path))?;
        let mut contents = [0; 1024];
        let mut result = String::new();

        if ext == "gz" {
            let mut gz = flate2::read::GzDecoder::new(std::io::BufReader::new(file));

            while gz.read(&mut contents).map_err(|e| IOError::with_path(e, &path))? > 0 {
                result.push_str(&String::from_utf8_lossy(&contents));
                contents = [0; 1024];
            }

            return Ok(LogOutput::censor_secrets(result, &credentials, None));
        } else if ext == "log" {
            while file.read(&mut contents).map_err(|e| IOError::with_path(e, &path))? > 0 {
                result.push_str(&String::from_utf8_lossy(&contents));
                contents = [0; 1024];
            }

            return Ok(LogOutput::censor_secrets(result, &credentials, None));
        }
    }

    Err(anyhow::anyhow!("log file extension {} not supported", path.display()).into())
}

/// The log cursor used to parse logs passed into [`LogManager`].
#[derive(Serialize, Debug)]
pub struct LogCursor {
    /// The cursor ID.
    pub cursor: u64,
    /// The [`LogOutput`] associated with this cursor.
    pub output: LogOutput,
    /// Whether or not this corresponds is a new log file.
    pub new: bool,
}

/// The log output, a wrapper around [`String`], with utilities for parsing and censoring log contents.
#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct LogOutput(String);

impl LogOutput {
    /// Censor user secrets because sometimes mclogs misses them, including username, realname and minecraft credentials.
    pub fn censor_secrets(mut output: String, credentials: &Vec<MinecraftCredentials>, credentials_store: Option<Credentials>) -> Self {
        let username = whoami::username();
        let realname = whoami::realname();
        output = output.replace(&format!("/{}/", username), "/{ENV_USERNAME}/");
        output = output.replace(&format!("\\{}\\", username), "\\{ENV_USERNAME}\\");
        output = output.replace(&format!("/{}/", realname), "/{ENV_REALNAME}/");
        output = output.replace(&format!("\\{}\\", realname), "\\{ENV_REALNAME}\\");

        for cred in credentials {
            output = output.replace(&cred.access_token, "{MC_ACCESS_TOKEN}");
            output = output.replace(&cred.username, "{MC_USERNAME}");
            output = output.replace(&cred.id.as_simple().to_string(), "{MC_UUID}");
            output = output.replace(&cred.id.as_hyphenated().to_string(), "{MC_UUID}");
        }

        Self(output)
    }
}
