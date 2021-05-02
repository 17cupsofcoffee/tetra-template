use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;

use eyre::{Result, WrapErr};
use tetra::graphics::{ImageData, Texture};
use tetra::Context;

pub struct Assets {
    textures: HashMap<String, Texture>,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> Result<Assets> {
        let mut textures = HashMap::new();

        let texture_dir =
            fs::read_dir("./resources/textures").wrap_err("Failed to open texture directory")?;

        for entry in texture_dir {
            let entry = entry.wrap_err("Failed to open entry in texture directory")?;
            let path = entry.path();

            let mut file_stem = match path.file_stem().and_then(OsStr::to_str) {
                Some(f) => f,
                None => continue,
            };

            let mut data = ImageData::from_file(&path)?;

            if file_stem.ends_with(".pm") {
                file_stem = file_stem.trim_end_matches(".pm");
            } else {
                data.premultiply();
            }

            textures.insert(file_stem.to_owned(), data.to_texture(ctx)?);
        }

        textures.insert(
            "solid".into(),
            Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?,
        );

        Ok(Assets { textures })
    }

    pub fn get_texture(&self, key: &str) -> &Texture {
        // TODO: Maybe make this return an Option or a default texture?
        self.textures.get(key).unwrap()
    }
}
