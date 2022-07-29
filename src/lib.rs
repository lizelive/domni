#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Lore {
    pub title: String,
    pub description: String,
}

/// the current state of a value
pub struct Capacity {
    pub name: Token,
    pub lore: Ref<Lore>,
}

/// a value that can be changed
pub struct Attribute {
    pub name: Token,
    pub lore: Ref<Lore>,
}

pub type Token = String;

pub type Scaler = i32;
pub type Ref<T> = T;

pub enum ValueRef {
    Attribute(Ref<Attribute>),
    Capacity(Ref<Capacity>),
}

pub enum Value {
    Const(Scaler),
    Ref(ValueRef),
    Min(Vec<Value>),
    Max(Vec<Value>),
    Sum(Vec<Value>),
    Product(Vec<Value>),
}

pub enum Priority {
    Low,
    Medium,
    High,
}

pub struct Weighted<T> {
    weight: Value,
    value: T,
}

pub struct Modifiers {
    /// weighted and added to base value
    base: Vec<Weighted<Value>>,

    /// increase by this amount
    add: Vec<Value>,

    /// makes sure this value is at least this value
    mul: Vec<Value>,

    /// force the value to be at least
    floor: Vec<Value>,

    /// post-floor add
    late_add: Vec<Value>,

    /// makes sure value is capped at this value
    limit: Vec<Value>,
}

/*

MAX_SKILL_RUST = 3

TARGET_LIGHT_BOOST = lerp(clip(unlerp(TARGET_LIGHT, LIGHT_LOW_THRESHOLD, THRESHOLD_LIGHT_HIGH)), LIGHT_LOW_BOOST, HIGH_LIGHT_BOOST)
LOCAL_LIGHT_BOOST = lerp(clip(unlerp(LOCAL_LIGHT, LIGHT_LOW_THRESHOLD, THRESHOLD_LIGHT_HIGH)), LIGHT_LOW_BOOST, HIGH_LIGHT_BOOST)


ABILITY_MOVE = LOCAL_LIGHT_BOOST *

MAX_SKILL_MINER = 20

XP_BOOST_MINER = XP_BOOST_GLOBAL * percentage(PASSION_MINER)

ABILITY_MINER = SKILL_MINER * ( KINESTHETIC_SENSE * 3 + STRENGTH * 3 + TOUGHNESS * 2 + SPATIAL_SENSE * 2 + ENDURANCE * 1 + WILLPOWER * 1) * TOOL_MINING_POWER * LOCAL_LIGHT_BOOST * GLOBAL_FOCUS

MAX_SKILL_MINER = max(SKILL_MINER, MAX_SKILL_MINER)


SKILL_MINER = max(MAX_SKILL_MINER - MAX_SKILL_RUST ,  clamp(log2(ramp(XP_MINER)+1), INNATE_SKILL_MINER, MAX_SKILL_MINER))

COUNTER_MINER = acc(unitize(IS_MINING))

XP_MINER = ramp(XP_MINER + degrade(COUNTER_MINER)) + XP_BOOST_MINER * learn(COUNTER_MINER)

TIME_TO_MINE = TARGET_HARDNESS / ABILITY_MINER

 */

pub struct Race {
    innate: Skills,
}

struct PhysicalProperties {}

struct State {
    high: Option<Scaler>,
    low: Option<Scaler>,
    density: Scaler,
}

enum ThermalDefs {}

struct Material {
    spec_heat: Scaler,
    ignite_point: Option<Scaler>,
    heatdam_point: Option<Scaler>,
    colddam_point: Option<Scaler>,
    mat_fixed_temp: Option<Scaler>,

    melting_point: Scaler,
    boiling_point: Scaler,
    solid_density: Scaler,
    liquid_density: Scaler,

    molar_mass: Scaler,
    impact_yield: Scaler,
    impact_fracture: Scaler,
    impact_strain_at_yield: Scaler,
    compressive_yield: Scaler,
    compressive_fracture: Scaler,
    compressive_strain_at_yield: Scaler,
    tensile_yield: Scaler,
    tensile_fracture: Scaler,
    tensile_strain_at_yield: Scaler,
    torsion_yield: Scaler,
    torsion_fracture: Scaler,
    torsion_strain_at_yield: Scaler,
    shear_yield: Scaler,
    shear_fracture: Scaler,
    shear_strain_at_yield: Scaler,
    bending_yield: Scaler,
    bending_fracture: Scaler,
    bending_strain_at_yield: Scaler,
    max_edge: Scaler,
    absorption: Scaler,
}

struct Body {}

struct Caste {}

pub struct Creature {
    body: Body,
    castes: Vec<Caste>,
}

pub type Skills = Vec<Skill>;

pub struct Skill {
    /// the max level obtained
    max: Scaler,

    /// the xp
    xp: Scaler,
}

impl Modifiers {
    pub fn evaluate(&self) -> Scaler {
        let base = {
            let mut base_acc: Scaler = 0;
            let mut base_weight_acc: Scaler = 0;

            for base in self.base {
                base_acc += base.value.evaluate();
                base_weight_acc += base.weight.evaluate();
            }

            if base_weight_acc == 0 {
                0
            } else {
                base_acc / base_weight_acc
            }
        };
    }
}

impl Value {
    pub fn evaluate(&self) -> Scaler {
        todo!()
    }
}

fn modify(value: Capacity, modifiers: Vec<Mod>) -> Scaler {
    unimplemented!()
}
