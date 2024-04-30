use std::net::IpAddr;
use std::time::Instant;
use bytes::{Bytes, BytesMut};
use uuid::Uuid;
use valence_protocol::{PacketEncoder, Property};

//todo change that to be components in the ECS
pub struct PrimitiveClientComponents {
    /// The username for the client.
    pub username: String,
    /// UUID of the client.
    pub uuid: Uuid,
    /// IP address of the client.
    pub ip: IpAddr,
    /// Properties of this client from the game profile.
    pub properties: Vec<Property>,
    /// The abstract socket connection.
    pub conn: Box<dyn ClientConnection>,
    /// The packet encoder to use. This should be in sync with [`Self::conn`].
    pub enc: PacketEncoder,
}

#[derive(Clone, Debug)]
pub struct ReceivedPacket {
    /// The moment in time this packet arrived. This is _not_ the instant this
    /// packet was returned from [`ClientConnection::try_recv`].
    pub timestamp: Instant,
    /// This packet's ID.
    pub id: i32,
    /// The content of the packet, excluding the leading varint packet ID.
    pub body: Bytes,
}

/// Player properties from the game profile.
#[derive(Debug, Default)]
pub struct Properties(pub Vec<Property>);

impl Properties {
    /// Finds the property with the name "textures".
    pub fn textures(&self) -> Option<&Property> {
        self.0.iter().find(|p| p.name == "textures")
    }

    /// Finds the property with the name "textures" mutably.
    pub fn textures_mut(&mut self) -> Option<&mut Property> {
        self.0.iter_mut().find(|p| p.name == "textures")
    }

    /// Returns the value of the "textures" property. It's a base64-encoded
    /// JSON string that contains the skin and cape URLs.
    pub fn skin(&self) -> Option<&str> {
        self.textures().map(|p| p.value.as_str())
    }

    /// Sets the value of the "textures" property, or adds it if it does not
    /// exist. Can be used for custom skins on player entities.
    ///
    /// `signature` is the Yggdrasil signature for the texture data. It is
    /// required if you want the skin to show up on vanilla Notchian
    /// clients. You can't sign skins yourself, so you'll have to get it from
    /// Mojang.
    pub fn set_skin(&mut self, skin: impl Into<String>, signature: impl Into<String>) {
        if let Some(prop) = self.textures_mut() {
            prop.value = skin.into();
            prop.signature = Some(signature.into());
        } else {
            self.0.push(Property {
                name: "textures".to_owned(),
                value: skin.into(),
                signature: Some(signature.into()),
            });
        }
    }
}

/// Represents the bidirectional packet channel between the server and a client
/// in the "play" state.
pub trait ClientConnection: Send + Sync + 'static {
    /// Sends encoded clientbound packet data. This function must not block and
    /// the data should be sent as soon as possible.
    fn try_send(&mut self, bytes: BytesMut) -> anyhow::Result<()>;
    /// Receives the next pending serverbound packet. This must return
    /// immediately without blocking.
    fn try_recv(&mut self) -> anyhow::Result<Option<ReceivedPacket>>;
    /// The number of pending packets waiting to be received via
    /// [`Self::try_recv`].
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}