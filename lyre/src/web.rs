use std::{fs, path::PathBuf};

use eyre::{eyre, Ok, Result, WrapErr};
use melody::{utils::in_dir_with_ext, Melody};

pub struct Parcel;
impl Melody for Parcel {
    fn name() -> &'static str {
        "Parcel"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        in_dir_with_ext("./web/", ".js")
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["generated/static/main.js"])
    }

    fn perform() -> Result<()> {
        let output = std::process::Command::new("npm.cmd")
            .args([
                "exec",
                "--",
                "parcel",
                "build",
                "--dist-dir",
                "./generated/static",
                "./web/main.js",
            ])
            .output()?;

        if !output.status.success() {
            return Err(eyre!("{}", String::from_utf8(output.stderr)?));
        }

        Ok(())
    }
}

pub struct Favicon;
impl Melody for Favicon {
    fn name() -> &'static str {
        "Favicon"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["content/favicon.ico"])
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["generated/static/favicon.ico"])
    }

    fn perform() -> Result<()> {
        std::fs::copy("content/favicon.ico", "generated/static/favicon.ico")
            .wrap_err("content/favicon.ico is missing")?;

        Ok(())
    }
}

pub struct SCSS;
impl Melody for SCSS {
    fn name() -> &'static str {
        "SCSS"
    }

    fn source() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        in_dir_with_ext("./web/", ".scss")
    }

    fn rendition() -> Result<impl IntoIterator<Item = impl Into<PathBuf>>> {
        Ok(["generated/static/styles.css"])
    }

    fn perform() -> Result<()> {
        let css = grass::from_path(
            "web/styles.scss",
            &grass::Options::default()
                .style(grass::OutputStyle::Compressed)
                .load_path("node_modules/@picocss/pico/scss/")
                .load_path("node_modules/@catppuccin/palette/scss"),
        )?;

        fs::write("generated/static/styles.css", css)?;

        Ok(())
    }
}
