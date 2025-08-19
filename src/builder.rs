//! VKMS device builder.
//!
//! The `VkmsDeviceBuilder` struct is used to build a VKMS device.
//!
//! A `VkmsDeviceBuilder` is composed of:
//!
//! - Planes
//! - CRTCs
//! - Encoders
//! - Connectors
//!
//! Each of these components is represented by a struct, and each struct contains its configuration.

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
    /// Whether the VKMS device is enabled or not, stored in `vkms/<device name>/enabled`.
    enabled: bool,
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

/// Connector status.
#[derive(Debug)]
pub enum ConnectorStatus {
    Connected,
    Disconnected,
    Unknown,
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
    /// Whether a VKMS CRTC writeback connector is enabled or not, stored in
    /// `crtcs/<crtc name>/writeback`. `false` by default.
    is_writeback_enabled: bool,
}

/// Encoder configuration.
#[derive(Debug)]
pub struct EncoderConfig {
    /// Name of the encoder, used as the name of the encoder node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/encoders/<encoder name>`.
    name: String,
    /// Possible CRTCs for the encoder, stored in `encoders/<encoder name>/possible_crtcs` as
    /// symbolic links to the CRTC nodes.
    possible_crtcs: Vec<String>,
}

/// Connector configuration.
#[derive(Debug)]
pub struct ConnectorConfig {
    /// Name of the connector, used as the name of the connector node in configfs, for example:
    /// `/sys/kernel/config/vkms/<device name>/connectors/<connector name>`.
    name: String,
    /// Status of the connector, stored in `connectors/<connector name>/status`.
    status: ConnectorStatus,
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
            enabled: false,
            planes: Vec::new(),
            crtcs: Vec::new(),
            encoders: Vec::new(),
            connectors: Vec::new(),
        }
    }

    /// Given a configfs path and a device name, builds a `VkmsDeviceBuilder` from the current
    /// filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the VKMS device cannot be created.
    pub fn from_fs(configfs_path: &str, name: &str) -> Result<Self, io::Error> {
        let mut device = Self::new(configfs_path, name);

        // Set the device enabled status
        let enabled = fs::read_to_string(format!("{}/enabled", &device.path()))?;
        device = device.enabled(enabled.trim() == "1");

        // Create the device planes
        let planes_path = format!("{}/planes", &device.path());
        for plane_dir in fs::read_dir(&planes_path)? {
            let name = plane_dir?.file_name().to_string_lossy().into_owned();
            let plane = PlaneConfig::from_fs(&planes_path, &name)?;
            device = device.add_plane(plane);
        }

        // Create the device CRTCs
        let crtcs_path = format!("{}/crtcs", &device.path());
        for crtc_dir in fs::read_dir(&crtcs_path)? {
            let name = crtc_dir?.file_name().to_string_lossy().into_owned();
            let crtc = CrtcConfig::from_fs(&crtcs_path, &name)?;
            device = device.add_crtc(crtc);
        }

        // Create the device encoders
        let encoders_path = format!("{}/encoders", &device.path());
        for encoder_dir in fs::read_dir(&encoders_path)? {
            let name = encoder_dir?.file_name().to_string_lossy().into_owned();
            let encoder = EncoderConfig::from_fs(&encoders_path, &name)?;
            device = device.add_encoder(encoder);
        }

        // Create the device connectors
        let connectors_path = format!("{}/connectors", &device.path());
        for connector_dir in fs::read_dir(&connectors_path)? {
            let name = connector_dir?.file_name().to_string_lossy().into_owned();
            let connector = ConnectorConfig::from_fs(&connectors_path, &name)?;
            device = device.add_connector(connector);
        }

        Ok(device)
    }

    /// Returns the path to the VKMS device.
    pub fn path(&self) -> String {
        format!("{}/vkms/{}", self.configfs_path, self.name)
    }

    /// Sets the enabled status of the VKMS device.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
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
        fs::create_dir(self.path())?;

        // Create the CRTC nodes at /sys/kernel/config/vkms/<device name>/crtcs/<crtc name>
        for crtc in &self.crtcs {
            let crtc_path = format!("{}/crtcs/{}", &self.path(), &crtc.name);
            fs::create_dir(&crtc_path)?;

            // Set the writeback mode of the CRTC
            let is_writeback = if crtc.is_writeback_enabled {
                b"1"
            } else {
                b"0"
            };
            fs::write(format!("{}/writeback", &crtc_path), is_writeback)?;
        }

        // Create the plane nodes at /sys/kernel/config/vkms/<device name>/planes/<plane name>
        for plane in &self.planes {
            let plane_path = format!("{}/planes/{}", &self.path(), &plane.name);
            fs::create_dir(&plane_path)?;

            // Set the type of the plane
            let kind = match plane.kind {
                PlaneKind::Overlay => b"0",
                PlaneKind::Primary => b"1",
                PlaneKind::Cursor => b"2",
            };
            fs::write(format!("{}/type", &plane_path), kind)?;

            // Link with the possible CRTCs for the plane
            for crtc in &plane.possible_crtcs {
                let original_crtc = format!("{}/crtcs/{}", &self.path(), &crtc);
                let linked_crtc = format!("{}/possible_crtcs/{}", &plane_path, &crtc);
                os::unix::fs::symlink(&original_crtc, &linked_crtc)?;
            }
        }

        // Create the encoder nodes at /sys/kernel/config/vkms/<device name>/encoders/<encoder name>
        for encoder in &self.encoders {
            let encoder_path = format!("{}/encoders/{}", &self.path(), &encoder.name);
            fs::create_dir(&encoder_path)?;

            // Link with the possible CRTCs for the encoder
            for crtc in &encoder.possible_crtcs {
                let original_crtc = format!("{}/crtcs/{}", &self.path(), &crtc);
                let linked_crtc = format!("{}/possible_crtcs/{}", &encoder_path, &crtc);
                os::unix::fs::symlink(&original_crtc, &linked_crtc)?;
            }
        }

        // Create the connector nodes at /sys/kernel/config/vkms/<device name>/connectors/<connector name>
        for connector in &self.connectors {
            let connector_path = format!("{}/connectors/{}", &self.path(), &connector.name);
            fs::create_dir(&connector_path)?;

            // Set the status of the connector
            let status = match connector.status {
                ConnectorStatus::Connected => b"1",
                ConnectorStatus::Disconnected => b"2",
                ConnectorStatus::Unknown => b"3",
            };
            fs::write(format!("{}/status", &connector_path), status)?;

            // Link with the possible encoders for the connector
            for encoder in &connector.possible_encoders {
                let original_encoder = format!("{}/encoders/{}", &self.path(), &encoder);
                let linked_encoder = format!("{}/possible_encoders/{}", &connector_path, &encoder);
                os::unix::fs::symlink(&original_encoder, &linked_encoder)?;
            }
        }

        // Enable the VKMS device
        let enabled = if self.enabled { b"1" } else { b"0" };
        fs::write(format!("{}/enabled", &self.path()), enabled)?;

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

    /// Given a path to the planes directory (e.g. `/sys/kernel/config/vkms/<device name>/planes`)
    /// and a plane name, builds a `PlaneConfig` from the current filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the plane cannot be created.
    pub fn from_fs(planes_path: &str, name: &str) -> Result<Self, io::Error> {
        let mut plane = Self::new(name);
        let plane_path = format!("{planes_path}/{name}");

        // Set the type of the plane
        let kind_str = fs::read_to_string(format!("{}/type", &plane_path))?;
        let kind = match kind_str.trim() {
            "0" => PlaneKind::Overlay,
            "1" => PlaneKind::Primary,
            "2" => PlaneKind::Cursor,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid plane type",
                ))
            }
        };
        plane = plane.kind(kind);

        // Set the possible CRTCs for the plane
        let possible_crtcs_path = format!("{}/possible_crtcs", &plane_path);
        let mut possible_crtcs = Vec::new();
        for possible_crtc_link in fs::read_dir(possible_crtcs_path)? {
            let target = fs::read_link(possible_crtc_link?.path())?;
            let target_name = target.file_name().unwrap().to_string_lossy().into_owned();
            possible_crtcs.push(target_name);
        }
        plane = plane.possible_crtcs(&possible_crtcs);

        Ok(plane)
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

    /// Given a path to the CRTCs directory (e.g. `/sys/kernel/config/vkms/<device name>/crtcs`)
    /// and a CRTC name, builds a `CrtcConfig` from the current filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the CRTC cannot be created.
    pub fn from_fs(crtcs_path: &str, name: &str) -> Result<Self, io::Error> {
        let mut crtc = Self::new(name);
        let crtc_path = format!("{crtcs_path}/{name}");

        // Set if the writeback is enabled or not
        let is_writeback_enabled = fs::read_to_string(format!("{}/writeback", &crtc_path))?;
        crtc = crtc.writeback_enabled(is_writeback_enabled.trim() == "1");

        Ok(crtc)
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

    /// Given a path to the encoders directory (e.g. `/sys/kernel/config/vkms/<device name>/encoders`)
    /// and an encoder name, builds a `EncoderConfig` from the current filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder cannot be created.
    pub fn from_fs(encoders_path: &str, name: &str) -> Result<Self, io::Error> {
        let mut encoder = Self::new(name);
        let encoder_path = format!("{encoders_path}/{name}");

        // Set the possible CRTCs for the encoder
        let possible_crtcs_path = format!("{}/possible_crtcs", &encoder_path);
        let mut possible_crtcs = Vec::new();
        for possible_crtc_link in fs::read_dir(possible_crtcs_path)? {
            let target = fs::read_link(possible_crtc_link?.path())?;
            let target_name = target.file_name().unwrap().to_string_lossy().into_owned();
            possible_crtcs.push(target_name);
        }
        encoder = encoder.possible_crtcs(&possible_crtcs);

        Ok(encoder)
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
            status: ConnectorStatus::Connected,
            possible_encoders: Vec::new(),
        }
    }

    /// Given a path to the connectors directory (e.g. `/sys/kernel/config/vkms/<device name>/connectors`)
    /// and a connector name, builds a `ConnectorConfig` from the current filesystem state.
    ///
    /// # Errors
    ///
    /// Returns an error if the connector cannot be created.
    pub fn from_fs(connectors_path: &str, name: &str) -> Result<Self, io::Error> {
        let mut connector = Self::new(name);
        let connector_path = format!("{connectors_path}/{name}");

        // Set the status of the connector
        let status = fs::read_to_string(format!("{}/status", &connector_path))?;
        let status = match status.trim() {
            "1" => ConnectorStatus::Connected,
            "2" => ConnectorStatus::Disconnected,
            "3" => ConnectorStatus::Unknown,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid connector status",
                ))
            }
        };
        connector = connector.status(status);

        // Set the possible encoders for the connector
        let possible_encoders_path = format!("{}/possible_encoders", &connector_path);
        let mut possible_encoders = Vec::new();
        for possible_encoder_link in fs::read_dir(possible_encoders_path)? {
            let target = fs::read_link(possible_encoder_link?.path())?;
            let target_name = target.file_name().unwrap().to_string_lossy().into_owned();
            possible_encoders.push(target_name);
        }
        connector = connector.possible_encoders(&possible_encoders);

        Ok(connector)
    }

    /// Sets the status of the connector.
    pub fn status(mut self, status: ConnectorStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the possible encoders for the connector.
    pub fn possible_encoders(mut self, possible_encoders: &[String]) -> Self {
        self.possible_encoders = possible_encoders.to_owned();
        self
    }
}
