use std::fs;
use std::io;
use std::os;

/// VKMS device builder.
#[derive(Debug, Default)]
pub struct VkmsDeviceBuilder {
    /// Path to the configfs directory, usually `/sys/kernel/config`.
    configfs_path: String,
    /// Name of the VKMS device, used as the name of the device node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>`.
    name: String,
    /// Planes of the VKMS device.
    planes: Vec<PlaneConfig>,
    /// CRTCs of the VKMS device.
    crtcs: Vec<CrtcConfig>,
    /// Encoders of the VKMS device.
    encoders: Vec<EncoderConfig>,
    /// Connectors of the VKMS device.
    connectors: Vec<ConnectorConfig>,
}

/// Valid plane types, as defined in the kernel.
#[derive(Debug)]
pub enum PlaneKind {
    Overlay,
    Primary,
    Cursor,
}

/// Plane configuration.
#[derive(Debug)]
pub struct PlaneConfig {
    /// Name of the plane, used as the name of the plane node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/planes/<plane name>`.
    name: String,
    /// Type of the plane, stored in `planes/<plane name>/type`.
    kind: PlaneKind,
    /// Possible CRTCs for the plane, stored in `planes/<plane name>/possible_crtcs` as symbolic
    /// links to the CRTC nodes.
    possible_crtcs: Vec<String>,
}

/// CRTC configuration.
#[derive(Debug)]
pub struct CrtcConfig {
    /// Name of the CRTC, used as the name of the CRTC node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/crtcs/<crtc name>`.
    name: String,
    /// Whether a VKMS CRTC writeback connector is enabled or not, stored in `crtcs/<crtc name>/writeback`.
    /// `false` by default.
    is_writeback_enabled: bool,
}

/// Encoder configuration.
#[derive(Debug, Default)]
pub struct EncoderConfig {
    /// Name of the encoder, used as the name of the encoder node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/encoders/<encoder name>`.
    name: String,
    /// Possible CRTCs for the encoder, stored in `encoders/<encoder name>/possible_crtcs` as
    /// symbolic links to the CRTC nodes.
    possible_crtcs: Vec<String>,
}

/// Connector configuration.
#[derive(Debug, Default)]
pub struct ConnectorConfig {
    /// Name of the connector, used as the name of the connector node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/connectors/<connector name>`.
    name: String,
    /// Possible encoders for the connector, stored in
    /// `connectors/<connector name>/possible_encoders` as symbolic links to the encoder nodes.
    possible_encoders: Vec<String>,
}

impl VkmsDeviceBuilder {
    /// Creates a new VKMS device builder. See the `VkmsDeviceBuilder` struct documentation for
    /// more information.
    pub fn new(configfs_path: &str, name: &str) -> Self {
        VkmsDeviceBuilder {
            configfs_path: configfs_path.to_owned(),
            name: name.to_owned(),
            ..VkmsDeviceBuilder::default()
        }
    }

    /// Adds a plane to the VKMS device.
    pub fn add_plane(mut self, plane: PlaneConfig) -> Self {
        self.planes.push(plane);
        self
    }

    /// Adds a CRTC to the VKMS device.
    pub fn add_crtc(mut self, crtc: CrtcConfig) -> Self {
        self.crtcs.push(crtc);
        self
    }

    /// Adds an encoder to the VKMS device.
    pub fn add_encoder(mut self, encoder: EncoderConfig) -> Self {
        self.encoders.push(encoder);
        self
    }

    /// Adds a connector to the VKMS device.
    pub fn add_connector(mut self, connector: ConnectorConfig) -> Self {
        self.connectors.push(connector);
        self
    }

