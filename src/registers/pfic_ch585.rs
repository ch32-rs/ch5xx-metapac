
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
                        "RO,Interrupt Status Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RO,Interrupt Status Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RO,Interrupt Pending Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RO,Interrupt Pending Register 2.",
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
                    name: "cfgr",
                    description: Some(
                        "Interrupt Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                    name: "idcfgr",
                    description: Some(
                        "RW,Interrupt Fast ID Config Register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fiaddrr0",
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
                                "Fiaddrr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fiaddrr1",
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
                                "Fiaddrr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fiaddrr2",
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
                                "Fiaddrr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fiaddrr3",
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
                                "Fiaddrr3",
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                            access: Access::Write,
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
                        "Interrupt Priority configuration Register 0.",
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
                        "Interrupt Priority configuration Register 1.",
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
                        "Interrupt Priority configuration Register 2.",
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
                        "Interrupt Priority configuration Register 3.",
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
                        "Interrupt Priority configuration Register 4.",
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
                        "Interrupt Priority configuration Register 5.",
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
                        "Interrupt Priority configuration Register 6.",
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
                        "Interrupt Priority configuration Register 7.",
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
                        "Interrupt Priority configuration Register 8.",
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
                        "Interrupt Priority configuration Register 9.",
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
                        "Interrupt Priority configuration Register 10.",
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
                        "Interrupt Priority configuration Register 11.",
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
                        "Interrupt Priority configuration Register 12.",
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
                        "Interrupt Priority configuration Register 13.",
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
                        "Interrupt Priority configuration Register 14.",
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
                        "Interrupt Priority configuration Register 15.",
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
                        "Interrupt Priority configuration Register 16.",
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
                        "Interrupt Priority configuration Register 17.",
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
                        "Interrupt Priority configuration Register 18.",
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
                        "Interrupt Priority configuration Register 19.",
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
                        "Interrupt Priority configuration Register 20.",
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
                        "Interrupt Priority configuration Register 21.",
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
                        "Interrupt Priority configuration Register 22.",
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
                        "Interrupt Priority configuration Register 23.",
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
                        "Interrupt Priority configuration Register 24.",
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
                        "Interrupt Priority configuration Register 25.",
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
                        "Interrupt Priority configuration Register 26.",
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
                        "Interrupt Priority configuration Register 27.",
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
                        "Interrupt Priority configuration Register 28.",
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
                        "Interrupt Priority configuration Register 29.",
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
                        "Interrupt Priority configuration Register 30.",
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
                        "Interrupt Priority configuration Register 31.",
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
                        "Interrupt Priority configuration Register 32.",
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
                        "Interrupt Priority configuration Register 33.",
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
                        "Interrupt Priority configuration Register 34.",
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
                        "Interrupt Priority configuration Register 35.",
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
                        "Interrupt Priority configuration Register 36.",
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
                        "Interrupt Priority configuration Register 37.",
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
                        "Interrupt Priority configuration Register 38.",
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
                        "Interrupt Priority configuration Register 39.",
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
                        "Interrupt Priority configuration Register 40.",
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
                        "Interrupt Priority configuration Register 41.",
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
                        "Interrupt Priority configuration Register 42.",
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
                        "Interrupt Priority configuration Register 43.",
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
                        "Interrupt Priority configuration Register 44.",
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
                        "Interrupt Priority configuration Register 45.",
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
                        "Interrupt Priority configuration Register 46.",
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
                        "Interrupt Priority configuration Register 47.",
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
                        "Interrupt Priority configuration Register 48.",
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
                        "Interrupt Priority configuration Register 49.",
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
                        "Interrupt Priority configuration Register 50.",
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
                        "Interrupt Priority configuration Register 51.",
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
                        "Interrupt Priority configuration Register 52.",
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
                        "Interrupt Priority configuration Register 53.",
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
                        "Interrupt Priority configuration Register 54.",
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
                        "Interrupt Priority configuration Register 55.",
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
                        "Interrupt Priority configuration Register 56.",
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
                        "Interrupt Priority configuration Register 57.",
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
                        "Interrupt Priority configuration Register 58.",
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
                        "Interrupt Priority configuration Register 59.",
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
                        "Interrupt Priority configuration Register 60.",
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
                        "Interrupt Priority configuration Register 61.",
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
                        "Interrupt Priority configuration Register 62.",
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
                        "Interrupt Priority configuration Register 63.",
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
                    name: "resetsys",
                    description: Some(
                        "WO,RESETSYS.",
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
            name: "Fiaddrr0",
            extends: None,
            description: Some(
                "Interrupt 0 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fi0en",
                    description: Some(
                        "RW,Fast interrupt channel 0 enable bit.",
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
                    name: "addr0",
                    description: Some(
                        "RW,ADDR0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fiaddrr1",
            extends: None,
            description: Some(
                "Interrupt 1 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fi1en",
                    description: Some(
                        "RW,Fast interrupt channel 1 enable bit.",
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
                    name: "addr1",
                    description: Some(
                        "RW,ADDR1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fiaddrr2",
            extends: None,
            description: Some(
                "Interrupt 2 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fi2en",
                    description: Some(
                        "RW,Fast interrupt channel 2 enable bit.",
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
                    name: "addr2",
                    description: Some(
                        "RW,ADDR2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fiaddrr3",
            extends: None,
            description: Some(
                "Interrupt 3 address Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fi3en",
                    description: Some(
                        "RW,Fast interrupt channel 3 enable bit.",
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
                    name: "addr3",
                    description: Some(
                        "RW,ADDR3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 31,
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
            name: "Idcfgr",
            extends: None,
            description: Some(
                "RW,Interrupt Fast ID Config Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fiid0",
                    description: Some(
                        "RW,FIID0.",
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
                    name: "fiid1",
                    description: Some(
                        "RW,FIID1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fiid2",
                    description: Some(
                        "RW,FIID2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fiid3",
                    description: Some(
                        "RW,FIID3.",
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
                "RO,Interrupt Pending Register 1.",
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
                "RO,Interrupt Pending Register 2.",
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
                "Interrupt Priority configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior0",
                    description: Some(
                        "RW,Interrupt priority for number 0-3.",
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
                "Interrupt Priority configuration Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior1",
                    description: Some(
                        ">RW,Interrupt priority for number 4-7.",
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
                "Interrupt Priority configuration Register 10.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior10",
                    description: Some(
                        ">RW,Interrupt priority for number 40-43.",
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
                "Interrupt Priority configuration Register 11.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior11",
                    description: Some(
                        "RW,Interrupt priority for number 44-47.",
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
                "Interrupt Priority configuration Register 12.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior12",
                    description: Some(
                        "RW,Interrupt priority for number 48-51.",
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
                "Interrupt Priority configuration Register 13.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior13",
                    description: Some(
                        "RW,Interrupt priority for number 52-55.",
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
                "Interrupt Priority configuration Register 14.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior14",
                    description: Some(
                        "RW,Interrupt priority for number 56-59.",
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
                "Interrupt Priority configuration Register 15.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior15",
                    description: Some(
                        "RW,Interrupt priority for number 60-63.",
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
                "Interrupt Priority configuration Register 16.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior16",
                    description: Some(
                        "RW,Interrupt priority for number 64-67.",
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
                "Interrupt Priority configuration Register 17.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior17",
                    description: Some(
                        "RW,Interrupt priority for number 68-71.",
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
                "Interrupt Priority configuration Register 18.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior18",
                    description: Some(
                        "RW,Interrupt priority for number 72-75.",
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
                "Interrupt Priority configuration Register 19.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior19",
                    description: Some(
                        "RW,Interrupt priority for number 76-79.",
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
                "Interrupt Priority configuration Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior2",
                    description: Some(
                        ">RW,Interrupt priority for number 8-11.",
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
                "Interrupt Priority configuration Register 20.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior20",
                    description: Some(
                        "RW,RW,Interrupt priority for number 80-83.",
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
                "Interrupt Priority configuration Register 21.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior21",
                    description: Some(
                        "RW,Interrupt priority for number 84-87.",
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
                "Interrupt Priority configuration Register 22.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior22",
                    description: Some(
                        "RW,Interrupt priority for number 88-91.",
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
                "Interrupt Priority configuration Register 23.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior23",
                    description: Some(
                        "RW,Interrupt priority for number 92-95.",
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
                "Interrupt Priority configuration Register 24.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior24",
                    description: Some(
                        "RW,Interrupt priority for number 96-99.",
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
                "Interrupt Priority configuration Register 25.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior25",
                    description: Some(
                        "RW,Interrupt priority for number 100-103.",
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
                "Interrupt Priority configuration Register 26.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior26",
                    description: Some(
                        "RW,Interrupt priority for number 104-107.",
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
                "Interrupt Priority configuration Register 27.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior27",
                    description: Some(
                        "RW,Interrupt priority for number 108-111.",
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
                "Interrupt Priority configuration Register 28.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior28",
                    description: Some(
                        "RW,Interrupt priority for number 112-115.",
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
                "Interrupt Priority configuration Register 29.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior29",
                    description: Some(
                        "RW,Interrupt priority for number 116-119.",
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
                "Interrupt Priority configuration Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior3",
                    description: Some(
                        ">RW,Interrupt priority for number 12-15.",
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
                "Interrupt Priority configuration Register 30.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior30",
                    description: Some(
                        "RW,Interrupt priority for number 120-123.",
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
                "Interrupt Priority configuration Register 31.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior31",
                    description: Some(
                        "RW,Interrupt priority for number 124-127.",
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
                "Interrupt Priority configuration Register 32.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior32",
                    description: Some(
                        "RW,Interrupt priority for number 128-131.",
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
                "Interrupt Priority configuration Register 33.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior33",
                    description: Some(
                        "RW,Interrupt priority for number 132-135.",
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
                "Interrupt Priority configuration Register 34.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior34",
                    description: Some(
                        "RW,Interrupt priority for number 136-139.",
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
                "Interrupt Priority configuration Register 35.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior35",
                    description: Some(
                        "RW,Interrupt priority for number 140-143.",
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
                "Interrupt Priority configuration Register 36.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior36",
                    description: Some(
                        "RW,Interrupt priority for number 144-147.",
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
                "Interrupt Priority configuration Register 37.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior37",
                    description: Some(
                        "RW,Interrupt priority for number 148-151.",
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
                "Interrupt Priority configuration Register 38.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior38",
                    description: Some(
                        "RW,Interrupt priority for number 152-155.",
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
                "Interrupt Priority configuration Register 39.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior39",
                    description: Some(
                        "RW,Interrupt priority for number 156-159.",
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
                "Interrupt Priority configuration Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior4",
                    description: Some(
                        ">RW,Interrupt priority for number 16-19.",
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
                "Interrupt Priority configuration Register 40.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior40",
                    description: Some(
                        "RW,Interrupt priority for number 160-163.",
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
                "Interrupt Priority configuration Register 41.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior41",
                    description: Some(
                        "RW,Interrupt priority for number 164-167.",
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
                "Interrupt Priority configuration Register 42.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior42",
                    description: Some(
                        "RW,Interrupt priority for number 168-171.",
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
                "Interrupt Priority configuration Register 43.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior43",
                    description: Some(
                        "RW,Interrupt priority for number 172-175.",
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
                "Interrupt Priority configuration Register 44.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior44",
                    description: Some(
                        "RW,Interrupt priority for number 176-179.",
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
                "Interrupt Priority configuration Register 45.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior45",
                    description: Some(
                        "RW,Interrupt priority for number 180-183.",
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
                "Interrupt Priority configuration Register 46.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior46",
                    description: Some(
                        "RW,Interrupt priority for number 184-187.",
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
                "Interrupt Priority configuration Register 47.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior47",
                    description: Some(
                        "RW,Interrupt priority for number 188-191.",
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
                "Interrupt Priority configuration Register 48.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior48",
                    description: Some(
                        "RW,Interrupt priority for number 192-195.",
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
                "Interrupt Priority configuration Register 49.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior49",
                    description: Some(
                        "RW,Interrupt priority for number 196-199.",
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
                "Interrupt Priority configuration Register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior5",
                    description: Some(
                        ">RW,Interrupt priority for number 20-23.",
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
                "Interrupt Priority configuration Register 50.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior50",
                    description: Some(
                        "RW,Interrupt priority for number 200-203.",
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
                "Interrupt Priority configuration Register 51.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior51",
                    description: Some(
                        "RW,Interrupt priority for number 204-207.",
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
                "Interrupt Priority configuration Register 52.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior52",
                    description: Some(
                        "RW,Interrupt priority for number 208-211.",
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
                "Interrupt Priority configuration Register 53.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior53",
                    description: Some(
                        "RW,Interrupt priority for number 212-215.",
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
                "Interrupt Priority configuration Register 54.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior54",
                    description: Some(
                        "RW,Interrupt priority for number 216-219.",
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
                "Interrupt Priority configuration Register 55.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior55",
                    description: Some(
                        "RW,Interrupt priority for number 220-223.",
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
                "Interrupt Priority configuration Register 56.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior56",
                    description: Some(
                        "RW,Interrupt priority for number 224-227.",
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
                "Interrupt Priority configuration Register 57.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior57",
                    description: Some(
                        "RW,Interrupt priority for number 228-231.",
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
                "Interrupt Priority configuration Register 58.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior58",
                    description: Some(
                        "RW,Interrupt priority for number 232-235.",
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
                "Interrupt Priority configuration Register 59.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior59",
                    description: Some(
                        "RW,Interrupt priority for number 236-239.",
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
                "Interrupt Priority configuration Register 6.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior6",
                    description: Some(
                        ">RW,Interrupt priority for number 24-27.",
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
                "Interrupt Priority configuration Register 60.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior60",
                    description: Some(
                        "RW,Interrupt priority for number 240-243.",
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
                "Interrupt Priority configuration Register 61.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior61",
                    description: Some(
                        "RW,Interrupt priority for number 244-247.",
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
                "Interrupt Priority configuration Register 62.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior62",
                    description: Some(
                        "RW,Interrupt priority for number 248-251.",
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
                "Interrupt Priority configuration Register 63.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior63",
                    description: Some(
                        "RW,Interrupt priority for number 252-255.",
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
                "Interrupt Priority configuration Register 7.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior7",
                    description: Some(
                        ">RW,Interrupt priority for number 28-31.",
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
                "Interrupt Priority configuration Register 8.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior8",
                    description: Some(
                        "RW,Interrupt priority for number 32-35.",
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
                "Interrupt Priority configuration Register 9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iprior9",
                    description: Some(
                        ">RW,Interrupt priority for number 36-39.",
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
                "RO,Interrupt Status Register 1.",
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
                "RO,Interrupt Status Register 2.",
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
    ],
    enums: &[],
};
                