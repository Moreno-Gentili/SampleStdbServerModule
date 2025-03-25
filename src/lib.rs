mod components;
use components::Movable;
use spacetimedb::{ReducerContext};

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // the "movables" table is not showing up here
    // _ctx.db.movables()
}
