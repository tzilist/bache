use std::str::FromStr;

use uuid::Uuid;

use super::InstanceName;
use crate::errors::Error;

pub struct ResourceName {
    pub instance_name: InstanceName,
    pub uuid: Option<Uuid>,
    pub hash: String,
    pub size: usize,
}

impl TryFrom<&str> for ResourceName {
    type Error = Error;

    // Bazel will send resource names in the patterns:
    // * `{instance_name}/uploads/{uuid}/blobs/{hash}/{size}`
    // * `{instance_name}/blobs/{hash}/{size}`
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.splitn(6, '/');

        let instance_name = parts
            .next()
            .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?
            .into();

        let blobs_or_uploads = parts
            .next()
            .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?;

        let mut uuid: Option<Uuid> = None;

        if blobs_or_uploads == "uploads" {
            let raw_uuid = parts
                .next()
                .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?;

            uuid = Some(
                Uuid::from_str(raw_uuid)
                    .map_err(|_| Error::InvalidResourceName(value.to_string()))?,
            );

            // this next section should be `blobs`
            let blob_part = parts
                .next()
                .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?;
            if blob_part != "blobs" {
                return Err(Error::InvalidResourceName(value.to_string()));
            }
        } else if blobs_or_uploads != "blobs" {
            return Err(Error::InvalidResourceName(value.to_string()));
        }

        let hash = parts
            .next()
            .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?
            .to_string();

        let raw_size = parts
            .next()
            .ok_or_else(|| Error::InvalidResourceName(value.to_string()))?;

        let size: usize = raw_size
            .parse()
            .map_err(|_| Error::InvalidResourceName(value.to_string()))?;

        Ok(Self {
            instance_name,
            uuid,
            hash,
            size,
        })
    }
}

impl TryFrom<String> for ResourceName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let resource_name = value.as_str();
        resource_name.try_into()
    }
}
