pub use altv::prelude::*;
use altv::AnimationFlags;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on_player_connect(|c| {
        let player = c.player.clone();
        player.spawn("mp_m_freemode_01", (0, 0, 72))?;
        player.emit("test", ["test".try_into()?, true.try_into()?])?;
        player.play_animation("cellphone@", "cellphone_text_in", Default::default())?;

        player.play_animation(
            "cellphone@",
            "cellphone_text_in",
            altv::PlayAnimation {
                flags: altv::AnimationFlags::HoldLastFrame | AnimationFlags::AbortOnWeaponDamage,
                ..Default::default()
            },
        )?;

        Ok(())
    });
}
