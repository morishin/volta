use image::Image;

use distro;
use notion_fail::{Fallible, ResultExt};

use semver::Version;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NodeVersion {
    pub runtime: String,
    pub npm: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Platform {
    #[serde(default)]
    pub yarn: Option<String>,
    #[serde(default)]
    pub node: Option<NodeVersion>,
}

impl Platform {
    pub fn into_image(self) -> Fallible<Option<Image>> {
        Ok(match self.node {
            Some(NodeVersion { runtime, npm }) => {
                let node = distro::node::NodeVersion {
                    runtime: Version::parse(&runtime).unknown()?,
                    npm: Version::parse(&npm).unknown()?,
                };
                let yarn = if let Some(yarn) = self.yarn {
                    Some(Version::parse(&yarn).unknown()?)
                } else {
                    None
                };

                Some(Image { node, yarn })
            }
            None => None,
        })
    }
}

impl Image {
    pub fn to_serial(&self) -> Platform {
        Platform {
            yarn: self.yarn.as_ref().map(|yarn| yarn.to_string()),
            node: Some(NodeVersion {
                runtime: self.node.runtime.to_string(),
                npm: self.node.npm.to_string(),
            }),
        }
    }
}
