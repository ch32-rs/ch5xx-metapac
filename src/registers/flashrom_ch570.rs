use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Flashrom",
        extends: None,
        description: Some("Serial flash command engine."),
        items: &[
            BlockItem {
                name: "data",
                description: Some("RW, 32-bit program data."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "data_byte",
                description: Some("RW, command, address and byte-program data port."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "sck",
                description: Some("RW, engine clock timing."),
                array: None,
                byte_offset: 0x5,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "ctrl",
                description: Some("RW, handshake control; bit 7 is busy."),
                array: None,
                byte_offset: 0x6,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cfg",
                description: Some("RW, engine access configuration."),
                array: None,
                byte_offset: 0x7,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 8,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[],
    enums: &[],
};
