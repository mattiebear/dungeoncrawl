use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // First time this runs the player is the only attacker.
    // Second time it's all of the enemies
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    victims.iter().for_each(|(intent, victim)| {
        if let Ok(health) = ecs
            .entry_mut(*victim) // This is how we query the health when all we have is the Entity
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;

            if health.current <= 0 {
                commands.remove(*victim);
            }

            println!("Health after attack: {}", health.current);
        }

        commands.remove(*intent);
    })
}
