use shared::packet::{Packet, ServerboundPacket, ServerboundPacketNonToken};

pub struct PacketH;

impl PacketH {
    pub fn login(username: String) -> Packet {
        Packet::Serverbound(ServerboundPacket::Login { username })
    }

    pub fn message(message: String, token: String) -> Packet {
        Packet::Serverbound(ServerboundPacket::Message { token, message })
    }

    pub fn heartbeat(token: String) -> Packet {
        Packet::Serverbound(ServerboundPacket::Heartbeat { token })
    }

    pub fn disconnect(token: String) -> Packet {
        Packet::Serverbound(ServerboundPacket::Disconnect { token })
    }
}

impl PacketH {
    pub fn login_nt(username: String) -> ServerboundPacketNonToken {
        ServerboundPacketNonToken::Login { username }
    }

    pub fn message_nt(message: String) -> ServerboundPacketNonToken {
        ServerboundPacketNonToken::Message { message }
    }

    pub fn heartbeat_nt() -> ServerboundPacketNonToken {
        ServerboundPacketNonToken::Heartbeat {}
    }

    pub fn disconnect_nt() -> ServerboundPacketNonToken {
        ServerboundPacketNonToken::Disconnect {}
    }
}

impl PacketH {
    // means from ServerboundPacketNonToken to ServerboundPacket
    pub fn from_spnt_to_sp(spnt: ServerboundPacketNonToken, token: String) -> ServerboundPacket {
        match spnt {
            ServerboundPacketNonToken::Login { username } => {
                ServerboundPacket::Login { username: username }
            }
            ServerboundPacketNonToken::Message { message } => {
                ServerboundPacket::Message { token, message }
            }
            ServerboundPacketNonToken::Heartbeat => ServerboundPacket::Heartbeat { token },
            ServerboundPacketNonToken::Disconnect => ServerboundPacket::Disconnect { token },
        }
    }
}
