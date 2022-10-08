use anyhow::{bail, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Parameters {
    pub brightness: f64,
    pub contrast: f64,
}
impl TryFrom<HashMap<String, String>> for Parameters {
    type Error = anyhow::Error;
    fn try_from(item: HashMap<String, String>) -> Result<Self> {
        let brightness: f64;
        let brightness_formatted;
        let contrast;
        if let Some(b) = item.get("brightness") {
            brightness = b.parse()?;
            if brightness >= 0.0 {
                brightness_formatted = 255.0 * (2.0 * brightness - 1.0);
            } else {
                bail!("Wrong brightness parameter");
            }
        } else {
            bail!("No brightness parameter");
        }
        if let Some(c) = item.get("contrast") {
            contrast = c.parse()?;
            if !(0.0..=1.0).contains(&contrast) {
                bail!("Wrong contrast parameter");
            }
        } else {
            bail!("No contrast parameter");
        }
        Ok(Parameters {
            brightness: brightness_formatted,
            contrast,
        })
    }
}