    /// Builds the VKMS device.
    ///
    /// # Errors
    ///
    /// Returns an error if the VKMS device cannot be created.
    pub fn build(self) -> Result<(), io::Error> {
        // Create the device node at /sys/kernel/config/vkms/<device name>
        let device_path = format!("{}/vkms/{}", self.configfs_path, self.name);
        fs::create_dir(&device_path)?;

        // Create the CRTC nodes at /sys/kernel/config/vkms/<device name>/crtcs/<crtc name>
        for crtc in self.crtcs {
            let crtc_path = format!("{}/crtcs/{}", &device_path, &crtc.name);
            fs::create_dir(&crtc_path)?;

            // Set the writeback mode of the CRTC
            let is_writeback = if crtc.is_writeback_enabled {
                b"1"
            } else {
                b"0"
            };
            fs::write(format!("{}/writeback", &crtc_path), &is_writeback)?;
        }

        // Create the plane nodes at /sys/kernel/config/vkms/<device name>/planes/<plane name>
        for plane in self.planes {
            let plane_path = format!("{}/planes/{}", &device_path, &plane.name);
            fs::create_dir(&plane_path)?;

            // Set the type of the plane
            let kind = match plane.kind {
                PlaneKind::Overlay => b"0",
                PlaneKind::Primary => b"1",
                PlaneKind::Cursor => b"2",
            };
            fs::write(format!("{}/type", &plane_path), &kind)?;

            // Link with the possible CRTCs for the plane
            for crtc in plane.possible_crtcs {
                let original_crtc = format!("{}/crtcs/{}", &device_path, &crtc);
                let linked_crtc = format!("{}/possible_crtcs/{}", &plane_path, &crtc);
                os::unix::fs::symlink(&original_crtc, &linked_crtc)?;
            }
        }

        // Create the encoder nodes at /sys/kernel/config/vkms/<device name>/encoders/<encoder name>
        for encoder in self.encoders {
            let encoder_path = format!("{}/encoders/{}", &device_path, &encoder.name);
            fs::create_dir(&encoder_path)?;

            // Link with the possible CRTCs for the encoder
            for crtc in encoder.possible_crtcs {
                let original_crtc = format!("{}/crtcs/{}", &device_path, &crtc);
                let linked_crtc = format!("{}/possible_crtcs/{}", &encoder_path, &crtc);
                os::unix::fs::symlink(&original_crtc, &linked_crtc)?;
            }
        }

        // Create the connector nodes at /sys/kernel/config/vkms/<device name>/connectors/<connector name>
        for connector in self.connectors {
            let connector_path = format!("{}/connectors/{}", &device_path, &connector.name);
            fs::create_dir(&connector_path)?;

            // Link with the possible encoders for the connector
            for encoder in connector.possible_encoders {
                let original_encoder = format!("{}/encoders/{}", &device_path, &encoder);
                let linked_encoder = format!("{}/possible_encoders/{}", &connector_path, &encoder);
                os::unix::fs::symlink(&original_encoder, &linked_encoder)?;
            }
        }

        // Enable the VKMS device
        fs::write(format!("{}/enabled", &device_path), b"1")?;

        Ok(())
    }
}

impl PlaneConfig {
    /// Creates a new plane configuration. See the `PlaneConfig` struct documentation for more
    /// information.
    pub fn new(name: &str) -> Self {
        PlaneConfig {
            name: name.to_owned(),
            kind: PlaneKind::Overlay,
            possible_crtcs: Vec::new(),
        }
    }

    /// Sets the type of the plane.
    pub fn kind(mut self, kind: PlaneKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the possible CRTCs for the plane.
    pub fn possible_crtcs(mut self, possible_crtcs: &[String]) -> Self {
        self.possible_crtcs = possible_crtcs.to_owned();
        self
    }
}

impl CrtcConfig {
    /// Creates a new CRTC configuration. See the `CrtcConfig` struct documentation for more
    /// information.
    pub fn new(name: &str) -> Self {
        CrtcConfig {
            name: name.to_owned(),
            is_writeback_enabled: false,
        }
    }

    /// Sets the VKMS CRTC writeback connector status.
    pub fn writeback_enabled(mut self, writeback: bool) -> Self {
        self.is_writeback_enabled = writeback;
        self
    }
}

impl EncoderConfig {
    /// Creates a new encoder configuration. See the `EncoderConfig` struct documentation for more
    /// information.
    pub fn new(name: &str) -> Self {
        EncoderConfig {
            name: name.to_owned(),
            possible_crtcs: Vec::new(),
        }
    }

    /// Sets the possible CRTCs for the encoder.
    pub fn possible_crtcs(mut self, possible_crtcs: &[String]) -> Self {
        self.possible_crtcs = possible_crtcs.to_owned();
        self
    }
}

impl ConnectorConfig {
    /// Creates a new connector configuration. See the `ConnectorConfig` struct documentation for
    /// more information.
    pub fn new(name: &str) -> Self {
        ConnectorConfig {
            name: name.to_owned(),
            possible_encoders: Vec::new(),
        }
    }

    /// Sets the possible encoders for the connector.
    pub fn possible_encoders(mut self, possible_encoders: &[String]) -> Self {
        self.possible_encoders = possible_encoders.to_owned();
        self
    }
}
