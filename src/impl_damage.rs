use crate::DamageType;
use crate::Player;

impl Player {
    pub fn hurt(&mut self, amount: f32) {
        self.health -= amount;
        self.health = self.health.max(0.0);
    }
    #[must_use] pub fn calculate_damage(
        &self,
        amount: f32,
        damage_types: &[DamageType],
    ) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }
        let mut mul: f32 = 1.0;
        for dt1 in self.get_weaknesses() {
            for dt2 in damage_types {
                if dt1 == *dt2 {
                    mul += 0.5;
                }
            }
        }
        amount * mul
    }
    #[must_use] pub fn calculate_fall_damage(&self) -> f32 {
        if self.velocity_y < 3.0 {
            return 0.0;
        }
        self.calculate_damage(self.velocity_y * 1.5, &[DamageType::Physical])
    }
    pub fn take_fall_damage(&mut self) {
        self.hurt(self.calculate_fall_damage());
    }
}
