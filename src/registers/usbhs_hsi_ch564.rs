use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "UsbhsHsi",
        extends: None,
        description: Some("USBHS HSI calibration register."),
        items: &[BlockItem {
            name: "cal_ctrl",
            description: Some("HSI clock calibration control register."),
            array: None,
            byte_offset: 0x28,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("CalCtrl"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "CalCtrl",
        extends: None,
        description: Some("HSI clock calibration control register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "cal_rst",
                description: Some("the calibration module is globally reset."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "cal_en",
                description: Some("alignment enabled."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "sft_rst",
                description: Some(
                    "exti fine calibration mode and receive that BUS_RESET. should be set.",
                ),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "sof_free",
                description: Some("the uplink interface enters the enable mode."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "clk_sel",
                description: Some("clock selection."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
