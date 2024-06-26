use crate::text::into_byte_format;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct Recorder {
    base_filename: String,
    suffix: usize,
    file_handler: Option<File>,
    file_size: u128,
}

impl Recorder {
    pub fn new(filename: String) -> Result<Self, String> {
        let filename = PathBuf::from(filename)
            .with_extension("")
            .file_name()
            .ok_or("Cannot get filename to record")?
            .to_str()
            .ok_or("Cannot convert record filename to string")?
            .to_string();

        Ok(Self {
            base_filename: filename,
            suffix: 1,
            file_handler: None,
            file_size: 0,
        })
    }

    pub fn get_filename(&self) -> String {
        format!("{}_rec{}.txt", self.base_filename, self.suffix)
    }

    pub fn get_size(&self) -> String {
        into_byte_format(self.file_size)
    }

    pub fn is_recording(&self) -> bool {
        self.file_handler.is_some()
    }

    pub async fn start_record(&mut self) -> Result<(), String> {
        let file = File::create(self.get_filename())
            .await
            .map_err(|err| err.to_string())?;
        self.file_handler = Some(file);

        Ok(())
    }

    pub fn stop_record(&mut self) {
        if let Some(file) = self.file_handler.take() {
            drop(file);
        }

        self.suffix += 1;
        self.file_size = 0;
    }

    pub async fn add_content(&mut self, content: String) -> Result<(), String> {
        let Some(file) = self.file_handler.as_mut() else {
            return Err("No file recording now".to_string());
        };

        let content = if !content.ends_with('\n') {
            content + "\r\n"
        } else {
            content
        };

        file.write_all(content.as_bytes())
            .await
            .map_err(|err| err.to_string())?;
        self.file_size += content.len() as u128;

        Ok(())
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        if let Some(file) = self.file_handler.take() {
            drop(file);
        }
    }
}
