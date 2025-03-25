#[spacetimedb::table(name = movables, public)]
pub struct Movable {
    #[primary_key]
    pub entity_id: String,
    pub position: f32,
    pub velocity: f32,
}