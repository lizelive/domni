use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BodyAttributeEnum {
    /// Alters the damage done in melee (increases velocity of weapon swings), increases muscle mass
    /// (thicker muscle layer also resists damage more), and increases how much a creature can
    /// carry. Higher strength also increases the speed of fast gaits in most creatures, including
    /// running, soaring, galloping, and fast swimming, however proportionate gains in muscle mass
    /// can eventually reduce speed enough to overtake the gains (moreso with larger creatures than
    /// dwarves.). The dwarf median of 1250 is higher than average.
    /// - 2250–5000: unbelievably strong
    /// - 2000–2249: mighty
    /// - 1750–1999: very strong
    /// - 1500–1749: strong
    /// - 1001–1499: (no description)
    /// - 751–1000: weak
    /// - 501–750: very weak
    /// - 251–500: unquestionably weak
    /// - 0–250: unfathomably weak
    #[serde(alias = "STRENGTH")]
    Strength,
    /// This attribute affects the speed of a creature's fast-moving gaits in the same way as
    /// strength — a creature with maximum agility and strength can run faster than a creature with
    /// minimum agility and strength. The median dwarf agility of 900 is slightly below average.
    ///
    /// Because dwarves in general have below-average agility, and because the descriptions are
    /// based on the difference from the racial average, no dwarves will ever display as "abysmally
    /// clumsy".
    /// - 1900–5000: amazingly agile
    /// - 1650–1899: extremely agile
    /// - 1400–1649: very agile
    /// - 1150–1399: agile
    /// - 651–1149: (no description)
    /// - 401–650: clumsy
    /// - 151–400: quite clumsy
    /// - 0–150: totally clumsy
    /// - (never displayed): abysmally clumsy
    #[serde(alias = "AGILITY")]
    Agility,
    /// Reduces physical damage of all kinds, including bleeding and suffocation. The dwarf median
    /// of 1250 is higher than average.
    /// - 2250–5000: basically unbreakable
    /// - 2000–2249: incredibly tough
    /// - 1750–1999: quite durable
    /// - 1500–1749: tough
    /// - 1001–1499: (no description)
    /// - 751–1000: flimsy
    /// - 501–750: very flimsy
    /// - 251–500: remarkably flimsy
    /// - 0–250: shockingly fragile
    #[serde(alias = "TOUGHNESS")]
    Toughness,
    /// Reduces the rate at which dwarves become exhausted. The dwarf median endurance of 1000 is
    /// exactly the human average.
    /// - 2000–5000: absolutely inexhaustible
    /// - 1750–1999: indefatigable
    /// - 1500–1749: very slow to tire
    /// - 1250–1499: slow to tire
    /// - 751–1249: (no description)
    /// - 501–750: quick to tire
    /// - 251–500: very quick to tire
    /// - 1–250: extremely quick to tire
    /// - 0: truly quick to tire
    #[serde(alias = "ENDURANCE")]
    Endurance,
    /// Increases the rate of wound healing and reduces fat. The dwarf median recuperation of 1000
    /// is exactly the human average.
    /// - 2000–5000: possessed of amazing recuperative powers
    /// - 1750–1999: incredibly quick to heal
    /// - 1500–1749: quite quick to heal
    /// - 1250–1499: quick to heal
    /// - 751–1249: (no description)
    /// - 501–750: slow to heal
    /// - 251–500: very slow to heal
    /// - 1–250: really slow to heal
    /// - 0: shockingly slow to heal
    #[serde(alias = "RECUPERATION")]
    Recuperation,
    /// Reduces the risk of contracting syndromes and their negative effects when active, including
    /// the alcohol-induced ones. The dwarf median disease resistance of 1000 is exactly the human
    /// average.
    /// - 2000–5000: virtually never sick
    /// - 1750–1999: almost never sick
    /// - 1500–1749: very rarely sick
    /// - 1250–1499: rarely sick
    /// - 751–1249: (no description)
    /// - 501–750: susceptible to disease
    /// - 251–500: quite susceptible to disease
    /// - 1–250: really susceptible to disease
    /// - 0: stunningly susceptible to disease
    #[serde(alias = "DISEASE_RESISTANCE")]
    DiseaseResistance,
}
impl Default for BodyAttributeEnum {
    fn default() -> Self {
        Self::Strength
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SoulAttributeEnum {
    /// The dwarf median analytical ability of 1250 is slightly above average.
    /// - 2250–5000: awesome intellectual powers
    /// - 2000–2249: great analytical abilities
    /// - 1750–1999: a sharp intellect
    /// - 1500–1749: a good intellect
    /// - 1001–1499: (no description)
    /// - 751–1000: poor analytical abilities
    /// - 501–750: very bad analytical abilities
    /// - 251–500: a lousy intellect
    /// - 0–250: a stunning lack of analytical ability
    #[serde(alias = "ANALYTICAL_ABILITY")]
    AnalyticalAbility,
    /// The dwarf median focus of 1500 is above average.
    /// - 2542–5000: unbreakable focus
    /// - 2292–2541: a great ability to focus
    /// - 2042–2291: very good focus
    /// - 1792–2041: the ability to focus
    /// - 1293–1791: (no description)
    /// - 1043–1292: poor focus
    /// - 793–1042: quite poor focus
    /// - 543–792: really poor focus
    /// - 0–542: the absolute inability to focus
    #[serde(alias = "FOCUS")]
    Focus,
    /// Willpower directly reduces exertion and pain effects, and also affects the threshold before
    /// creatures flee in battle and the effects of stress. The dwarf median willpower of 1000 is
    /// exactly the human average.
    /// - 2000–5000: an unbreakable will
    /// - 1750–1999: an iron will
    /// - 1500–1749: a lot of willpower
    /// - 1250–1499: willpower
    /// - 751–1249: (no description)
    /// - 501–750: little willpower
    /// - 251–500: a large deficit of willpower
    /// - 1–250: next to no willpower
    /// - 0: absolutely no willpower
    #[serde(alias = "WILLPOWER")]
    Willpower,
    /// The dwarf median creativity of 1250 is slightly above the human average.
    /// - 2250–5000: a boundless creative imagination
    /// - 2000–2249: great creativity
    /// - 1751–1999: very good creativity
    /// - 1500–1750: good creativity
    /// - 1001–1499: (no description)
    /// - 751–1000: meager creativity
    /// - 501–750: poor creativity
    /// - 251–500: lousy creativity
    /// - 0–250: next to no creative talent
    #[serde(alias = "CREATIVITY")]
    Creativity,
    /// The dwarf median intuition of 1000 is exactly the human average.
    /// - 2000–5000: uncanny intuition
    /// - 1750–1999: great intuition
    /// - 1500–1749: very good intuition
    /// - 1250–1499: good intuition
    /// - 751–1249: (no description)
    /// - 501–750: bad intuition
    /// - 251–500: very bad intuition
    /// - 1–250: lousy intuition
    /// - 0: horrible intuition
    #[serde(alias = "INTUITION")]
    Intuition,
    /// Some non-skill tasks are affected by Patience. The dwarf median patience of 1250 is slightly
    /// above the human average.
    /// - 2250–5000: absolutely boundless patience
    /// - 2000–2249: a deep well of patience
    /// - 1750–1999: a great deal of patience
    /// - 1500–1749: a sum of patience
    /// - 1001–1499: (no description)
    /// - 751–1000: a shortage of patience
    /// - 501–750: little patience
    /// - 251–500: very little patience
    /// - 0–250: no patience at all
    #[serde(alias = "PATIENCE")]
    Patience,
    /// The dwarf median memory of 1250 is slightly above the human average.
    /// - 2250–5000: an astonishing memory
    /// - 2000–2249: an amazing memory
    /// - 1750–1999: a great memory
    /// - 1500–1749: a good memory
    /// - 1001–1499: (no description)
    /// - 751–1000: an iffy memory
    /// - 501–750: a poor memory
    /// - 251–500: a really bad memory
    /// - 0–250: little memory to speak of
    #[serde(alias = "MEMORY")]
    Memory,
    /// The dwarf median linguistic ability of 1000 is exactly the human average.
    /// - 2000–5000: an astonishing ability with languages and words
    /// - 1750–1999: a great affinity for language
    /// - 1500–1749: a natural inclination toward language
    /// - 1250–1499: a way with words
    /// - 751–1249: (no description)
    /// - 501–750: a little difficulty with words
    /// - 251–500: little linguistic ability
    /// - 1–250: very little linguistic ability
    /// - 0: difficulty with words and language
    #[serde(alias = "LINGUISTIC_ABILITY")]
    LinguisticAbility,
    /// The dwarf median spatial sense of 1500 is above the human average.
    /// - 2542–5000: a stunning feel for spatial relationships
    /// - 2292–2541: an amazing spatial sense
    /// - 2042–2291: a great feel for the surrounding space
    /// - 1792–2041: a good spatial sense
    /// - 1293–1791: (no description)
    /// - 1043–1292: a questionable spatial sense
    /// - 793–1042: poor spatial senses
    /// - 543–792: an atrocious spatial sense
    /// - 0–542: no sense for spatial relationships
    #[serde(alias = "SPATIAL_SENSE")]
    SpatialSense,
    /// Musicality affects performances such as playing music with instruments and singing. The
    /// dwarf median musicality of 1000 is exactly the human average.
    /// - 2000–5000: an astonishing knack for music
    /// - 1750–1999: a great musical sense
    /// - 1500–1749: a natural ability with music
    /// - 1250–1499: a feel for music
    /// - 751–1249: (no description)
    /// - 501–750: an iffy sense for music
    /// - 251–500: little natural inclination toward music
    /// - 1–250: next to no natural musical ability
    /// - 0: absolutely no feel for music at all
    #[serde(alias = "MUSICALITY")]
    Musicality,
    /// Most skills involving any movement at all (lots of them), and many non-skilled tasks as well
    /// are affected by Kinesthetic Sense. The dwarf median kinesthetic sense of 1000 is exactly the
    /// human average.
    /// - 2000–5000: an astounding feel for the position of `[his/her]` own body
    /// - 1750–1999: a great kinesthetic sense
    /// - 1500–1749: a very good sense of the position of `[his/her]` own body
    /// - 1250–1499: a good kinesthetic sense
    /// - 751–1249: (no description)
    /// - 501–750: a meager kinesthetic sense
    /// - 251–500: a poor kinesthetic sense
    /// - 1–250: a very clumsy kinesthetic sense
    /// - 0: an unbelievably atrocious sense of the position of `[his/her]` own body
    #[serde(alias = "KINESTHETIC_SENSE")]
    KinestheticSense,
    /// The dwarf median empathy of 1000 is exactly the human average.
    /// - 2000–5000: an absolutely remarkable sense of others' emotions
    /// - 1750–1999: a great sense of empathy
    /// - 1500–1749: a very good sense of empathy
    /// - 1250–1499: an ability to read emotions fairly well
    /// - 751–1249: (no description)
    /// - 501–750: poor empathy
    /// - 251–500: a very bad sense of empathy
    /// - 1–250: next to no empathy
    /// - 0: the utter inability to judge others' emotions
    #[serde(alias = "EMPATHY")]
    Empathy,
    /// The dwarf median social awareness of 1000 is exactly the human average.
    /// - 2000–5000: a shockingly profound feel for social relationships
    /// - 1750–1999: a great feel for social relationships
    /// - 1500–1749: a very good feel for social relationships
    /// - 1250–1499: a good feel for social relationships
    /// - 751–1249: (no description)
    /// - 501–750: a meager ability with social relationships
    /// - 251–500: a poor ability to manage or understand social relationships
    /// - 1–250: a lack of understanding of social relationships
    /// - 0: an absolute inability to understand social relationships
    #[serde(alias = "SOCIAL_AWARENESS")]
    SocialAwareness,
}
impl Default for SoulAttributeEnum {
    fn default() -> Self {
        Self::AnalyticalAbility
    }
}
