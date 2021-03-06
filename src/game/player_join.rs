use crate::player::Player;
use crate::game::packets::{Packet, PlayerInfoPlayer, PlayerInfoAction, WorldBorderAction, Slot};
use crate::game::position::Position;
use crate::game::nbt::{NBTTag, CompoundElement};
use crate::game::chat::ChatComponent;
use crate::data_writer::DataWriter;

/*
36 - join game
23 - n - plugin message
13 - n - server difficulty
66 - spawn position
48 - n - player abilities
63 - held item change
6 - n - statistics
14 - n - chat message
50 - player info (add player)
50 - player info (update latency)
52 - player position and look
61 - world border
78 - time update
19 - window items
 */

pub fn handle_join(player: &mut Player) {
    player.connection.send_packet(&Packet::KeepAlive {id: 0});
    player.connection.send_packet(&Packet::JoinGame {
        entity_id: 0,
        gamemode: 1,
        dimension: 0,
        difficulty: 0,
        max_players: 255,
        level_type: "teste".to_string(),
        reduced_debug_info: false
    });
    player.connection.send_packet(&Packet::SpawnPosition {location: Position {
        x: 0,
        y: 50,
        z: 0
    }});
    player.connection.send_packet(&Packet::HeldItemChange {slot: 0});
    player.connection.send_packet(&Packet::PlayerInfo {action_id: 0, players: vec!(PlayerInfoPlayer {
        uuid: player.uuid.clone(),
        action: PlayerInfoAction::AddPlayer {
            name: player.nickname.clone(),
            properties: vec!(),
            gamemode: 0,
            ping: 0,
            display_name: None
        }
    })});
    player.connection.send_packet(&Packet::PlayerPositionAndLook {
        x: 0.0,
        y: 50.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0
    });
    player.connection.send_packet(&Packet::WorldBorder {action: WorldBorderAction::SetSize {radius: 100f64}});
    player.connection.send_packet(&Packet::TimeUpdate {world_age: 0, time_of_day: 12000});

    let bitmask = 0b0000000000000001 as u16;
    let mut writer = DataWriter::new();
    let mut blocks = [[[0i32; 16]; 16]; 16];
    let stone = (1 << 4) | 0;

    for y in 0..16 {
        for z in 0..16 {
            for x in 0..16 {
                writer.write_u16_le(stone);
            }
        }
    }
    writer.write_varint(16);
    writer.write_varint((16 * 16 * 16) * 2);
    for x in 0..4096 {
        writer.write_u8(0);
    }
    for x in 0..4096 {
        writer.write_u8(0);
    }
    for x in 0..256 {
        writer.write_u8(1);
    }

    for y in 0..3 {
        for x in 0..3 {
            player.connection.send_packet(&Packet::ChunkData {
                bitmask,
                ground_up_continuous: true,
                x: y,
                y: x,
                data: writer.data.clone()
            });
        }
    }


    // player.connection.send_packet(&Packet::WindowItems {window_id: 0, slots: vec!(
    //     Slot {
    //         item_id: 1,
    //         item_count: Some(1),
    //         item_damage: Some(69),
    //         nbt: Some(NBTTag::Compound {
    //             compound: vec!(CompoundElement {
    //                 name: "display".to_owned(),
    //                 tag: NBTTag::Compound {
    //                     compound: vec!(
    //                         CompoundElement {name: "Name".to_owned(), tag: NBTTag::String {string: "Pyrocah".to_owned()}},
    //                     )
    //                 }
    //             })
    //         })
    //     }
    // )});
}