use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::thread;

pub const CURRENT_VERSION: &str = "5.8.0";
pub const GITHUB_REPO: &str = "gabrielpgava/open-lake-controller-5.8";

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate(String),
    UpdateAvailable {
        version: String,
        download_url: String,
        release_notes: String,
    },
    Downloading(f32),
    Downloaded(String),
    Error(String),
}

#[derive(Debug, Clone, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Clone)]
pub struct AutoUpdater {
    status: Arc<Mutex<UpdateStatus>>,
}

impl Default for AutoUpdater {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoUpdater {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(UpdateStatus::Idle)),
        }
    }

    pub fn get_status(&self) -> UpdateStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn check_for_updates(&self) {
        let status_arc = Arc::clone(&self.status);
        {
            let mut s = status_arc.lock().unwrap();
            *s = UpdateStatus::Checking;
        }

        thread::spawn(move || {
            let url = format!("https://api.github.com/repos/{}/releases", GITHUB_REPO);
            
            // Execute curl or powershell to fetch releases securely
            let output = std::process::Command::new("curl.exe")
                .args(["-s", "-H", "User-Agent: open-lake-controller", &url])
                .output();

            match output {
                Ok(out) if out.status.success() => {
                    let json_str = String::from_utf8_lossy(&out.stdout);
                    match serde_json::from_str::<Vec<GithubRelease>>(&json_str) {
                        Ok(releases) if !releases.is_empty() => {
                            let latest = &releases[0];
                            let tag = latest.tag_name.trim_start_matches('v');
                            
                            if is_newer_version(tag, CURRENT_VERSION) {
                                let download_url = latest.assets.iter()
                                    .find(|a| a.name.ends_with(".zip") || a.name.ends_with(".exe") || a.name.ends_with(".tar.gz"))
                                    .map(|a| a.browser_download_url.clone())
                                    .unwrap_or_else(|| latest.html_url.clone());

                                let mut s = status_arc.lock().unwrap();
                                *s = UpdateStatus::UpdateAvailable {
                                    version: tag.to_string(),
                                    download_url,
                                    release_notes: latest.body.clone().unwrap_or_default(),
                                };
                            } else {
                                let mut s = status_arc.lock().unwrap();
                                *s = UpdateStatus::UpToDate(CURRENT_VERSION.to_string());
                            }
                        }
                        Ok(_) => {
                            // No releases published yet on github
                            let mut s = status_arc.lock().unwrap();
                            *s = UpdateStatus::UpToDate(format!("{} (Latest / No newer releases published)", CURRENT_VERSION));
                        }
                        Err(e) => {
                            let mut s = status_arc.lock().unwrap();
                            *s = UpdateStatus::Error(format!("Failed to parse release metadata: {}", e));
                        }
                    }
                }
                Ok(out) => {
                    let mut s = status_arc.lock().unwrap();
                    *s = UpdateStatus::Error(format!("GitHub API responded with code: {:?}", out.status.code()));
                }
                Err(e) => {
                    let mut s = status_arc.lock().unwrap();
                    *s = UpdateStatus::Error(format!("Network check error: {}", e));
                }
            }
        });
    }

    pub fn download_and_install(&self, url: String) {
        let status_arc = Arc::clone(&self.status);
        {
            let mut s = status_arc.lock().unwrap();
            *s = UpdateStatus::Downloading(0.2);
        }

        thread::spawn(move || {
            // Open the download URL in the system default browser or download asset
            if let Err(e) = open_browser(&url) {
                let mut s = status_arc.lock().unwrap();
                *s = UpdateStatus::Error(format!("Failed to open download url: {}", e));
            } else {
                let mut s = status_arc.lock().unwrap();
                *s = UpdateStatus::Downloaded(format!("Opened browser to download: {}", url));
            }
        });
    }
}

fn open_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "", url])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    }
    Ok(())
}

fn is_newer_version(remote: &str, current: &str) -> bool {
    let r_parts: Vec<u32> = remote.split('.').filter_map(|s| s.parse().ok()).collect();
    let c_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();

    for (r, c) in r_parts.iter().zip(c_parts.iter()) {
        if r > c {
            return true;
        } else if r < c {
            return false;
        }
    }
    r_parts.len() > c_parts.len()
}
