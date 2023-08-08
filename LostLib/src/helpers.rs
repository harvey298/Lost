

#[macro_export]
macro_rules! HandleAbility3 {
    () => { warn!("Ability 3 ignored!") };

    ($commands:expr, $player:expr, $packet_channel:expr) => {

        let class = $player.class;

        match class {
            lost_lib::defaults::Class::Fighter => todo!(),
            lost_lib::defaults::Class::Engineer => { DoEngineerAbility3!($commands, $player, $packet_channel); },
            lost_lib::defaults::Class::Chronowatcher => todo!(),
            lost_lib::defaults::Class::Emperor => todo!(),
            lost_lib::defaults::Class::Mutant => todo!(),
            lost_lib::defaults::Class::Harbinger => todo!(),
            lost_lib::defaults::Class::Paladin => todo!(),
            lost_lib::defaults::Class::Mystic => todo!(),
        }
    }

}