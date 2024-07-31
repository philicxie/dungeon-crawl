use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims :Vec<(Entity, Entity, i32)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim, attack.name))
        .collect();
    victims.iter().for_each(|(message, victim, name)| {
        if let Ok(mut health) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
            println!("Health before attack: {} for {}", health.current, name);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {} for {}", health.current, name);
        }
        commands.remove(*message);
    });
}