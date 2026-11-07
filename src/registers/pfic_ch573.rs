
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pfic",
            extends: None,
            description: Some(
                "Program Fast Interrupt Controller.",
            ),
            items: &[
                BlockItem {
                    name: "isr1",
                    description: Some(
                        "RO,Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr2",
                    description: Some(
                        "RO,Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr1",
                    description: Some(
                        "RO,Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipr2",
                    description: Some(
                        "RO,Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ithresdr",
                    description: Some(
                        "RW,Interrupt Priority Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ithresdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fibaddrr",
                    description: Some(
                        "RW,Interrupt Fast Address Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fibaddrr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "Interrupt Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gisr",
                    description: Some(
                        "Interrupt Global Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr0",
                    description: Some(
                        "Interrupt 0 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr1",
                    description: Some(
                        "Interrupt 1 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr2",
                    description: Some(
                        "Interrupt 2 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fifoaddrr3",
                    description: Some(
                        "Interrupt 3 address Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fifoaddrr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr1",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ienr2",
                    description: Some(
                        "Interrupt Setting Register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ienr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer1",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "irer2",
                    description: Some(
                        "Interrupt Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Irer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr1",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipsr2",
                    description: Some(
                        "Interrupt Pending Register.",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ipsr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr1",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprr2",
                    description: Some(
                        "Interrupt Pending Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr1",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iactr2",
                    description: Some(
                        "Interrupt ACTIVE Register.",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iactr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior0",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior1",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior2",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x440,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior3",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x460,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior4",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x480,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior5",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x4a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior6",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior7",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x4e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior8",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior9",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x520,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior10",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x540,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior11",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x560,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior12",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x580,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior13",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x5a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior14",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x5c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior15",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x5e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior16",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior17",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x620,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior18",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x640,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior19",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x660,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior20",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x680,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior21",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x6a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior22",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior23",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x6e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior24",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior25",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x720,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior26",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x740,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior27",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x760,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior28",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x780,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior29",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x7a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior30",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x7c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior31",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x7e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior32",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x800,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior33",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x820,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior34",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x840,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior35",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x860,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior35",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior36",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x880,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior36",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior37",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x8a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior37",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior38",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x8c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior38",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior39",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x8e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior39",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior40",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x900,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior40",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior41",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x920,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior41",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior42",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x940,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior42",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior43",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x960,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior43",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior44",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x980,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior44",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior45",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x9a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior45",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior46",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x9c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior46",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior47",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x9e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior47",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior48",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior48",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior49",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior49",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior50",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior50",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior51",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior51",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior52",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior52",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior53",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xaa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior53",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior54",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xad0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior54",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior55",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xae0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior55",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior56",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior56",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior57",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xb20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior57",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior58",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xb40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior58",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior59",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xb60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior59",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior60",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xb80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior60",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior61",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xba0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior61",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior62",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xbe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior62",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iprior63",
                    description: Some(
                        "Interrupt Priority configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xc00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iprior63",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctlr",
                    description: Some(
                        "System Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xd10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vtctlr",
                    description: Some(
                        "System Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xd14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vtctlr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "Interrupt Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hwstkctrl",
                    description: Some(
                        "RW,HWSTKCTRL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nestctrl",
                    description: Some(
                        "RW,NESTCTRL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nmiset",
                    description: Some(
                        "WO,NMISET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nmireset",
                    description: Some(
                        "WO,NMIRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "excset",
                    description: Some(
                        "WO,EXCSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "excreset",
                    description: Some(
                        "WO,EXCRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pficreset",
                    description: Some(
                        "WO,PFICRSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sysreset",
                    description: Some(
                        "WO,SYSRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "keycode",
                    description: Some(
                        "WO,KEYCODE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fibaddrr",
            extends: None,
            description: Some(
                "RW,Interrupt Fast Address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "baseaddr",
                    description: Some(
                        "BASEADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr0",
            extends: None,
            description: Some(
                "Interrupt 0 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr0",
                    description: Some(
                        "RW,OFFADDR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid0",
                    description: Some(
                        "RW,IRQID0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr1",
            extends: None,
            description: Some(
                "Interrupt 1 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr1",
                    description: Some(
                        "RW,OFFADDR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid1",
                    description: Some(
                        "RW,IRQID1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr2",
            extends: None,
            description: Some(
                "Interrupt 2 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr2",
                    description: Some(
                        "RW,OFFADDR2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid2",
                    description: Some(
                        "RW,IRQID2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fifoaddrr3",
            extends: None,
            description: Some(
                "Interrupt 3 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "offaddr3",
                    description: Some(
                        "RW,OFFADDR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "irqid3",
                    description: Some(
                        "RW,IRQID3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gisr",
            extends: None,
            description: Some(
                "Interrupt Global Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "neststa",
                    description: Some(
                        "RO,NESTSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gactsta",
                    description: Some(
                        "RO,GACTSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpendsta",
                    description: Some(
                        "RO,GPENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr1",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "RW1,IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iactr2",
            extends: None,
            description: Some(
                "Interrupt ACTIVE Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iacts",
                    description: Some(
                        "RW1,IACTS.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr1",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "RW1,INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ienr2",
            extends: None,
            description: Some(
                "Interrupt Setting Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inten",
                    description: Some(
                        "RW1,INTEN.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr1",
            extends: None,
            description: Some(
                "RO,Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipr2",
            extends: None,
            description: Some(
                "RO,Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendsta",
                    description: Some(
                        "PENDSTA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior0",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior0",
                    description: Some(
                        "RW,IPRIOR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior1",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior1",
                    description: Some(
                        ">RW,IPRIOR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior10",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior10",
                    description: Some(
                        ">RW,IPRIOR10.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior11",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior11",
                    description: Some(
                        "RW,IPRIOR11.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior12",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior12",
                    description: Some(
                        "RW,IPRIOR12.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior13",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior13",
                    description: Some(
                        "RW,IPRIOR13.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior14",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior14",
                    description: Some(
                        "RW,IPRIOR14.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior15",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior15",
                    description: Some(
                        "RW,IPRIOR15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior16",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior16",
                    description: Some(
                        "RW,IPRIOR16.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior17",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior17",
                    description: Some(
                        "RW,IPRIOR17.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior18",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior18",
                    description: Some(
                        "RW,IPRIOR18.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior19",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior19",
                    description: Some(
                        "RW,IPRIOR19.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior2",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior2",
                    description: Some(
                        ">RW,IPRIOR2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior20",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior20",
                    description: Some(
                        "RW,IPRIOR20.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior21",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior21",
                    description: Some(
                        "RW,IPRIOR21.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior22",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior22",
                    description: Some(
                        "RW,IPRIOR22.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior23",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior23",
                    description: Some(
                        "RW,IPRIOR23.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior24",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior24",
                    description: Some(
                        "RW,IPRIOR24.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior25",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior25",
                    description: Some(
                        "RW,IPRIOR25.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior26",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior26",
                    description: Some(
                        "RW,IPRIOR26.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior27",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior27",
                    description: Some(
                        "RW,IPRIOR27.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior28",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior28",
                    description: Some(
                        "RW,IPRIOR28.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior29",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior29",
                    description: Some(
                        "RW,IPRIOR29.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior3",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior3",
                    description: Some(
                        ">RW,IPRIOR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior30",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior30",
                    description: Some(
                        "RW,IPRIOR30.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior31",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior31",
                    description: Some(
                        "RW,IPRIOR31.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior32",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior32",
                    description: Some(
                        "RW,IPRIOR32.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior33",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior33",
                    description: Some(
                        "RW,IPRIOR33.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior34",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior34",
                    description: Some(
                        "RW,IPRIOR34.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior35",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior35",
                    description: Some(
                        "RW,IPRIOR35.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior36",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior36",
                    description: Some(
                        "RW,IPRIOR36.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior37",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior37",
                    description: Some(
                        "RW,IPRIOR37.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior38",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior38",
                    description: Some(
                        "RW,IPRIOR38.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior39",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior39",
                    description: Some(
                        "RW,IPRIOR39.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior4",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior4",
                    description: Some(
                        ">RW,IPRIOR4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior40",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior40",
                    description: Some(
                        "RW,IPRIOR40.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior41",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior41",
                    description: Some(
                        "RW,IPRIOR41.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior42",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior42",
                    description: Some(
                        "RW,IPRIOR42.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior43",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior43",
                    description: Some(
                        "RW,IPRIOR43.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior44",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior44",
                    description: Some(
                        "RW,IPRIOR44.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior45",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior45",
                    description: Some(
                        "RW,IPRIOR45.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior46",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior46",
                    description: Some(
                        "RW,IPRIOR46.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior47",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior47",
                    description: Some(
                        "RW,IPRIOR47.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior48",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior48",
                    description: Some(
                        "RW,IPRIOR48.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior49",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior49",
                    description: Some(
                        "RW,IPRIOR49.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior5",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior5",
                    description: Some(
                        ">RW,IPRIOR5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior50",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior50",
                    description: Some(
                        "RW,IPRIOR50.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior51",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior51",
                    description: Some(
                        "RW,IPRIOR51.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior52",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior52",
                    description: Some(
                        "RW,IPRIOR52.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior53",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior53",
                    description: Some(
                        "RW,IPRIOR53.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior54",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior54",
                    description: Some(
                        "RW,IPRIOR54.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior55",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior55",
                    description: Some(
                        "RW,IPRIOR55.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior56",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior56",
                    description: Some(
                        "RW,IPRIOR56.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior57",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior57",
                    description: Some(
                        "RW,IPRIOR57.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior58",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior58",
                    description: Some(
                        "RW,IPRIOR58.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior59",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior59",
                    description: Some(
                        "RW,IPRIOR59.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior6",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior6",
                    description: Some(
                        ">RW,IPRIOR6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior60",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior60",
                    description: Some(
                        "RW,IPRIOR60.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior61",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior61",
                    description: Some(
                        "RW,IPRIOR61.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior62",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior62",
                    description: Some(
                        "RW,IPRIOR62.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior63",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior63",
                    description: Some(
                        "RW,IPRIOR63.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior7",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior7",
                    description: Some(
                        ">RW,IPRIOR7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior8",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior8",
                    description: Some(
                        ">RW,IPRIOR8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprior9",
            extends: None,
            description: Some(
                "Interrupt Priority configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior9",
                    description: Some(
                        ">RW,IPRIOR9.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr1",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendreset",
                    description: Some(
                        "RW1,PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Iprr2",
            extends: None,
            description: Some(
                "Interrupt Pending Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendreset",
                    description: Some(
                        "RW1,PENDRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr1",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "RW1,PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipsr2",
            extends: None,
            description: Some(
                "Interrupt Pending Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pendset",
                    description: Some(
                        "RW1,PENDSET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer1",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intreset",
                    description: Some(
                        "RW1,INTRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Irer2",
            extends: None,
            description: Some(
                "Interrupt Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intreset",
                    description: Some(
                        "RW1,INTRESET.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr1",
            extends: None,
            description: Some(
                "RO,Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr2",
            extends: None,
            description: Some(
                "RO,Interrupt Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "intensta",
                    description: Some(
                        "Interrupt ID Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ithresdr",
            extends: None,
            description: Some(
                "RW,Interrupt Priority Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "threshold",
                    description: Some(
                        "RW,THRESHOLD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sctlr",
            extends: None,
            description: Some(
                "System Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sleeponexit",
                    description: Some(
                        "RW,SLEEPONEXIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sleepdeep",
                    description: Some(
                        "RW,SLEEPDEEP.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wfitowfe",
                    description: Some(
                        "RW,WFITOWFE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sevonpend",
                    description: Some(
                        "RW,SEVONPEND.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "setevent",
                    description: Some(
                        "WO,SETEVENT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vtctlr",
            extends: None,
            description: Some(
                "System Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtaddr",
                    description: Some(
                        "VTADDR.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                