use std::io;

#[derive(Debug)]
pub enum PlaneKind {
    Primary,
    Overlay,
    Cursor,
}
#[derive(Debug, Default)]
pub struct VkmsDeviceBuilder {
    name: String,
    planes: Vec<PlaneConfig>,
    crtcs: Vec<CrtcConfig>,
    encoders: Vec<EncoderConfig>,
    connectors: Vec<ConnectorConfig>,
}

#[derive(Debug)]
pub struct PlaneConfig {
    name: String,
    kind: PlaneKind,
    possible_crtcs: Vec<String>,
}

#[derive(Debug)]
pub struct CrtcConfig {
    name: String,
}

#[derive(Debug, Default)]
pub struct EncoderConfig {
    name: String,
    possible_crtcs: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ConnectorConfig {
    name: String,
    possible_encoders: Vec<String>,
}

impl VkmsDeviceBuilder {
    pub fn new(name: &str) -> Self {
        VkmsDeviceBuilder {
            name: name.to_owned(),
            ..VkmsDeviceBuilder::default()
        }
    }

    pub fn add_plane(mut self, plane: PlaneConfig) -> Self {
        self.planes.push(plane);
        self
    }

    pub fn add_crtc(mut self, crtc: CrtcConfig) -> Self {
        self.crtcs.push(crtc);
        self
    }

    pub fn add_encoder(mut self, encoder: EncoderConfig) -> Self {
        self.encoders.push(encoder);
        self
    }

    pub fn add_connector(mut self, connector: ConnectorConfig) -> Self {
        self.connectors.push(connector);
        self
    }

    pub fn build(self) -> Result<(), io::Error> {
        // TODO: Implement the actual VKMS device creation logic
        Ok(())
    }
}

impl PlaneConfig {
    pub fn new(name: &str) -> Self {
        PlaneConfig {
            name: name.to_owned(),
            kind: PlaneKind::Primary,
            possible_crtcs: Vec::new(),
        }
    }

    pub fn kind(mut self, kind: PlaneKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn possible_crtcs(mut self, possible_crtcs: &[String]) -> Self {
        self.possible_crtcs = possible_crtcs.to_owned();
        self
    }
}

impl CrtcConfig {
    pub fn new(name: &str) -> Self {
        CrtcConfig {
            name: name.to_owned(),
        }
    }
}

impl EncoderConfig {
    pub fn new(name: &str) -> Self {
        EncoderConfig {
            name: name.to_owned(),
            possible_crtcs: Vec::new(),
        }
    }

    pub fn possible_crtcs(mut self, possible_crtcs: &[String]) -> Self {
        self.possible_crtcs = possible_crtcs.to_owned();
        self
    }
}

impl ConnectorConfig {
    pub fn new(name: &str) -> Self {
        ConnectorConfig {
            name: name.to_owned(),
            possible_encoders: Vec::new(),
        }
    }

    pub fn possible_encoders(mut self, possible_encoders: &[String]) -> Self {
        self.possible_encoders = possible_encoders.to_owned();
        self
    }
}
