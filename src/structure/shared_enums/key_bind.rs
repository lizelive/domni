use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

/// For more key binds see: issue #76
pub enum KeyBindEnum {
    #[serde(alias = "HOTKEY_STILL_BREW")]
    HotkeyStillBrew,
    #[serde(alias = "HOTKEY_KITCHEN_RENDER_FAT")]
    HotkeyKitchenRenderFat,
    #[serde(alias = "CUSTOM_A")]
    CustomA,
    #[serde(alias = "CUSTOM_B")]
    CustomB,
    #[serde(alias = "CUSTOM_C")]
    CustomC,
    #[serde(alias = "CUSTOM_D")]
    CustomD,
    #[serde(alias = "CUSTOM_E")]
    CustomE,
    #[serde(alias = "CUSTOM_F")]
    CustomF,
    #[serde(alias = "CUSTOM_G")]
    CustomG,
    #[serde(alias = "CUSTOM_H")]
    CustomH,
    #[serde(alias = "CUSTOM_I")]
    CustomI,
    #[serde(alias = "CUSTOM_J")]
    CustomJ,
    #[serde(alias = "CUSTOM_K")]
    CustomK,
    #[serde(alias = "CUSTOM_L")]
    CustomL,
    #[serde(alias = "CUSTOM_M")]
    CustomM,
    #[serde(alias = "CUSTOM_N")]
    CustomN,
    #[serde(alias = "CUSTOM_O")]
    CustomO,
    #[serde(alias = "CUSTOM_P")]
    CustomP,
    #[serde(alias = "CUSTOM_Q")]
    CustomQ,
    #[serde(alias = "CUSTOM_R")]
    CustomR,
    #[serde(alias = "CUSTOM_S")]
    CustomS,
    #[serde(alias = "CUSTOM_T")]
    CustomT,
    #[serde(alias = "CUSTOM_U")]
    CustomU,
    #[serde(alias = "CUSTOM_V")]
    CustomV,
    #[serde(alias = "CUSTOM_W")]
    CustomW,
    #[serde(alias = "CUSTOM_X")]
    CustomX,
    #[serde(alias = "CUSTOM_Y")]
    CustomY,
    #[serde(alias = "CUSTOM_Z")]
    CustomZ,
    #[serde(alias = "CUSTOM_SHIFT_A")]
    CustomShiftA,
    #[serde(alias = "CUSTOM_SHIFT_B")]
    CustomShiftB,
    #[serde(alias = "CUSTOM_SHIFT_C")]
    CustomShiftC,
    #[serde(alias = "CUSTOM_SHIFT_D")]
    CustomShiftD,
    #[serde(alias = "CUSTOM_SHIFT_E")]
    CustomShiftE,
    #[serde(alias = "CUSTOM_SHIFT_F")]
    CustomShiftF,
    #[serde(alias = "CUSTOM_SHIFT_G")]
    CustomShiftG,
    #[serde(alias = "CUSTOM_SHIFT_H")]
    CustomShiftH,
    #[serde(alias = "CUSTOM_SHIFT_I")]
    CustomShiftI,
    #[serde(alias = "CUSTOM_SHIFT_J")]
    CustomShiftJ,
    #[serde(alias = "CUSTOM_SHIFT_K")]
    CustomShiftK,
    #[serde(alias = "CUSTOM_SHIFT_L")]
    CustomShiftL,
    #[serde(alias = "CUSTOM_SHIFT_M")]
    CustomShiftM,
    #[serde(alias = "CUSTOM_SHIFT_N")]
    CustomShiftN,
    #[serde(alias = "CUSTOM_SHIFT_O")]
    CustomShiftO,
    #[serde(alias = "CUSTOM_SHIFT_P")]
    CustomShiftP,
    #[serde(alias = "CUSTOM_SHIFT_Q")]
    CustomShiftQ,
    #[serde(alias = "CUSTOM_SHIFT_R")]
    CustomShiftR,
    #[serde(alias = "CUSTOM_SHIFT_S")]
    CustomShiftS,
    #[serde(alias = "CUSTOM_SHIFT_T")]
    CustomShiftT,
    #[serde(alias = "CUSTOM_SHIFT_U")]
    CustomShiftU,
    #[serde(alias = "CUSTOM_SHIFT_V")]
    CustomShiftV,
    #[serde(alias = "CUSTOM_SHIFT_W")]
    CustomShiftW,
    #[serde(alias = "CUSTOM_SHIFT_X")]
    CustomShiftX,
    #[serde(alias = "CUSTOM_SHIFT_Y")]
    CustomShiftY,
    #[serde(alias = "CUSTOM_SHIFT_Z")]
    CustomShiftZ,
    #[serde(alias = "CUSTOM_CTRL_A")]
    CustomCtrlA,
    #[serde(alias = "CUSTOM_CTRL_B")]
    CustomCtrlB,
    #[serde(alias = "CUSTOM_CTRL_C")]
    CustomCtrlC,
    #[serde(alias = "CUSTOM_CTRL_D")]
    CustomCtrlD,
    #[serde(alias = "CUSTOM_CTRL_E")]
    CustomCtrlE,
    #[serde(alias = "CUSTOM_CTRL_F")]
    CustomCtrlF,
    #[serde(alias = "CUSTOM_CTRL_G")]
    CustomCtrlG,
    #[serde(alias = "CUSTOM_CTRL_H")]
    CustomCtrlH,
    #[serde(alias = "CUSTOM_CTRL_I")]
    CustomCtrlI,
    #[serde(alias = "CUSTOM_CTRL_J")]
    CustomCtrlJ,
    #[serde(alias = "CUSTOM_CTRL_K")]
    CustomCtrlK,
    #[serde(alias = "CUSTOM_CTRL_L")]
    CustomCtrlL,
    #[serde(alias = "CUSTOM_CTRL_M")]
    CustomCtrlM,
    #[serde(alias = "CUSTOM_CTRL_N")]
    CustomCtrlN,
    #[serde(alias = "CUSTOM_CTRL_O")]
    CustomCtrlO,
    #[serde(alias = "CUSTOM_CTRL_P")]
    CustomCtrlP,
    #[serde(alias = "CUSTOM_CTRL_Q")]
    CustomCtrlQ,
    #[serde(alias = "CUSTOM_CTRL_R")]
    CustomCtrlR,
    #[serde(alias = "CUSTOM_CTRL_S")]
    CustomCtrlS,
    #[serde(alias = "CUSTOM_CTRL_T")]
    CustomCtrlT,
    #[serde(alias = "CUSTOM_CTRL_U")]
    CustomCtrlU,
    #[serde(alias = "CUSTOM_CTRL_V")]
    CustomCtrlV,
    #[serde(alias = "CUSTOM_CTRL_W")]
    CustomCtrlW,
    #[serde(alias = "CUSTOM_CTRL_X")]
    CustomCtrlX,
    #[serde(alias = "CUSTOM_CTRL_Y")]
    CustomCtrlY,
    #[serde(alias = "CUSTOM_CTRL_Z")]
    CustomCtrlZ,
    #[serde(alias = "CUSTOM_ALT_A")]
    CustomAltA,
    #[serde(alias = "CUSTOM_ALT_B")]
    CustomAltB,
    #[serde(alias = "CUSTOM_ALT_C")]
    CustomAltC,
    #[serde(alias = "CUSTOM_ALT_D")]
    CustomAltD,
    #[serde(alias = "CUSTOM_ALT_E")]
    CustomAltE,
    #[serde(alias = "CUSTOM_ALT_F")]
    CustomAltF,
    #[serde(alias = "CUSTOM_ALT_G")]
    CustomAltG,
    #[serde(alias = "CUSTOM_ALT_H")]
    CustomAltH,
    #[serde(alias = "CUSTOM_ALT_I")]
    CustomAltI,
    #[serde(alias = "CUSTOM_ALT_J")]
    CustomAltJ,
    #[serde(alias = "CUSTOM_ALT_K")]
    CustomAltK,
    #[serde(alias = "CUSTOM_ALT_L")]
    CustomAltL,
    #[serde(alias = "CUSTOM_ALT_M")]
    CustomAltM,
    #[serde(alias = "CUSTOM_ALT_N")]
    CustomAltN,
    #[serde(alias = "CUSTOM_ALT_O")]
    CustomAltO,
    #[serde(alias = "CUSTOM_ALT_P")]
    CustomAltP,
    #[serde(alias = "CUSTOM_ALT_Q")]
    CustomAltQ,
    #[serde(alias = "CUSTOM_ALT_R")]
    CustomAltR,
    #[serde(alias = "CUSTOM_ALT_S")]
    CustomAltS,
    #[serde(alias = "CUSTOM_ALT_T")]
    CustomAltT,
    #[serde(alias = "CUSTOM_ALT_U")]
    CustomAltU,
    #[serde(alias = "CUSTOM_ALT_V")]
    CustomAltV,
    #[serde(alias = "CUSTOM_ALT_W")]
    CustomAltW,
    #[serde(alias = "CUSTOM_ALT_X")]
    CustomAltX,
    #[serde(alias = "CUSTOM_ALT_Y")]
    CustomAltY,
    #[serde(alias = "CUSTOM_ALT_Z")]
    CustomAltZ,
}

impl Default for KeyBindEnum {
    fn default() -> Self {
        Self::CustomA
    }
}
