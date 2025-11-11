use std::{fs::File, io::Write};

use std::io::BufWriter;

use crate::{
    errors::CoreError,
    search::{Arch, Language, Platform},
};

const BASE_URL: &str = "https://crackmes.one/";

#[derive(Debug)]
pub struct Challenge {
    pub language: Language,
    pub author: String,
    pub name: String,
    pub url: String,
    pub arch: Arch,
    pub difficulty: f32,
    pub quality: f32,
    pub platform: Platform,
}

impl Challenge {
    fn get_download_url(&self) -> String {
        format!("{}/static{}.zip", BASE_URL, self.url)
    }

    pub async fn download(&self) -> Result<(), CoreError> {
        let resp = reqwest::get(self.get_download_url())
            .await
            .map_err(|_| CoreError::DownloadFailure)?;
        let body = resp.bytes().await.map_err(|_| CoreError::DownloadFailure)?;
        let out =
            File::create(format!("{}.zip", self.name)).map_err(|_| CoreError::DownloadFailure)?;
        let _ = BufWriter::new(out).write_all(&body);
        Ok(())
    }
}
