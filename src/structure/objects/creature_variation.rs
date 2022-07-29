use crate::core::{AllowEmpty, Any, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

// TODO: for all of these, support `!ARG` tokens, up to `!ARG65537` at least
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreatureVariationToken {
    /// Argument 1 of `[CREATURE_VARIATION:...]`
    #[serde(alias = "CREATURE_VARIATION")]
    pub reference: Option<ReferenceTo<Self>>,
    // TODO: properly implement the below tokens; CV_NEW_TAG, CV_REMOVE_TAG, CV_NEW_CTAG, CV_REMOVE_CTAG:
    // --------------------------------------------------------------------------------------------
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Adds the given token to the creature.
    ///
    /// `APPLY_CREATURE_VARIATION` is the only token that cannot be added.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CV_NEW_TAG:PETVALUE:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:200]`, and `[PETVALUE:200]` will be
    /// added to the creature.
    ///
    /// You can replace any amount of the token with `!ARG` placeholders; for instance, you could
    /// do `[CV_NEW_TAG:NAME:!ARG1 bear:!ARG1 bears:!ARG1 bear]`, and replace `!ARG1` with `grizzly`,
    /// and then the token `[NAME:grizzly bear:grizzly bear:grizzly bears]` would be added to the
    /// creature, or you could use `[CV_NEW_TAG:NAME:!ARG1]`, and by using
    /// `[APPLY_CREATURE_VARIATION:EXAMPLE_VARIATION:dwarf|dwarves|dwarven]`, you would end up with
    /// `[NAME:dwarf:dwarves:dwarven]`. You can even use an `!ARG` for the tag itself, like this
    /// `[CV_NEW_TAG:!ARG1]`.
    #[serde(alias = "CV_NEW_TAG", alias = "CV_ADD_TAG")]
    pub cv_new_tag: Vec<(Reference, Option<(Vec<Any>,)>)>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Removes from the creature all tokens which start with the given chain of token arguments.
    ///
    /// For example, `[CV_REMOVE_TAG:BODY:HUMANOID_SIMPLE]` would remove
    /// `[BODY:HUMANOID_SIMPLE:3_EYES]`, but `[CV_REMOVE_TAG:BODY:3_EYES]`, or `[CV_REMOVE_TAG:3_EYES]`
    /// would not. Neither would `[CV_REMOVE_TAG:BODY:HUMANOID_SIMPLE]` be able to remove
    /// `[BODY:HUMANOID_SIMPLE_NECK]`, as it looks for whole arguments.
    ///
    /// `[CV_REMOVE_TAG:BODY]` would remove both examples above, as they both start with `BODY`.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CV_REMOVE_TAG:PETVALUE:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:200]`, and `[PETVALUE:200]` will be
    /// removed from the creature.
    ///
    /// You can replace any amount of the token with `!ARG` placeholders; for instance, you could
    /// do `[CV_REMOVE_TAG:BODY:!ARG1:!ARG2]`, and use
    /// `[APPLY_CREATURE_VARIATION:EXAMPLE_REMOVAL:HUMANOID_SIMPLE:BRAIN]` to only remove `BODY`
    /// tokens that start with `HUMANOID_SIMPLE:BRAIN`.
    #[serde(alias = "CV_REMOVE_TAG")]
    pub cv_remove_tag: Vec<(Reference, Option<(Vec<Any>,)>)>,
    /// List of `[CV_CONVERT_TAG]` tokens.
    #[serde(alias = "CV_CONVERT_TAG")]
    pub cv_convert_tag: Vec<CvConvertTag>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Adds the given token to the creature, but only when the specified numbered
    /// argument of `APPLY_CREATURE_VARIATION has a specfic value.
    ///
    /// For example, `[CV_REMOVE_CTAG:2:HUMANOID:CAN_SPEAK]` will only add `[CAN_SPEAK]`
    /// if the second argument used in `APPLY_CREATURE_VARIATION` (also known as `!ARG2`)
    /// is `HUMANOID`.
    ///
    /// `APPLY_CREATURE_VARIATION` is the only token that cannot be added.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CV_NEW_TAG:PETVALUE:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:200]`, and `[PETVALUE:200]` will be
    /// added to the creature.
    ///
    /// You can replace any amount of the token with `!ARG` placeholders; for instance, you could
    /// do `[CV_NEW_TAG:NAME:!ARG1 bear:!ARG1 bears:!ARG1 bear]`, and replace `!ARG1` with `grizzly`,
    /// and then the token `[NAME:grizzly bear:grizzly bears:grizzly bear]` would be added to the
    /// creature. Or you could use `[CV_NEW_TAG:NAME:!ARG1]`, and by using
    /// `[APPLY_CREATURE_VARIATION:EXAMPLE_VARIATION:dwarf|dwarves|dwarven]`, you would end up with
    /// `[NAME:dwarf:dwarves:dwarven]`. You can even use an `!ARG` for the tag itself, like this
    /// `[CV_NEW_TAG:!ARG1]`.
    #[serde(alias = "CV_NEW_CTAG", alias = "CV_ADD_CTAG")]
    pub cv_new_ctag: Vec<(u16, Reference, Option<(Vec<Any>,)>)>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Removes from the creature all tokens which start with the given chain of token arguments.
    ///
    /// For example, `[CV_REMOVE_TAG:BODY:HUMANOID_SIMPLE]` would remove
    /// `[BODY:HUMANOID_SIMPLE:3_EYES]`, but `[CV_REMOVE_TAG:BODY:3_EYES]`, or `[CV_REMOVE_TAG:3_EYES]`
    /// would not. Neither would `[CV_REMOVE_TAG:BODY:HUMANOID_SIMPLE]` be able to remove
    /// `[BODY:HUMANOID_SIMPLE_NECK]`, as it looks for whole arguments.
    ///
    /// `[CV_REMOVE_TAG:BODY]` would remove both examples above, as they both start with `BODY`.
    ///
    /// Unlike `CV_REMOVE_TAG` however, the token will only be removed when the specified numbered
    /// argument of `APPLY_CREATURE_VARIATION has a specfic value.
    ///
    /// For example, `[CV_REMOVE_CTAG:2:HUMANOID:PET_EXOTIC]` will only remove `[PET_EXOTIC]`
    /// if the second argument used in `APPLY_CREATURE_VARIATION` (also known as `!ARG2`)
    /// is `HUMANOID`.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CV_REMOVE_TAG:PETVALUE:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:200]`, and `[PETVALUE:200]` will be
    /// removed from the creature.
    ///
    /// You can replace any amount of the token with `!ARG` placeholders; for instance, you could
    /// do `[CV_REMOVE_TAG:BODY:!ARG1:!ARG2]`, and use
    /// `[APPLY_CREATURE_VARIATION:EXAMPLE_REMOVAL:HUMANOID_SIMPLE:BRAIN]` to only remove `BODY`
    /// tokens that start with `HUMANOID_SIMPLE:BRAIN`.
    #[serde(alias = "CV_REMOVE_CTAG")]
    pub cv_remove_ctag: Vec<(u16, Reference, Option<(Vec<Any>,)>)>,
    /// List of `[CV_CONVERT_CTAG]` tokens.
    #[serde(alias = "CV_CONVERT_CTAG")]
    pub cv_convert_ctag: Vec<CvConvertCTag>,
}

/// Starts a "conversion block" to modify the arguments of existing tokens on a creature.
/// A conversion block contains one `CVCT_MASTER`, one `CVCT_TARGET`, and one `CVCT_REPLACEMENT`
/// (note, `CVCT_REPLACEMENT` is optional, and leaving it out may be used to erase parts of
/// arguments instead of replacing them).
///
/// Note that if a creature variation contains multiple `CV_CONVERT_TAG` (or `CV_CONVERT_CTAG`)
/// blocks altering the same token, the replacements will be applied in reverse order
/// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
/// for more detail).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CvConvertTag {
    /// `CV_CONVERT_TAG`; has no arguments.
    #[serde(alias = "CV_CONVERT_TAG")]
    pub cv_convert_tag: Option<()>,
    // TODO: properly implement the 3 below tokens:
    // --------------------------------------------------------------------------------------------
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Specifies which tokens of the creature may be modified, by looking at full arguments
    /// at the start of said token.
    ///
    /// For example, `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` would target
    /// `[BODY:HUMANOID_SIMPLE:3_EYES]`, but `[CVCT_MASTER:BODY:3_EYES]`, or `[CVCT_MASTER:3_EYES]`
    /// would not. Neither would `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` be able to target
    /// `[BODY:HUMANOID_SIMPLE_NECK]`, as it looks for whole arguments.
    ///
    /// `[CVCT_MASTER:BODY]` would target both examples above, as they both start with `BODY`.
    ///
    /// If no `CVCT_MASTER` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_MASTER:]`), *all* tokens of
    /// the creature are selected.
    ///
    /// Note that if a creature variation contains multiple `CV_CONVERT_TAG` (or `CV_CONVERT_CTAG`)
    /// blocks altering the same token, the replacements will be applied in reverse order
    /// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
    /// for more detail).
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CVCT_MASTER:BODY:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:HUMANOID_SIMPLE]`, and this would
    /// cause the token to function as `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]`.
    #[serde(alias = "CVCT_MASTER")]
    pub cvct_master: Option<
        AllowEmpty<(
            Reference, // TODO: ref can be name of any token that nests under CREATURE
            Option<(Vec<Any>,)>,
        )>,
    >,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Locates the specified parameters or portions of parameters, within all tokens specified
    /// by `CVCT_MASTER`.
    ///
    /// For example, this conversion block:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:STATE_NAME]
    ///     [CVCT_TARGET:adder]
    /// ```
    /// Will select every instance of `adder` in all arguments of all `STATE_NAME` tokens
    /// in that creature, and *only* the portion saying `adder`:
    ///
    /// ```df_raw
    /// [STATE_NAME:ALL_SOLID:frozen adder venom]
    /// [STATE_NAME:LIQUID:adder venom]
    /// ```
    /// You may target references and integers as well as strings, though be warned that integers
    /// cannot be targeted exactly; for instance, `[CVCT_TARGET:1]` will select the `1` in `10`
    /// or `101` as well.
    ///
    /// You can target the token itself, for instance, you could replace the `[PET]` token
    /// with `[PET_EXOTIC]`:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:PET]
    ///     [CVCT_TARGET:PET]
    ///     [CVCT_REPLACEMENT:PET_EXOTIC]
    /// ```
    /// Be warned however that it has not been tested whether subsequent arguments are deleted by this,
    /// and they most likely are not, so this is likely only safe to use for tokens with no arguments
    /// (like my example above).
    ///
    /// If no `CVCT_TARGET` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_TARGET:]`), the game will freeze
    /// when loading the creature.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CVCT_TARGET:!ARG1 monster]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:gila]`, and this would
    /// cause the token to function as `[CVCT_TARGET:gila monster]`.
    #[serde(alias = "CVCT_TARGET")]
    pub cvct_target: Option<(Vec<Any>,)>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Replaces the string specified by `CVCT_TARGET` within the tokens specified by `CVCT_MASTER`.
    /// This means the targeted part of a token can be changed anywhere in the token, e.g:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:2EYES]
    ///     [CVCT_REPLACEMENT:2EYESTALKS]
    /// ```
    /// Would affect both of these:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYES]
    /// [BODY:INSECT:2EYES:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Converting them into:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYESTALKS]
    /// [BODY:INSECT:2EYESTALKS:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Colons can be included as part of both the target and the replacement string, for example:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:BASIC_1PARTBODY:BASIC_HEAD]
    ///     [CVCT_REPLACEMENT:HUMANOID:3FINGERS]
    /// ```
    /// Will convert `[BODY:BASIC_1PARTBODY:BASIC_HEAD:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`, into
    /// `[BODY:HUMANOID:3FINGERS:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`. All occurrences of the target
    ///  string are replaced, for example:
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:DESCRIPTION]
    ///     [CVCT_TARGET:TRAIT]
    ///     [CVCT_REPLACEMENT:modderiffic]
    /// ```
    /// Will convert `[DESCRIPTION:This is an example creature. It is TRAIT, very very TRAIT.]`,
    /// into `[DESCRIPTION:This is an example creature. It is modderiffic, very very modderiffic.]`.
    ///
    /// If no `CVCT_REPLACEMENT` is given, the target string is simply removed.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you used:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:DESCRIPTION]
    ///     [CVCT_TARGET:TRAIT]
    ///     [CVCT_REPLACEMENT:!ARG1]
    /// ```
    /// Then the creature variation can be used like so:
    /// `[APPLY_CREATURE_VARIATION:VARIATION_ID:mod-tastic]`, and this would allow you to change
    /// the above example into `[DESCRIPTION:This is an example creature. It is mod-tastic, very very mod-tastic.]`.
    #[serde(alias = "CVCT_REPLACEMENT")]
    pub cvct_replacement: Option<(Vec<Any>,)>,
}

