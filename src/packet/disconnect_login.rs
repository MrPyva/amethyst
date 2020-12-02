use crate::net::network_manager::MinecraftClient;
use std::sync::Arc;
use crate::game::chat::ChatComponent;
use crate::packet::WritePacket;
use crate::data_writer::DataWriter;

#[derive(Debug)]
pub struct PacketDisconnectLogin {
    pub client: Arc<MinecraftClient>,
    pub reason: ChatComponent
}

impl WritePacket for PacketDisconnectLogin {
    fn write(&self) -> Vec<u8> {
        let mut writer = DataWriter::new();
        println!("{}", self.reason.to_json());

        writer.write_u8(0x00);
        writer.write_string(&self.reason.to_json());
        writer.set_lenght(writer.data.len() as u32);

        writer.data
    }
}