use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PersonalityTraitEnum {
    /// - 91-100: is always in love with somebody and easily develops positive feelings
    /// - 76-90: very easily falls into love and develops positive feelings
    /// - 61-75: can easily fall in love or develop positive sentiments
    /// - 40-60: (no description)
    /// - 25-39: does not easily fall in love and rarely develops positive sentiments
    /// - 10-24: is not the type to fall in love or even develop positive feelings
    /// - 0-9: never falls in love or develops positive feelings toward anything
    #[serde(alias = "LOVE_PROPENSITY")]
    LovePropensity,
    /// - 91-100: is often inflamed by hatred and easily develops hatred toward things
    /// - 76-90: is prone to hatreds and often develops negative feelings
    /// - 61-75: is quick to form negative views about things
    /// - 40-60: (no description)
    /// - 25-39: does not easily hate or develop negative feelings
    /// - 10-24: very rarely develops negative feelings toward things
    /// - 0-9: never feels hatred toward anyone or anything
    #[serde(alias = "HATE_PROPENSITY")]
    HatePropensity,
    /// - 91-100: is consumed by overpowering feelings of jealousy
    /// - 76-90: is prone to strong feelings of jealousy
    /// - 61-75: often feels envious of others
    /// - 40-60: (no description)
    /// - 25-39: doesn't often feel envious of others
    /// - 10-24: is rarely jealous
    /// - 0-9: never envies others their status, situation or possessions
    #[serde(alias = "ENVY_PROPENSITY")]
    EnvyPropensity,
    /// - 91-100: often feels filled with joy
    /// - 76-90: can be very happy and optimistic
    /// - 61-75: is often cheerful
    /// - 40-60: (no description)
    /// - 25-39: is rarely happy or enthusiastic
    /// - 10-24: is dour as a rule
    /// - 0-9: is never the slightest bit cheerful about anything
    #[serde(alias = "CHEER_PROPENSITY")]
    CheerPropensity,
    /// - 91-100: is frequently depressed
    /// - 76-90: is often sad and dejected
    /// - 61-75: often feels discouraged
    /// - 40-60: (no description)
    /// - 25-39: rarely feels discouraged
    /// - 10-24: almost never feels discouraged
    /// - 0-9: never feels discouraged
    #[serde(alias = "DEPRESSION_PROPENSITY")]
    DepressionPropensity,
    /// - 91-100: is in a constant state of internal rage
    /// - 76-90: is very quick to anger
    /// - 61-75: is quick to anger
    /// - 40-60: (no description)
    /// - 25-39: is slow to anger
    /// - 10-24: is very slow to anger
    /// - 0-9: never becomes angry
    #[serde(alias = "ANGER_PROPENSITY")]
    AngerPropensity,
    /// - 91-100: is a nervous wreck
    /// - 76-90: is always tense and jittery
    /// - 61-75: is often nervous
    /// - 40-60: (no description)
    /// - 25-39: has a calm demeanor
    /// - 10-24: has a very calm demeanor
    /// - 0-9: has an incredibly calm demeanor
    #[serde(alias = "ANXIETY_PROPENSITY")]
    AnxietyPropensity,
    /// - 91-100: is constantly ablaze with feelings of lust
    /// - 76-90: is prone to strong feelings of lust
    /// - 61-75: often feels lustful
    /// - 40-60: (no description)
    /// - 25-39: does not often feel lustful
    /// - 10-24: rarely looks on others with lust
    /// - 0-9: never feels lustful passions
    #[serde(alias = "LUST_PROPENSITY")]
    LustPropensity,
    /// - 91-100: becomes completely helpless in stressful situations
    /// - 76-90: cracks easily under pressure
    /// - 61-75: doesn't handle stress well
    /// - 40-60: (no description)
    /// - 25-39: can handle stress
    /// - 10-24: is confident under pressure
    /// - 0-9: is impervious to the effects of stress
    #[serde(alias = "STRESS_VULNERABILITY")]
    StressVulnerability,
    /// - 91-100: is as avaricious as they come, obsessed with acquiring wealth
    /// - 76-90: is very greedy
    /// - 61-75: has a greedy streak
    /// - 40-60: (no description)
    /// - 25-39: doesn't focus on material goods
    /// - 10-24: desires little for [him/her]self in the way of possessions
    /// - 0-9: often neglects `[his/her]` own wellbeing, having no interest in material goods
    #[serde(alias = "GREED")]
    Greed,
    /// - 91-100: is ruled by irresistible cravings and urges
    /// - 76-90: feels strong urges and seeks short-term rewards
    /// - 61-75: occasionally overindulges
    /// - 40-60: (no description)
    /// - 25-39: doesn't often experience strong cravings or urges
    /// - 10-24: only rarely feels strong cravings or urges
    /// - 0-9: never feels tempted to overindulge in anything
    #[serde(alias = "IMMODERATION")]
    Immoderation,
    /// - 91-100: is given to rough-and-tumble brawling, even to the point of starting fights for no
    /// reason
    /// - 76-90: would never pass up a chance for a good fistfight
    /// - 61-75: likes to brawl
    /// - 40-60: (no description)
    /// - 25-39: tends to avoid any physical confrontations
    /// - 10-24: does not enjoy participating in physical confrontations
    /// - 0-9: would flee even the most necessary battle to avoid any form of physical
    /// confrontation
    #[serde(alias = "VIOLENT")]
    Violent,
    /// - 91-100: is unbelievably stubborn, and will stick with even the most futile action once
    /// `[his/her]` mind is made up
    /// - 76-90: is very stubborn
    /// - 61-75: is stubborn
    /// - 40-60: (no description)
    /// - 25-39: has a noticeable lack of perseverance
    /// - 10-24: doesn't stick with things if even minor difficulties arise
    /// - 0-9: drops any activity at the slightest hint of difficulty or even the suggestion of
    /// effort being required
    #[serde(alias = "PERSEVERANCE")]
    Perseverance,
    /// - 91-100: is completely careless with resources when completing projects, and invariably
    /// wastes a lot of time and effort
    /// - 76-90: is not careful with resources when working on projects and often spends unnecessary
    /// effort
    /// - 61-75: tends to be a little wasteful when working on projects
    /// - 40-60: (no description)
    /// - 25-39: tends to be a little tight with resources when working on projects
    /// - 10-24: is stingy with resources on projects and refuses to expend any extra effort
    /// - 0-9: cuts any corners possible when working on a project, regardless of the consequences,
    /// rather than wasting effort or resources
    #[serde(alias = "WASTEFULNESS")]
    Wastefulness,
    /// - 91-100: revels in chaos and discord, and `[he/she]` encourages it whenever possible
    /// - 76-90: finds a chaotic mess preferable to the boredom of harmonious living
    /// - 61-75: doesn't mind a little tumult and discord in day-to-day living
    /// - 40-60: (no description)
    /// - 25-39: prefers that everyone live as harmoniously as possible
    /// - 10-24: feels best when everyone gets along without any strife or contention
    /// - 0-9: would be deeply satisfied if everyone could live as one in complete harmony
    #[serde(alias = "DISCORD")]
    Discord,
    /// - 91-100: is quite a bold flatterer, extremely friendly but just a little insufferable
    /// - 76-90: is very friendly and always tries to say nice things to others
    /// - 61-75: is a friendly individual
    /// - 40-60: (no description)
    /// - 25-39: is somewhat quarrelsome
    /// - 10-24: is unfriendly and disagreeable
    /// - 0-9: is a dyed-in-the-wool quarreler, never missing a chance to lash out in verbal
    /// hostility
    #[serde(alias = "FRIENDLINESS")]
    Friendliness,
    /// - 91-100: exhibits a refined politeness and is determined to keep the guiding rules of
    /// etiquette and decorum as if life itself depended on it
    /// - 76-90: is very polite and observes appropriate rules of decorum when possible
    /// - 61-75: is quite polite
    /// - 40-60: (no description)
    /// - 25-39: could be considered rude
    /// - 10-24: is very impolite and inconsiderate of propriety
    /// - 0-9: is a vulgar being who does not care a lick for even the most basic rules of
    /// civilized living
    #[serde(alias = "POLITENESS")]
    Politeness,
    /// - 91-100: disdains even the best advice of associates and family, relying strictly on
    /// `[his/her]` own counsel
    /// - 76-90: dislikes receiving advice, preferring to keep `[his/her]` own counsel
    /// - 61-75: has a tendency to go it alone, without considering the advice of others
    /// - 40-60: (no description)
    /// - 25-39: tends to ask others for help with difficult decisions
    /// - 10-24: relies on the advice of others during decision making
    /// - 0-9: is unable to make decisions without a great deal of input from others
    #[serde(alias = "DISDAIN_ADVICE")]
    DisdainAdvice,
    /// - 91-100: is utterly fearless when confronted with danger, to the point of lacking common
    /// sense
    /// - 76-90: is incredibly brave in the face of looming danger, perhaps a bit foolhardy
    /// - 61-75: is brave in the face of imminent danger
    /// - 40-60: (no description)
    /// - 25-39: is somewhat fearful in the face of imminent danger
    /// - 10-24: has great trouble mastering fear when confronted by danger
    /// - 0-9: is a coward, completely overwhelmed by fear when confronted with danger
    #[serde(alias = "BRAVERY")]
    Bravery,
    /// - 91-100: presupposes success in any venture requiring `[his/her]` skills with what could be
    /// called blind overconfidence
    /// - 76-90: is extremely confident of [him/her]self in situations requiring `[his/her]` skills
    /// - 61-75: is generally quite confident of `[his/her]` abilities when undertaking specific
    /// ventures
    /// - 40-60: (no description)
    /// - 25-39: sometimes acts with little determination and confidence
    /// - 10-24: lacks confidence in `[his/her]` abilities
    /// - 0-9: has no confidence at all in `[his/her]` talent and abilities
    #[serde(alias = "CONFIDENCE")]
    Confidence,
    /// - 91-100: is completely wrapped up in `[his/her]` own appearance, abilities and other personal
    /// matters
    /// - 76-90: is greatly pleased by `[his/her]` own looks and accomplishments
    /// - 61-75: is pleased by `[his/her]` own appearance and talents
    /// - 40-60: (no description)
    /// - 25-39: is not inherently proud of `[his/her]` talents and accomplishments
    /// - 10-24: takes no pleasure in `[his/her]` talents and appearance
    /// - 0-9: could not care less about `[his/her]` appearance, talents or other personal vanities
    #[serde(alias = "VANITY")]
    Vanity,
    /// - 91-100: has a relentless drive, completely consumed by ambition
    /// - 76-90: is very ambitious, always looking for a way to better `[his/her]` situation
    /// - 61-75: is quite ambitious
    /// - 40-60: (no description)
    /// - 25-39: isn't particularly ambitious
    /// - 10-24: is not driven and rarely feels the need to pursue even a modest success
    /// - 0-9: has no ambition whatsoever
    #[serde(alias = "AMBITION")]
    Ambition,
    /// - 91-100: unerringly returns favors and has a profound sense of gratitude for the kind actions
    /// of others
    /// - 76-90: feels a strong need to reciprocate any favor done for `[him/her]`
    /// - 61-75: is grateful when others help `[him/her]` out and tries to return favors
    /// - 40-60: (no description)
    /// - 25-39: takes offered help and gifts without feeling particularly grateful
    /// - 10-24: accepts favors without developing a sense of obligation, preferring to act as the
    /// current situation demands
    /// - 0-9: does not feel the slightest need to reciprocate favors that others do for `[him/her]`,
    /// no matter how major the help or how much `[he/she]` needed it
    #[serde(alias = "GRATITUDE")]
    Gratitude,
    /// - 91-100: always presents [him/her]self as extravagantly as possible, displaying a magnificent
    /// image to the world
    /// - 76-90: likes to present [him/her]self boldly, even if it would offend an average sense of
    /// modesty
    /// - 61-75: doesn't mind wearing something special now and again
    /// - 40-60: (no description)
    /// - 25-39: prefers to present [him/her]self modestly
    /// - 10-24: presents [him/her]self modestly and frowns on any flashy accoutrements
    /// - 0-9: cleaves to an austere lifestyle, disdaining even minor immodesties in appearance
    #[serde(alias = "IMMODESTY")]
    Immodesty,
    /// - 91-100: finds something humorous in everything, no matter how serious or inappropriate
    /// - 76-90: finds the humor in most situations
    /// - 61-75: has an active sense of humor
    /// - 40-60: (no description)
    /// - 25-39: has little interest in joking around
    /// - 10-24: does not find most jokes humorous
    /// - 0-9: is utterly humorless
    #[serde(alias = "HUMOR")]
    Humor,
    /// - 91-100: is vengeful and never forgets or forgives past grievances
    /// - 76-90: has little time for forgiveness and will generally seek retribution
    /// - 61-75: tends to hang on to grievances
    /// - 40-60: (no description)
    /// - 25-39: doesn't tend to hold on to grievances
    /// - 10-24: does not generally seek retribution for past wrongs
    /// - 0-9: has no sense of vengeance or retribution
    #[serde(alias = "VENGEFUL")]
    Vengeful,
    /// - 91-100: is absorbed in delusions of self-importance
    /// - 76-90: has an overinflated sense of self-worth
    /// - 61-75: thinks `[he/she]` is fairly important in the grand scheme of things
    /// - 40-60: (no description)
    /// - 25-39: is very humble
    /// - 10-24: has a low sense of self-esteem
    /// - 0-9: is completely convinced of `[his/her]` own worthlessness
    #[serde(alias = "PRIDE")]
    Pride,
    /// - 91-100: is deliberately cruel to those unfortunate enough to be subject to `[his/her]` sadism
    /// - 76-90: is sometimes cruel
    /// - 61-75: generally acts impartially and is rarely moved to mercy
    /// - 40-60: (no description)
    /// - 25-39: often acts with compassion
    /// - 10-24: is easily moved to mercy
    /// - 0-9: always acts with mercy and compassion at the forefront of `[his/her]` considerations
    #[serde(alias = "CRUELTY")]
    Cruelty,
    /// - 91-100: pursues matters with a single-minded focus, often overlooking other matters
    /// - 76-90: can be very single-minded
    /// - 61-75: generally acts with a narrow focus on the current activity
    /// - 40-60: (no description)
    /// - 25-39: can occasionally lose focus on the matter at hand
    /// - 10-24: is somewhat scatterbrained
    /// - 0-9: is a complete scatterbrain, unable to focus on a single matter for more than a
    /// passing moment
    #[serde(alias = "SINGLEMINDED")]
    Singleminded,
    /// - 91-100: has such a developed sense of optimism that `[he/she]` always assumes the best outcome
    /// will eventually occur, no matter what
    /// - 76-90: is an optimist
    /// - 61-75: generally finds [him/her]self quite hopeful about the future
    /// - 40-60: (no description)
    /// - 25-39: tends to assume the worst of two outcomes will be the one that comes to pass
    /// - 10-24: is a pessimist
    /// - 0-9: despairs of anything positive happening in the future and lives without feelings of
    /// hope
    #[serde(alias = "HOPEFUL")]
    Hopeful,
    /// - 91-100: is implacably curious, without any respect for propriety or privacy
    /// - 76-90: is very curious, sometimes to `[his/her]` detriment
    /// - 61-75: is curious and eager to learn
    /// - 40-60: (no description)
    /// - 25-39: isn't particularly curious about the world
    /// - 10-24: is very rarely moved by curiosity
    /// - 0-9: is incurious and never seeks out knowledge or information to satisfy [him/her]self
    #[serde(alias = "CURIOUS")]
    Curious,
    /// - 91-100: is gripped by a crippling shyness
    /// - 76-90: is bashful
    /// - 61-75: tends to consider what others think of `[him/her]`
    /// - 40-60: (no description)
    /// - 25-39: is not particularly interested in what others think of `[him/her]`
    /// - 10-24: is generally unhindered by the thoughts of others concerning `[his/her]` actions
    /// - 0-9: is shameless, absolutely unfazed by the thoughts of others
    #[serde(alias = "BASHFUL")]
    Bashful,
    /// - 91-100: is private to the point of paranoia, unwilling to reveal even basic information
    /// about [him/her]self
    /// - 76-90: has a strong tendency toward privacy
    /// - 61-75: tends not to reveal personal information
    /// - 40-60: (no description)
    /// - 25-39: tends to share `[his/her]` own experiences and thoughts with others
    /// - 10-24: is not a private person and freely shares details of `[his/her]` life
    /// - 0-9: shares intimate details of life without sparing a thought to repercussions or
    /// propriety
    #[serde(alias = "PRIVACY")]
    Privacy,
    /// - 91-100: is obsessed with details and will often take a great deal of extra time to make sure
    /// things are done the right way
    /// - 76-90: is a perfectionist
    /// - 61-75: tries to do things correctly each time
    /// - 40-60: (no description)
    /// - 25-39: doesn't try to get things done perfectly
    /// - 10-24: is inattentive to detail in `[his/her]` own work
    /// - 0-9: is frustratingly sloppy and careless with every task `[he/she]` sets to carry out
    #[serde(alias = "PERFECTIONIST")]
    Perfectionist,
    /// - 91-100: is completely closed-minded and never changes `[his/her]` mind after forming an
    /// initial idea
    /// - 76-90: is intellectually stubborn, rarely changing `[his/her]` mind during a debate
    /// regardless of the merits
    /// - 61-75: tends to be a bit stubborn in changing `[his/her]` mind about things
    /// - 40-60: (no description)
    /// - 25-39: doesn't cling tightly to ideas and is open to changing `[his/her]` mind
    /// - 10-24: often finds [him/her]self changing `[his/her]` mind to agree with somebody else
    /// - 0-9: easily changes `[his/her]` mind and will generally go with the prevailing view on
    /// anything
    #[serde(alias = "CLOSEMINDED")]
    Closeminded,
    /// - 91-100: is not bothered in the slightest by deviations from the norm or even extreme
    /// differences in lifestyle or appearance
    /// - 76-90: is very comfortable around others that are different from `[him/her]`
    /// - 61-75: is quite comfortable with others that have a different appearance or culture
    /// - 40-60: (no description)
    /// - 25-39: is somewhat uncomfortable around those that appear unusual or live differently from
    /// `[him/her]`
    /// - 10-24: is made deeply uncomfortable by differences in culture or appearance
    /// - 0-9: cannot tolerate differences in culture, lifestyle or appearance
    #[serde(alias = "TOLERANT")]
    Tolerant,
    /// - 91-100: is emotionally obsessive, forming life-long attachments even if they aren't
    /// reciprocated
    /// - 76-90: forms strong emotional bonds with others, at times to `[his/her]` detriment
    /// - 61-75: has a tendency toward forming deep emotional bonds with others
    /// - 40-60: (no description)
    /// - 25-39: tends to form only tenuous emotional bonds with others
    /// - 10-24: forms only fleeting and rare emotional bonds with others
    /// - 0-9: does not have feelings of emotional attachment and has never felt even a moment's
    /// connection with another being
    #[serde(alias = "EMOTIONALLY_OBSESSIVE")]
    EmotionallyObsessive,
    /// - 91-100: is buffeted by others' emotions and can't help but to respond to them
    /// - 76-90: is swayed by emotional appeals
    /// - 61-75: tends to be swayed by the emotions of others
    /// - 40-60: (no description)
    /// - 25-39: tends not to be swayed by emotional appeals
    /// - 10-24: does not generally respond to emotional appeals
    /// - 0-9: is never moved by the emotions of others
    #[serde(alias = "SWAYED_BY_EMOTIONS")]
    SwayedByEmotions,
    /// - 91-100: is truly fulfilled by assisting those in need
    /// - 76-90: finds helping others very emotionally rewarding
    /// - 61-75: finds helping others emotionally rewarding
    /// - 40-60: (no description)
    /// - 25-39: does not go out of `[his/her]` way to help others
    /// - 10-24: dislikes helping others
    /// - 0-9: feels helping others is an imposition on `[his/her]` time
    #[serde(alias = "ALTRUISM")]
    Altruism,
    /// - 91-100: has a profound sense of duty and obligation
    /// - 76-90: has a strong sense of duty
    /// - 61-75: has a sense of duty
    /// - 40-60: (no description)
    /// - 25-39: finds obligations confining
    /// - 10-24: dislikes obligations and will try to avoid being bound by them
    /// - 0-9: hates vows, obligations, promises and other binding elements that could restrict
    /// `[him/her]`
    #[serde(alias = "DUTIFULNESS")]
    Dutifulness,
    /// - 91-100: never deliberates before acting, to the point of being considered thoughtless
    /// - 76-90: doesn't generally think before acting
    /// - 61-75: can sometimes act without deliberation
    /// - 40-60: (no description)
    /// - 25-39: tends to think before acting
    /// - 10-24: can get caught up in internal deliberations when action is necessary
    /// - 0-9: never acts without prolonged deliberation, even to `[his/her]` own detriment and the
    /// harm of those around `[him/her]`
    #[serde(alias = "THOUGHTLESSNESS")]
    Thoughtlessness,
    /// - 91-100: is obsessed with order and structure in `[his/her]` own life, with everything kept in
    /// its proper place
    /// - 76-90: lives an orderly life, organized and neat
    /// - 61-75: tries to keep `[his/her]` things orderly
    /// - 40-60: (no description)
    /// - 25-39: tends to make a small mess with `[his/her]` own possessions
    /// - 10-24: is sloppy with `[his/her]` living space
    /// - 0-9: is completely oblivious to any conception of neatness and will just leave things
    /// strewn about without a care
    #[serde(alias = "ORDERLINESS")]
    Orderliness,
    /// - 91-100: is naturally trustful of everybody
    /// - 76-90: is very trusting
    /// - 61-75: is trusting
    /// - 40-60: (no description)
    /// - 25-39: is slow to trust others
    /// - 10-24: does not trust others
    /// - 0-9: sees others as selfish and conniving
    #[serde(alias = "TRUST")]
    Trust,
    /// - 91-100: truly treasures the company of others
    /// - 76-90: enjoys being in crowds
    /// - 61-75: enjoys the company of others
    /// - 40-60: (no description)
    /// - 25-39: tends to avoid crowds
    /// - 10-24: prefers to be alone
    /// - 0-9: considers spending time alone much more important than associating with others
    #[serde(alias = "GREGARIOUSNESS")]
    Gregariousness,
    /// - 91-100: is assertive to the point of aggression, unwilling to let others get a word in
    /// edgewise when `[he/she]` has something to say
    /// - 76-90: has an overbearing personality
    /// - 61-75: is assertive
    /// - 40-60: (no description)
    /// - 25-39: tends to be passive in discussions
    /// - 10-24: only rarely tries to assert [him/her]self in conversation
    /// - 0-9: would never under any circumstances speak up or otherwise put forth `[his/her]` point
    /// of view in a discussion
    #[serde(alias = "ASSERTIVENESS")]
    Assertiveness,
    /// - 91-100: is driven by a bouncing frenetic energy
    /// - 76-90: lives at a high-energy kinetic pace
    /// - 61-75: lives a fast-paced life
    /// - 40-60: (no description)
    /// - 25-39: likes to take it easy
    /// - 10-24: lives at a slow-going and leisurely pace
    /// - 0-9: has an utterly languid pace of easy living, calm and slow
    #[serde(alias = "ACTIVITY_LEVEL")]
    ActivityLevel,
    /// - 91-100: never fails to seek out the most stressful and even dangerous situations
    /// - 76-90: seeks out exciting and adventurous situations
    /// - 61-75: likes a little excitement now and then
    /// - 40-60: (no description)
    /// - 25-39: doesn't seek out excitement
    /// - 10-24: actively avoids exciting or stressful situations
    /// - 0-9: does everything in `[his/her]` power to avoid excitement and stress
    #[serde(alias = "EXCITEMENT_SEEKING")]
    ExcitementSeeking,
    /// - 91-100: is bored by reality and would rather disappear utterly and forever into a world of
    /// made-up fantasy
    /// - 76-90: is given to flights of fancy to the point of distraction
    /// - 61-75: has an active imagination
    /// - 40-60: (no description)
    /// - 25-39: isn't given to flights of fancy
    /// - 10-24: is grounded in reality
    /// - 0-9: is interested only in facts and the real world
    #[serde(alias = "IMAGINATION")]
    Imagination,
    /// - 91-100: eschews practical concerns for philosophical discussion, puzzles, riddles and the
    /// world of ideas
    /// - 76-90: strongly prefers discussions of ideas and abstract concepts over handling specific
    /// practical issues
    /// - 61-75: has a tendency to consider ideas and abstractions over practical applications
    /// - 40-60: (no description)
    /// - 25-39: likes to keep things practical, without delving too deeply into the abstract
    /// - 10-24: dislikes abstract discussions and would much rather focus on practical examples
    /// - 0-9: is concerned only with matters practical to the situation at hand, with absolutely
    /// no inclination toward abstract discussion
    #[serde(alias = "ABSTRACT_INCLINED")]
    AbstractInclined,
    /// - 91-100: can easily become absorbed in art and the beauty of the natural world
    /// - 76-90: greatly moved by art and natural beauty
    /// - 61-75: is moved by art and natural beauty
    /// - 40-60: (no description)
    /// - 25-39: does not have a great aesthetic sensitivity
    /// - 10-24: is not readily moved by art or natural beauty
    /// - 0-9: is completely unmoved by art or the beauty of nature
    #[serde(alias = "ART_INCLINED")]
    ArtInclined,
}
impl Default for PersonalityTraitEnum {
    fn default() -> Self {
        Self::LovePropensity
    }
}