/// Starts a "conversion block" to modify the arguments of existing tokens on a creature, but
/// unlike `CV_CONVERT_TAG`, the block will only apply a change when the specified numbered argument
/// of `APPLY_CREATURE_VARIATION has a specfic value.
///
/// For example, `[CV_CONVERT_CTAG:2:HUMANOID]` will only add apply the effects of the following
/// conversion block if the second argument used in `APPLY_CREATURE_VARIATION` (also known as
/// `!ARG2`) is `HUMANOID`.
///
/// A conversion block contains one `CVCT_MASTER`, one `CVCT_TARGET`, and one `CVCT_REPLACEMENT`
/// (note, `CVCT_REPLACEMENT` is optional, and leaving it out may be used to erase parts of
/// arguments instead of replacing them).
///
/// Note that if a creature variation contains multiple `CV_CONVERT_TAG` (or `CV_CONVERT_CTAG`)
/// blocks altering the same token, the replacements will be applied in reverse order
/// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
/// for more detail).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CvConvertCTag {
    /// Arguments of `[CV_CONVERT_CTAG:...]`
    #[serde(alias = "CV_CONVERT_CTAG")]
    pub cv_convert_ctag: Option<(u16, Any)>,
    // TODO: properly implement the 3 below tokens:
    // --------------------------------------------------------------------------------------------
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Specifies which tokens of the creature may be modified, by looking at full arguments
    /// at the start of said token.
    ///
    /// For example, `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` would target
    /// `[BODY:HUMANOID_SIMPLE:3_EYES]`, but `[CVCT_MASTER:BODY:3_EYES]`, or `[CVCT_MASTER:3_EYES]`
    /// would not. Neither would `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` be able to target
    /// `[BODY:HUMANOID_SIMPLE_NECK]`, as it looks for whole arguments.
    ///
    /// `[CVCT_MASTER:BODY]` would target both examples above, as they both start with `BODY`.
    ///
    /// If no `CVCT_MASTER` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_MASTER:]`), *all* tokens of
    /// the creature are selected.
    ///
    /// Note that if a creature variation contains multiple `CV_CONVERT_TAG` (or `CV_CONVERT_CTAG`)
    /// blocks altering the same token, the replacements will be applied in reverse order
    /// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
    /// for more detail).
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CVCT_MASTER:BODY:!ARG1]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:HUMANOID_SIMPLE]`, and this would
    /// cause the token to function as `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]`.
    #[serde(alias = "CVCT_MASTER")]
    pub cvct_master: Option<
        AllowEmpty<(
            Reference, // TODO: ref can be name of any token that nests under CREATURE
            Option<(Vec<Any>,)>,
        )>,
    >,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Locates the specified parameters or portions of parameters, within all tokens specified
    /// by `CVCT_MASTER`.
    ///
    /// For example, this conversion block:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:STATE_NAME]
    ///     [CVCT_TARGET:adder]
    /// ```
    /// Will select every instance of `adder` in all arguments of all `STATE_NAME` tokens
    /// in that creature, and *only* the portion saying `adder`:
    ///
    /// ```df_raw
    /// [STATE_NAME:ALL_SOLID:frozen adder venom]
    /// [STATE_NAME:LIQUID:adder venom]
    /// ```
    /// You may target references and integers as well as strings, though be warned that integers
    /// cannot be targeted exactly; for instance, `[CVCT_TARGET:1]` will select the `1` in `10`
    /// or `101` as well.
    ///
    /// You can target the token itself, for instance, you could replace the `[PET]` token
    /// with `[PET_EXOTIC]`:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:PET]
    ///     [CVCT_TARGET:PET]
    ///     [CVCT_REPLACEMENT:PET_EXOTIC]
    /// ```
    /// Be warned however that it has not been tested whether subsequent arguments are deleted by this,
    /// and they most likely are not, so this is likely only safe to use for tokens with no arguments
    /// (like my example above).
    ///
    /// If no `CVCT_TARGET` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_TARGET:]`), the game will freeze
    /// when loading the creature.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you use `[CVCT_TARGET:!ARG1 monster]`, then the creature variation can be
    /// used like so: `[APPLY_CREATURE_VARIATION:VARIATION_ID:gila]`, and this would
    /// cause the token to function as `[CVCT_TARGET:gila monster]`.
    #[serde(alias = "CVCT_TARGET")]
    pub cvct_target: Option<(Vec<Any>,)>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Replaces the string specified by `CVCT_TARGET` within the tokens specified by `CVCT_MASTER`.
    /// This means the targeted part of a token can be changed anywhere in the token, e.g:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:2EYES]
    ///     [CVCT_REPLACEMENT:2EYESTALKS]
    /// ```
    /// Would affect both of these:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYES]
    /// [BODY:INSECT:2EYES:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Converting them into:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYESTALKS]
    /// [BODY:INSECT:2EYESTALKS:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Colons can be included as part of both the target and the replacement string, for example:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:BASIC_1PARTBODY:BASIC_HEAD]
    ///     [CVCT_REPLACEMENT:HUMANOID:3FINGERS]
    /// ```
    /// Will convert `[BODY:BASIC_1PARTBODY:BASIC_HEAD:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`, into
    /// `[BODY:HUMANOID:3FINGERS:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`. All occurrences of the target
    ///  string are replaced, for example:
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:DESCRIPTION]
    ///     [CVCT_TARGET:TRAIT]
    ///     [CVCT_REPLACEMENT:modderiffic]
    /// ```
    /// Will convert `[DESCRIPTION:This is an example creature. It is TRAIT, very very TRAIT.]`,
    /// into `[DESCRIPTION:This is an example creature. It is modderiffic, very very modderiffic.]`.
    ///
    /// If no `CVCT_REPLACEMENT` is given, the target string is simply removed.
    ///
    /// You can replace any portion of the arguments of this token with `!ARGn`, with `n` being
    /// a number (eg, `!ARG1`, `!ARG2` etc etc). When this creature variation is applied (using
    /// `APPLY_CREATURE_VARIATION`), if any arguments other than the specified `CREATURE_VARIATION`
    /// ID are given, these extra arguments will replace all instances of `!ARGn`, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any `!ARG1`, the second any `!ARG2` and so on.
    ///
    /// For instance, if you used:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:DESCRIPTION]
    ///     [CVCT_TARGET:TRAIT]
    ///     [CVCT_REPLACEMENT:!ARG1]
    /// ```
    /// Then the creature variation can be used like so:
    /// `[APPLY_CREATURE_VARIATION:VARIATION_ID:mod-tastic]`, and this would allow you to change
    /// the above example into `[DESCRIPTION:This is an example creature. It is mod-tastic, very very mod-tastic.]`.
    #[serde(alias = "CVCT_REPLACEMENT")]
    pub cvct_replacement: Option<(Vec<Any>,)>,
}
