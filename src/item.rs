#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ItemRarity {
    Common,
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ItemName {
    Stone,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ItemType {
    Weapon,
    Tool,
    Material,
    Equipment,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct WeaponProps {
    damage: f32,   // how hard this hits
    cooldown: f32, // how fast this attacks
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct ToolProps {
    damage: f32,     // how hard this hits blocks
    multiplier: f32, // how many more items should you get for breaking
    spread: f32,     // how many extra blocks do you break (round down always)
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ItemProps {
    Weapon(WeaponProps),
    Tool(ToolProps),
    None,
}
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Item {
    pub max_stack: usize,
    pub amount: usize,
    pub rarity: ItemRarity,
    pub name: ItemName,
    pub color: (u8, u8, u8),
    pub props: ItemProps,
}
impl Item {
    #[must_use] pub fn new(
        max_stack: usize,
        amount: usize,
        rarity: ItemRarity,
        name: ItemName,
        color: (u8, u8, u8),
        props: ItemProps,
    ) -> Self {
        Self {
            max_stack,
            amount,
            rarity,
            name,
            color,
            props,
        }
    }

    #[must_use] pub fn is_weapon(&self) -> bool {
        matches!(self.props, ItemProps::Weapon(_))
    }

    #[must_use] pub fn weapon_props(&self) -> Option<&WeaponProps> {
        if let ItemProps::Weapon(w) = &self.props {
            Some(w)
        } else {
            None
        }
    }

    #[must_use] pub fn tool_props(&self) -> Option<&ToolProps> {
        if let ItemProps::Tool(t) = &self.props {
            Some(t)
        } else {
            None
        }
    }
}
