use crate::{
    entity::player::GameMode,
    protocol::{ClientPacket, VarInt},
};

pub struct CLogin {
    entity_id: i32,
    is_hardcore: bool,
    dimension_count: VarInt,
    dimension_names: Vec<String>,
    max_players: VarInt,
    view_distance: VarInt,
    simulated_distance: VarInt,
    reduced_debug_info: bool,
    enabled_respawn_screen: bool,
    limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: String,
    hashed_seed: i64,
    game_mode: GameMode,
    previous_gamemode: GameMode,
    debug: bool,
    is_flat: bool,
    has_death_loc: bool,
    death_dimension_name: Option<String>,
    death_loc: Option<String>, // POSITION NOT STRING
    portal_cooldown: VarInt,
    enforce_secure_chat: bool,
}

impl CLogin {
    pub fn new(
        entity_id: i32,
        is_hardcore: bool,
        dimension_count: VarInt,
        dimension_names: Vec<String>,
        max_players: VarInt,
        view_distance: VarInt,
        simulated_distance: VarInt,
        reduced_debug_info: bool,
        enabled_respawn_screen: bool,
        limited_crafting: bool,
        dimension_type: VarInt,
        dimension_name: String,
        hashed_seed: i64,
        game_mode: GameMode,
        previous_gamemode: GameMode,
        debug: bool,
        is_flat: bool,
        has_death_loc: bool,
        death_dimension_name: Option<String>,
        death_loc: Option<String>,
        portal_cooldown: VarInt,
        enforce_secure_chat: bool,
    ) -> Self {
        Self {
            entity_id,
            is_hardcore,
            dimension_count,
            dimension_names,
            max_players,
            view_distance,
            simulated_distance,
            reduced_debug_info,
            enabled_respawn_screen,
            limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_gamemode,
            debug,
            is_flat,
            has_death_loc,
            death_dimension_name,
            death_loc,
            portal_cooldown,
            enforce_secure_chat,
        }
    }
}

impl ClientPacket for CLogin {
    const PACKET_ID: VarInt = 0x2B;

    fn write(&self, bytebuf: &mut crate::protocol::bytebuf::buffer::ByteBuffer) {
        bytebuf.write_i32(self.entity_id);
        bytebuf.write_bool(self.is_hardcore);
        bytebuf.write_var_int(self.dimension_count);
        bytebuf.write_string_array(self.dimension_names.as_slice());
        bytebuf.write_var_int(self.max_players);
        bytebuf.write_var_int(self.view_distance);
        bytebuf.write_var_int(self.simulated_distance);
        bytebuf.write_bool(self.reduced_debug_info);
        bytebuf.write_bool(self.enabled_respawn_screen);
        bytebuf.write_bool(self.limited_crafting);
        bytebuf.write_var_int(self.dimension_type);
        bytebuf.write_string(&self.dimension_name);
        bytebuf.write_i64(self.hashed_seed);
        bytebuf.write_u8(self.game_mode.to_byte() as u8);
        bytebuf.write_i8(self.previous_gamemode.to_byte());
        bytebuf.write_bool(self.debug);
        bytebuf.write_bool(self.is_flat);
        bytebuf.write_bool(self.has_death_loc);
        bytebuf.write_option(&self.death_dimension_name, |buf, v| buf.write_string(v));
        bytebuf.write_option(&self.death_loc, |buf, v| buf.write_string(v));
        bytebuf.write_var_int(self.portal_cooldown);
        bytebuf.write_bool(self.enforce_secure_chat);
    }
}