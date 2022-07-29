use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SphereEnum {
    /// Sphere: `AGRICULTURE`
    /// - Friend Spheres: `FOOD`, `FERTILITY`, `RAIN`
    #[serde(alias = "AGRICULTURE")]
    Agriculture,
    /// Sphere: `ANIMALS`
    /// - Friend Spheres: `PLANTS`
    /// - Parent/Child Spheres: `NATURE`
    #[serde(alias = "ANIMALS")]
    Animals,
    /// Sphere: `ART`
    /// - Friend Spheres: `INSPIRATION`, `BEAUTY`
    /// - Parent/Child Spheres: `DANCE`, `MUSIC`, `PAINTING`, `POETRY`, `SONG`
    #[serde(alias = "ART")]
    Art,
    /// Sphere: `BALANCE`
    #[serde(alias = "BALANCE")]
    Balance,
    /// Sphere: `BEAUTY`
    /// - Friend Spheres: `ART`
    /// - Precluded Spheres: `BLIGHT`, `DEFORMITY`, `DISEASE`, `MUCK`  
    #[serde(alias = "BEAUTY")]
    Beauty,
    /// Sphere: `BIRTH`
    /// - Friend Spheres: `CHILDREN`, `CREATION`, `FAMILY`, `MARRIAGE`, `PREGNANCY`, `REBIRTH`, `YOUTH`
    #[serde(alias = "BIRTH")]
    Birth,
    /// Sphere: `BLIGHT`
    /// - Friend Spheres: `DISEASE`, `DEATH`
    /// - Precluded Spheres: `BEAUTY`, `FOOD`, `FERTILITY`, `HEALING`  
    #[serde(alias = "BLIGHT")]
    Blight,
    /// Sphere: `BOUNDARIES`
    /// - Parent/Child Spheres: `COASTS`
    #[serde(alias = "BOUNDARIES")]
    Boundaries,
    /// Sphere: `CAVERNS`
    /// - Friend Spheres: `MOUNTAINS`, `EARTH`
    #[serde(alias = "CAVERNS")]
    Caverns,
    /// Sphere: `CHAOS`
    /// - Friend Spheres: `WAR`
    /// - Precluded Spheres: `DISCIPLINE`, `ORDER`, `LAWS`  
    #[serde(alias = "CHAOS")]
    Chaos,
    /// Sphere: `CHARITY`
    /// - Friend Spheres: `GENEROSITY`, `SACRIFICE`
    /// - Precluded Spheres: `JEALOUSY`  
    #[serde(alias = "CHARITY")]
    Charity,
    /// Sphere: `CHILDREN`
    /// - Friend Spheres: `BIRTH`, `FAMILY`, `YOUTH`, `PREGNANCY`
    #[serde(alias = "CHILDREN")]
    Children,
    /// Sphere: `COASTS`
    /// - Friend Spheres: `LAKES`, `OCEANS`
    /// - Parent/Child Spheres: `BOUNDARIES`
    #[serde(alias = "COASTS")]
    Coasts,
    /// Sphere: `CONSOLATION`
    /// - Precluded Spheres: `MISERY`  
    #[serde(alias = "CONSOLATION")]
    Consolation,
    /// Sphere: `COURAGE`
    /// - Parent/Child Spheres: `VALOR`
    #[serde(alias = "COURAGE")]
    Courage,
    /// Sphere: `CRAFTS`
    /// - Friend Spheres: `CREATION`, `LABOR`, `METALS`
    #[serde(alias = "CRAFTS")]
    Crafts,
    /// Sphere: `CREATION`
    /// - Friend Spheres: `CRAFTS`, `BIRTH`, `PREGNANCY`, `REBIRTH`
    #[serde(alias = "CREATION")]
    Creation,
    /// Sphere: `DANCE`
    /// - Friend Spheres: `FESTIVALS`, `MUSIC`, `REVELRY`
    /// - Parent/Child Spheres: `ART`
    #[serde(alias = "DANCE")]
    Dance,
    /// Sphere: `DARKNESS`
    /// - Friend Spheres: `NIGHT`
    /// - Precluded Spheres: `DAWN`, `DAY`, `LIGHT`, `TWILIGHT`, `SUN`  
    #[serde(alias = "DARKNESS")]
    Darkness,
    /// Sphere: `DAWN`
    /// - Friend Spheres: `SUN`, `TWILIGHT`
    /// - Precluded Spheres: `NIGHT`, `DAY`, `DARKNESS`  
    #[serde(alias = "DAWN")]
    Dawn,
    /// Sphere: `DAY`
    /// - Friend Spheres: `LIGHT`, `SUN`
    /// - Precluded Spheres: `DARKNESS`, `NIGHT`, `DAWN`, `DUSK`, `DREAMS`, `NIGHTMARES`, `TWILIGHT`  
    #[serde(alias = "DAY")]
    Day,
    /// Sphere: `DEATH`
    /// - Friend Spheres: `BLIGHT`, `DISEASE`, `MURDER`, `REBIRTH`, `SUICIDE`, `WAR`
    /// - Precluded Spheres: `HEALING`, `LONGEVITY`, `YOUTH`  
    #[serde(alias = "DEATH")]
    Death,
    /// Sphere: `DEFORMITY`
    /// - Friend Spheres: `DISEASE`
    /// - Precluded Spheres: `BEAUTY`  
    #[serde(alias = "DEFORMITY")]
    Deformity,
    /// Sphere: `DEPRAVITY`
    /// - Friend Spheres: `LUST`
    /// - Precluded Spheres: `LAWS`  
    #[serde(alias = "DEPRAVITY")]
    Depravity,
    /// Sphere: `DISCIPLINE`
    /// - Friend Spheres: `LAWS`, `ORDER`
    /// - Precluded Spheres: `CHAOS`  
    #[serde(alias = "DISCIPLINE")]
    Discipline,
    /// Sphere: `DISEASE`
    /// - Friend Spheres: `BLIGHT`, `DEATH`, `DEFORMITY`
    /// - Precluded Spheres: `BEAUTY`, `HEALING`  
    #[serde(alias = "DISEASE")]
    Disease,
    /// Sphere: `DREAMS`
    /// - Friend Spheres: `NIGHT`, `NIGHTMARES`
    /// - Precluded Spheres: `DAY`  
    #[serde(alias = "DREAMS")]
    Dreams,
    /// Sphere: `DUSK`
    /// - Friend Spheres: `TWILIGHT`
    /// - Precluded Spheres: `NIGHT`, `DAY`  
    #[serde(alias = "DUSK")]
    Dusk,
    /// Sphere: `DUTY`
    /// - Friend Spheres: `ORDER`
    #[serde(alias = "DUTY")]
    Duty,
    /// Sphere: `EARTH`
    /// - Friend Spheres: `CAVERNS`, `MOUNTAINS`, `VOLCANOS`
    /// - Parent/Child Spheres: `METALS`, `MINERALS`, `SALT`
    #[serde(alias = "EARTH")]
    Earth,
    /// Sphere: `FAMILY`
    /// - Friend Spheres: `BIRTH`, `CHILDREN`, `MARRIAGE`, `PREGNANCY`
    #[serde(alias = "FAMILY")]
    Family,
    /// Sphere: `FAME`
    /// - Friend Spheres: `RUMORS`
    /// - Precluded Spheres: `SILENCE`  
    #[serde(alias = "FAME")]
    Fame,
    /// Sphere: `FATE`
    /// - Precluded Spheres: `LUCK`  
    #[serde(alias = "FATE")]
    Fate,
    /// Sphere: `FERTILITY`
    /// - Friend Spheres: `AGRICULTURE`, `FOOD`, `RAIN`
    /// - Precluded Spheres: `BLIGHT`  
    #[serde(alias = "FERTILITY")]
    Fertility,
    /// Sphere: `FESTIVALS`
    /// - Friend Spheres: `DANCE`, `MUSIC`, `REVELRY`, `SONG`
    /// - Precluded Spheres: `MISERY`  
    #[serde(alias = "FESTIVALS")]
    Festivals,
    /// Sphere: `FIRE`
    /// - Friend Spheres: `METALS`, `SUN`, `VOLCANOS`
    /// - Precluded Spheres: `WATER`, `OCEANS`, `LAKES`, `RIVERS`  
    #[serde(alias = "FIRE")]
    Fire,
    /// Sphere: `FISH`
    /// - Friend Spheres: `OCEANS`, `LAKES`, `RIVERS`, `WATER`
    /// - Parent/Child Spheres: `NATURE`, `ANIMALS`
    #[serde(alias = "FISH")]
    Fish,
    /// Sphere: `FISHING`
    /// - Friend Spheres: `FISH`, `HUNTING`
    #[serde(alias = "FISHING")]
    Fishing,
    /// Sphere: `FOOD`
    /// - Friend Spheres: `AGRICULTURE`, `FERTILITY`
    /// - Precluded Spheres: `BLIGHT`  
    #[serde(alias = "FOOD")]
    Food,
    /// Sphere: `FORGIVENESS`
    /// - Friend Spheres: `MERCY`
    /// - Precluded Spheres: `REVENGE`  
    #[serde(alias = "FORGIVENESS")]
    Forgiveness,
    /// Sphere: `FORTRESSES`
    /// - Friend Spheres: `WAR`
    #[serde(alias = "FORTRESSES")]
    Fortresses,
    /// Sphere: `FREEDOM`
    /// - Precluded Spheres: `ORDER`  
    #[serde(alias = "FREEDOM")]
    Freedom,
    /// Sphere: `GAMBLING`
    /// - Friend Spheres: `GAMES`, `LUCK`
    #[serde(alias = "GAMBLING")]
    Gambling,
    /// Sphere: `GAMES`
    /// - Friend Spheres: `GAMBLING`, `LUCK`
    #[serde(alias = "GAMES")]
    Games,
    /// Sphere: `GENEROSITY`
    /// - Friend Spheres: `CHARITY`, `SACRIFICE`
    #[serde(alias = "GENEROSITY")]
    Generosity,
    /// Sphere: `HAPPINESS`
    /// - Friend Spheres: `REVELRY`
    /// - Precluded Spheres: `MISERY`  
    #[serde(alias = "HAPPINESS")]
    Happiness,
    /// Sphere: `HEALING`
    /// - Precluded Spheres: `DISEASE`, `BLIGHT`, `DEATH`  
    #[serde(alias = "HEALING")]
    Healing,
    /// Sphere: `HOSPITALITY`
    #[serde(alias = "HOSPITALITY")]
    Hospitality,
    /// Sphere: `HUNTING`
    /// - Friend Spheres: `FISHING`
    #[serde(alias = "HUNTING")]
    Hunting,
    /// Sphere: `INSPIRATION`
    /// - Friend Spheres: `ART`, `PAINTING`, `POETRY`
    #[serde(alias = "INSPIRATION")]
    Inspiration,
    /// Sphere: `JEALOUSY`
    /// - Precluded Spheres: `CHARITY`  
    #[serde(alias = "JEALOUSY")]
    Jealousy,
    /// Sphere: `JEWELS`
    /// - Friend Spheres: `MINERALS`, `WEALTH`
    #[serde(alias = "JEWELS")]
    Jewels,
    /// Sphere: `JUSTICE`
    /// - Friend Spheres: `LAWS`
    #[serde(alias = "JUSTICE")]
    Justice,
    /// Sphere: `LABOR`
    /// - Friend Spheres: `CRAFTS`
    #[serde(alias = "LABOR")]
    Labor,
    /// Sphere: `LAKES`
    /// - Friend Spheres: `COASTS`, `FISH`, `OCEANS`, `RIVERS`
    /// - Parent/Child Spheres: `WATER`
    /// - Precluded Spheres: `FIRE`  
    #[serde(alias = "LAKES")]
    Lakes,
    /// Sphere: `LAWS`
    /// - Friend Spheres: `DISCIPLINE`, `JUSTICE`, `OATHS`, `ORDER`
    /// - Precluded Spheres: `CHAOS`, `DEPRAVITY`, `MURDER`, `THEFT`  
    #[serde(alias = "LAWS")]
    Laws,
    /// Sphere: `LIES`
    /// - Friend Spheres: `TREACHERY`, `TRICKERY`
    /// - Precluded Spheres: `TRUTH`  
    #[serde(alias = "LIES")]
    Lies,
    /// Sphere: `LIGHT`
    /// - Friend Spheres: `DAY`, `RAINBOWS`, `SUN`
    /// - Precluded Spheres: `DARKNESS`, `TWILIGHT`  
    #[serde(alias = "LIGHT")]
    Light,
    /// Sphere: `LIGHTNING`
    /// - Friend Spheres: `RAIN`, `STORMS`, `THUNDER`
    /// - Parent/Child Spheres: `WEATHER`  
    #[serde(alias = "LIGHTNING")]
    Lightning,
    /// Sphere: `LONGEVITY`
    /// - Friend Spheres: `YOUTH`
    /// - Precluded Spheres: `DEATH`  
    #[serde(alias = "LONGEVITY")]
    Longevity,
    /// Sphere: `LOVE`
    #[serde(alias = "LOVE")]
    Love,
    /// Sphere: `LOYALTY`
    /// - Friend Spheres: `OATHS`
    /// - Precluded Spheres: `TREACHERY`  
    #[serde(alias = "LOYALTY")]
    Loyalty,
    /// Sphere: `LUCK`
    /// - Friend Spheres: `GAMBLING`, `GAMES`
    /// - Precluded Spheres: `FATE`  
    #[serde(alias = "LUCK")]
    Luck,
    /// Sphere: `LUST`
    /// - Friend Spheres: `DEPRAVITY`
    #[serde(alias = "LUST")]
    Lust,
    /// Sphere: `MARRIAGE`
    /// - Friend Spheres: `BIRTH`, `FAMILY`, `OATHS`, `PREGNANCY`
    #[serde(alias = "MARRIAGE")]
    Marriage,
    /// Sphere: `MERCY`
    /// - Friend Spheres: `FORGIVENESS`
    /// - Precluded Spheres: `REVENGE`  
    #[serde(alias = "MERCY")]
    Mercy,
    /// Sphere: `METALS`
    /// - Friend Spheres: `CRAFTS`, `FIRE`, `MINERALS`
    /// - Parent/Child Spheres: `EARTH`
    #[serde(alias = "METALS")]
    Metals,
    /// Sphere: `MINERALS`
    /// - Friend Spheres: `JEWELS`, `METALS`
    /// - Parent/Child Spheres: `EARTH`
    #[serde(alias = "MINERALS")]
    Minerals,
    /// Sphere: `MISERY`
    /// - Friend Spheres: `TORTURE`
    /// - Precluded Spheres: `CONSOLATION`, `FESTIVALS`, `REVELRY`, `HAPPINESS`  
    #[serde(alias = "MISERY")]
    Misery,
    /// Sphere: `MIST`
    #[serde(alias = "MIST")]
    Mist,
    /// Sphere: `MOON`
    /// - Friend Spheres: `NIGHT`, `SKY`
    #[serde(alias = "MOON")]
    Moon,
    /// Sphere: `MOUNTAINS`
    /// - Friend Spheres: `CAVERNS`, `EARTH`, `VOLCANOS`
    #[serde(alias = "MOUNTAINS")]
    Mountains,
    /// Sphere: `MUCK`
    /// - Precluded Spheres: `BEAUTY`  
    #[serde(alias = "MUCK")]
    Muck,
    /// Sphere: `MURDER`
    /// - Friend Spheres: `DEATH`
    /// - Precluded Spheres: `LAWS`  
    #[serde(alias = "MURDER")]
    Murder,
    /// Sphere: `MUSIC`
    /// - Friend Spheres: `DANCE`, `FESTIVALS`, `REVELRY`, `SONG`
    /// - Parent/Child Spheres: `ART`
    /// - Precluded Spheres: `SILENCE`  
    #[serde(alias = "MUSIC")]
    Music,
    /// Sphere: `NATURE`
    /// - Friend Spheres: `RAIN`, `SUN`, `WATER`, `WEATHER`
    /// - Parent/Child Spheres: `ANIMALS`, `FISH`, `PLANTS`, `TREES`
    #[serde(alias = "NATURE")]
    Nature,
    /// Sphere: `NIGHT`
    /// - Friend Spheres: `DARKNESS`, `DREAMS`, `MOON`, `NIGHTMARES`, `STARS`
    /// - Precluded Spheres: `DAY`, `DAWN`, `DUSK`, `TWILIGHT`  
    #[serde(alias = "NIGHT")]
    Night,
    /// Sphere: `NIGHTMARES`
    /// - Friend Spheres: `DREAMS`, `NIGHT`
    /// - Precluded Spheres: `DAY`  
    #[serde(alias = "NIGHTMARES")]
    Nightmares,
    /// Sphere: `OATHS`
    /// - Friend Spheres: `LAWS`, `LOYALTY`, `MARRIAGE`
    /// - Precluded Spheres: `TREACHERY`  
    #[serde(alias = "OATHS")]
    Oaths,
    /// Sphere: `OCEANS`
    /// - Friend Spheres: `COASTS`, `FISH`, `LAKES`, `RIVERS`, `SALT`
    /// - Parent/Child Spheres: `WATER`
    /// - Precluded Spheres: `FIRE`  
    #[serde(alias = "OCEANS")]
    Oceans,
    /// Sphere: `ORDER`
    /// - Friend Spheres: `DISCIPLINE`, `DUTY`, `LAWS`
    /// - Precluded Spheres: `CHAOS`, `FREEDOM`  
    #[serde(alias = "ORDER")]
    Order,
    /// Sphere: `PAINTING`
    /// - Friend Spheres: `INSPIRATION`
    /// - Parent/Child Spheres: `ART`
    #[serde(alias = "PAINTING")]
    Painting,
    /// Sphere: `PEACE`
    #[serde(alias = "PEACE")]
    Peace,
    /// Sphere: `PERSUASION`
    /// - Friend Spheres: `POETRY`, `SPEECH`
    #[serde(alias = "PERSUASION")]
    Persuasion,
    /// Sphere: `PLANTS`
    /// - Friend Spheres: `ANIMALS`, `RAIN`
    /// - Parent/Child Spheres: `NATURE`
    #[serde(alias = "PLANTS")]
    Plants,
    /// Sphere: `POETRY`
    /// - Friend Spheres: `INSPIRATION`, `PERSUASION`, `SONG`, `WRITING`
    /// - Parent/Child Spheres: `ART`
    #[serde(alias = "POETRY")]
    Poetry,
    /// Sphere: `PREGNANCY`
    /// - Friend Spheres: `BIRTH`, `CHILDREN`, `CREATION`, `FAMILY`, `MARRIAGE`
    #[serde(alias = "PREGNANCY")]
    Pregnancy,
    /// Sphere: `RAIN`
    /// - Friend Spheres: `AGRICULTURE`, `FERTILITY`, `LIGHTNING`, `NATURE`, `PLANTS`, `RAINBOWS`, `STORMS`, `THUNDER`, `TREES`
    /// - Parent/Child Spheres: `WATER`, `WEATHER`
    #[serde(alias = "RAIN")]
    Rain,
    /// Sphere: `RAINBOWS`
    /// - Friend Spheres: `LIGHT`, `RAIN`, `SKY`
    /// - Parent/Child Spheres: `WEATHER`
    #[serde(alias = "RAINBOWS")]
    Rainbows,
    /// Sphere: `REBIRTH`
    /// - Friend Spheres: `BIRTH`, `CREATION`, `DEATH`
    #[serde(alias = "REBIRTH")]
    Rebirth,
    /// Sphere: `REVELRY`
    /// - Friend Spheres: `DANCE`, `FESTIVALS`, `HAPPINESS`, `MUSIC`, `SONG`
    /// - Precluded Spheres: `MISERY`  
    #[serde(alias = "REVELRY")]
    Revelry,
    /// Sphere: `REVENGE`
    /// - Precluded Spheres: `FORGIVENESS`, `MERCY`  
    #[serde(alias = "REVENGE")]
    Revenge,
    /// Sphere: `RIVERS`
    /// - Friend Spheres: `FISH`, `LAKES`, `OCEANS`
    /// - Parent/Child Spheres: `WATER`
    /// - Precluded Spheres: `FIRE`  
    #[serde(alias = "RIVERS")]
    Rivers,
    /// Sphere: `RULERSHIP`
    #[serde(alias = "RULERSHIP")]
    Rulership,
    /// Sphere: `RUMORS`
    /// - Friend Spheres: `FAME`
    #[serde(alias = "RUMORS")]
    Rumors,
    /// Sphere: `SACRIFICE`
    /// - Friend Spheres: `CHARITY`, `GENEROSITY`
    /// - Precluded Spheres: `WEALTH`  
    #[serde(alias = "SACRIFICE")]
    Sacrifice,
    /// Sphere: `SALT`
    /// - Friend Spheres: `OCEANS`
    /// - Parent/Child Spheres: `EARTH`
    #[serde(alias = "SALT")]
    Salt,
    /// Sphere: `SCHOLARSHIP`
    /// - Friend Spheres: `WISDOM`, `WRITING`
    #[serde(alias = "SCHOLARSHIP")]
    Scholarship,
    /// Sphere: `SEASONS`
    #[serde(alias = "SEASONS")]
    Seasons,
    /// Sphere: `SILENCE`
    /// - Precluded Spheres: `FAME`, `MUSIC`  
    #[serde(alias = "SILENCE")]
    Silence,
    /// Sphere: `SKY`
    /// - Friend Spheres: `MOON`, `RAINBOWS`, `SUN`, `STARS`, `WEATHER`, `WIND`
    #[serde(alias = "SKY")]
    Sky,
    /// Sphere: `SONG`
    /// - Friend Spheres: `FESTIVALS`, `MUSIC`, `POETRY`, `REVELRY`
    /// - Parent/Child Spheres: `ART`
    #[serde(alias = "SONG")]
    Song,
    /// Sphere: `SPEECH`
    /// - Friend Spheres: `PERSUASION`
    #[serde(alias = "SPEECH")]
    Speech,
    /// Sphere: `STARS`
    /// - Friend Spheres: `NIGHT`, `SKY`
    #[serde(alias = "STARS")]
    Stars,
    /// Sphere: `STORMS`
    /// - Friend Spheres: `LIGHTNING`, `RAIN`, `THUNDER`
    /// - Parent/Child Spheres: `WEATHER`
    #[serde(alias = "STORMS")]
    Storms,
    /// Sphere: `STRENGTH`
    #[serde(alias = "STRENGTH")]
    Strength,
    /// Sphere: `SUICIDE`
    /// - Friend Spheres: `DEATH`
    #[serde(alias = "SUICIDE")]
    Suicide,
    /// Sphere: `SUN`
    /// - Friend Spheres: `DAWN`, `DAY`, `FIRE`, `LIGHT`, `NATURE`, `SKY`
    /// - Precluded Spheres: `DARKNESS`  
    #[serde(alias = "SUN")]
    Sun,
    /// Sphere: `THEFT`
    /// - Precluded Spheres: `LAWS`, `TRADE`  
    #[serde(alias = "THEFT")]
    Theft,
    /// Sphere: `THRALLDOM`
    #[serde(alias = "THRALLDOM")]
    Thralldom,
    /// Sphere: `THUNDER`
    /// - Friend Spheres: `LIGHTNING`, `RAIN`, `STORMS`
    /// - Parent/Child Spheres: `WEATHER`
    #[serde(alias = "THUNDER")]
    Thunder,
    /// Sphere: `TORTURE`
    /// - Friend Spheres: `MISERY`
    #[serde(alias = "TORTURE")]
    Torture,
    /// Sphere: `TRADE`
    /// - Friend Spheres: `WEALTH`
    /// - Precluded Spheres: `THEFT`  
    #[serde(alias = "TRADE")]
    Trade,
    /// Sphere: `TRAVELERS`
    #[serde(alias = "TRAVELERS")]
    Travelers,
    /// Sphere: `TREACHERY`
    /// - Friend Spheres: `LIES`, `TRICKERY`
    /// - Precluded Spheres: `LOYALTY`, `OATHS`  
    #[serde(alias = "TREACHERY")]
    Treachery,
    /// Sphere: `TREES`
    /// - Friend Spheres: `RAIN`
    /// - Parent/Child Spheres: `NATURE`, `PLANTS`
    #[serde(alias = "TREES")]
    Trees,
    /// Sphere: `TRICKERY`
    /// - Friend Spheres: `LIES`, `TREACHERY`
    /// - Precluded Spheres: `TRUTH`  
    #[serde(alias = "TRICKERY")]
    Trickery,
    /// Sphere: `TRUTH`
    /// - Precluded Spheres: `LIES`, `TRICKERY`  
    #[serde(alias = "TRUTH")]
    Truth,
    /// Sphere: `TWILIGHT`
    /// - Friend Spheres: `DAWN`, `DUSK`
    /// - Precluded Spheres: `LIGHT`, `DARKNESS`, `DAY`, `NIGHT`  
    #[serde(alias = "TWILIGHT")]
    Twilight,
    /// Sphere: `VALOR`
    /// - Friend Spheres: `WAR`
    /// - Parent/Child Spheres: `COURAGE`
    #[serde(alias = "VALOR")]
    Valor,
    /// Sphere: `VICTORY`
    /// - Friend Spheres: `WAR`
    #[serde(alias = "VICTORY")]
    Victory,
    /// Sphere: `VOLCANOS`
    /// - Friend Spheres: `EARTH`, `FIRE`, `MOUNTAINS`
    #[serde(alias = "VOLCANOS")]
    Volcanos,
    /// Sphere: `WAR`
    /// - Friend Spheres: `CHAOS`, `DEATH`, `FORTRESSES`, `VALOR`, `VICTORY`
    #[serde(alias = "WAR")]
    War,
    /// Sphere: `WATER`
    /// - Friend Spheres: `FISH`, `NATURE`
    /// - Parent/Child Spheres: `LAKES`, `OCEANS`, `RIVERS`, `RAIN`
    /// - Precluded Spheres: `FIRE`  
    #[serde(alias = "WATER")]
    Water,
    /// Sphere: `WEALTH`
    /// - Friend Spheres: `JEWELS`, `TRADE`
    /// - Precluded Spheres: `SACRIFICE`  
    #[serde(alias = "WEALTH")]
    Wealth,
    /// Sphere: `WEATHER`
    /// - Friend Spheres: `NATURE`, `SKY`
    /// - Parent/Child Spheres: `LIGHTNING`, `RAIN`, `RAINBOWS`, `STORMS`, `THUNDER`, `WIND`
    #[serde(alias = "WEATHER")]
    Weather,
    /// Sphere: `WIND`
    /// - Friend Spheres: `SKY`
    /// - Parent/Child Spheres: `WEATHER`
    #[serde(alias = "WIND")]
    Wind,
    /// Sphere: `WISDOM`
    /// - Friend Spheres: `SCHOLARSHIP`
    #[serde(alias = "WISDOM")]
    Wisdom,
    /// Sphere: `WRITING`
    /// - Friend Spheres: `POETRY`, `SCHOLARSHIP`
    #[serde(alias = "WRITING")]
    Writing,
    /// Sphere: `YOUTH`
    /// - Friend Spheres: `BIRTH`, `CHILDREN`, `LONGEVITY`
    /// - Precluded Spheres: `DEATH`  
    #[serde(alias = "YOUTH")]
    Youth,
}

impl Default for SphereEnum {
    fn default() -> Self {
        Self::Agriculture
    }
}
