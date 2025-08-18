use log::debug;
use serde::Deserialize;
use serde_valid::json::FromJsonValue;
use serde_valid::Validate;
use std::fs;
use std::io;
use vkmsctl::{
    ConnectorConfig, CrtcConfig, EncoderConfig, PlaneConfig, PlaneKind, VkmsDeviceBuilder,
};

#[derive(Debug, Deserialize, Validate)]
struct ConfigValidator {
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    name: String,
    enabled: bool,
    #[validate]
    planes: Vec<PlaneValidator>,
    #[validate]
    crtcs: Vec<CrtcValidator>,
    #[validate]
    encoders: Vec<EncoderValidator>,
    #[validate]
    connectors: Vec<ConnectorValidator>,
}

#[derive(Debug, Validate, Deserialize)]
struct PlaneValidator {
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    name: String,
    #[validate(enumerate = ["primary", "overlay", "cursor"])]
    r#type: Option<String>,
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    possible_crtcs: Option<Vec<String>>,
}

#[derive(Debug, Validate, Deserialize)]
struct CrtcValidator {
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    name: String,

    writeback: Option<bool>,
}

#[derive(Debug, Validate, Deserialize)]
struct EncoderValidator {
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    name: String,
    #[validate(min_items = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    possible_crtcs: Option<Vec<String>>,
}

#[derive(Debug, Validate, Deserialize)]
struct ConnectorValidator {
    #[validate(min_length = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    name: String,
    #[validate(min_items = 1)]
    #[validate(pattern = r"^[a-zA-Z0-9._\- ]+$")]
    possible_encoders: Option<Vec<String>>,
}

/// Creates a VKMS device from the given JSON file.
///
/// # Errors
///
/// Returns an error if the JSON file is invalid or the VKMS device cannot be built.
pub fn create_vkms_device(configfs_path: &str, json_path: &str) -> Result<(), io::Error> {
    debug!("Building VKMS device from JSON file: {json_path}");
    let json_str = fs::read_to_string(json_path)?;

    let json: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let config = ConfigValidator::from_json_value(json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let builder = create_vkms_device_builder(&configfs_path, &config)?;
    builder.build()?;

    Ok(())
}

/// Returns a VKMS device builder from the given configuration.
///
/// # Errors
///
/// Returns an error if the configuration is invalid.
fn create_vkms_device_builder(
    configfs_path: &str,
    config: &ConfigValidator,
) -> Result<VkmsDeviceBuilder, io::Error> {
    debug!("Building VKMS device with name {}", config.name);
    let mut device = VkmsDeviceBuilder::new(&configfs_path, &config.name);
    debug!(" - Setting enabled status to {}", config.enabled);
    device = device.enabled(config.enabled);

    debug!("Adding planes to VKMS device:");
    for plane_config in &config.planes {
        debug!(" - Building plane with name {}", &plane_config.name);
        let mut plane = PlaneConfig::new(&plane_config.name);

        if let Some(kind) = &plane_config.r#type {
            debug!("   Setting plane type to {kind}");
            plane = match kind.as_str() {
                "primary" => plane.kind(PlaneKind::Primary),
                "overlay" => plane.kind(PlaneKind::Overlay),
                "cursor" => plane.kind(PlaneKind::Cursor),
                // This should never happen because the validator should catch it
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid plane type: {kind}"),
                    ))
                }
            };
        }

        if let Some(possible_crtcs) = &plane_config.possible_crtcs {
            debug!("   Setting possible CRTCs to {possible_crtcs:?}");
            plane = plane.possible_crtcs(possible_crtcs);
        }

        device = device.add_plane(plane);
    }

    debug!("Adding CRTCs to VKMS device:");
    for crtc_config in &config.crtcs {
        debug!(" - Building CRTC with name {}", &crtc_config.name);
        let mut crtc = CrtcConfig::new(&crtc_config.name);

        if let Some(writeback) = crtc_config.writeback {
            debug!("   Setting CRTC writeback enabled to {}", writeback);
            crtc = crtc.writeback_enabled(writeback);
        }

        device = device.add_crtc(crtc);
    }

    debug!("Adding encoders to VKMS device");
    for encoder_config in &config.encoders {
        debug!(" - Building encoder with name {}", &encoder_config.name);
        let mut encoder = EncoderConfig::new(&encoder_config.name);

        if let Some(possible_crtcs) = &encoder_config.possible_crtcs {
            debug!("   Setting possible CRTCs to {possible_crtcs:?}");
            encoder = encoder.possible_crtcs(possible_crtcs);
        }

        device = device.add_encoder(encoder);
    }

    debug!("Adding connectors to VKMS device:");
    for connector_config in &config.connectors {
        debug!(" - Building connector with name {}", &connector_config.name);
        let mut connector = ConnectorConfig::new(&connector_config.name);

        if let Some(possible_encoders) = &connector_config.possible_encoders {
            debug!("   Setting possible encoders to {possible_encoders:?}");
            connector = connector.possible_encoders(possible_encoders);
        }

        device = device.add_connector(connector);
    }

    Ok(device)
}
