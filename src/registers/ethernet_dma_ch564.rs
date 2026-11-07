
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EthernetDma",
            extends: None,
            description: Some(
                "Ethernet: DMA controller operation.",
            ),
            items: &[
                BlockItem {
                    name: "dmabmr",
                    description: Some(
                        "Ethernet DMA bus mode register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmabmr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatpdr",
                    description: Some(
                        "Ethernet DMA transmit poll demand. register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmarpdr",
                    description: Some(
                        "EHERNET DMA receive poll demand. register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmarpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmardlar",
                    description: Some(
                        "Ethernet DMA receive descriptor list address. register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmardlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatdlar",
                    description: Some(
                        "Ethernet DMA transmit descriptor list. address register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatdlar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmasr",
                    description: Some(
                        "Ethernet DMA status register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmasr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaomr",
                    description: Some(
                        "Ethernet DMA operation mode. register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaomr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaier",
                    description: Some(
                        "Ethernet DMA interrupt enable. register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamfbocr",
                    description: Some(
                        "Ethernet DMA missed frame and buffer. overflow counter register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamfbocr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtdr",
                    description: Some(
                        "Ethernet DMA current host transmit. descriptor register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrdr",
                    description: Some(
                        "Ethernet DMA current host receive descriptor. register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachtbar",
                    description: Some(
                        "Ethernet DMA current host transmit buffer. address register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachtbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmachrbar",
                    description: Some(
                        "Ethernet DMA current host receive buffer. address register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmachrbar",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dmabmr",
            extends: None,
            description: Some(
                "Ethernet DMA bus mode register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "Software reset.",
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
                    name: "dsl",
                    description: Some(
                        "Descriptor skip length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmachrbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive buffer. address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrbap",
                    description: Some(
                        "Host receive buffer address. pointer.",
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
            name: "Dmachrdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host receive descriptor. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hrdap",
                    description: Some(
                        "Host receive descriptor address. pointer.",
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
            name: "Dmachtbar",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit buffer. address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htbap",
                    description: Some(
                        "Host transmit buffer address. pointer.",
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
            name: "Dmachtdr",
            extends: None,
            description: Some(
                "Ethernet DMA current host transmit. descriptor register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htdap",
                    description: Some(
                        "Host transmit descriptor address. pointer.",
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
            name: "Dmaier",
            extends: None,
            description: Some(
                "Ethernet DMA interrupt enable. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tie",
                    description: Some(
                        "Transmit interrupt enable.",
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
                    name: "tpsie",
                    description: Some(
                        "Transmit process stopped interrupt. enable.",
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
                    name: "tbuie",
                    description: Some(
                        "Transmit buffer unavailable interrupt. enable.",
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
                    name: "tjtie",
                    description: Some(
                        "Transmit jabber timeout interrupt. enable.",
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
                    name: "roie",
                    description: Some(
                        "Overflow interrupt enable.",
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
                    name: "tuie",
                    description: Some(
                        "Underflow interrupt enable.",
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
                    name: "rie",
                    description: Some(
                        "Receive interrupt enable.",
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
                    name: "rbuie",
                    description: Some(
                        "Receive buffer unavailable interrupt. enable.",
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
                    name: "rpsie",
                    description: Some(
                        "Receive process stopped interrupt. enable.",
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
                    name: "rwtie",
                    description: Some(
                        "receive watchdog timeout interrupt. enable.",
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
                Field {
                    name: "etie",
                    description: Some(
                        "Early transmit interrupt. enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phylink_ier",
                    description: Some(
                        "Early send interrupt enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ers",
                    description: Some(
                        "Early receive interrupt. enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aise",
                    description: Some(
                        "Abnormal interrupt summary. enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nise",
                    description: Some(
                        "Normal interrupt summary. enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmamfbocr",
            extends: None,
            description: Some(
                "Ethernet DMA missed frame and buffer. overflow counter register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfc",
                    description: Some(
                        "Missed frames by the. controller.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "omfc",
                    description: Some(
                        "Overflow bit for missed frame. counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mfa",
                    description: Some(
                        "Missed frames by the. application.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ofoc",
                    description: Some(
                        "Overflow bit for FIFO overflow. counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmaomr",
            extends: None,
            description: Some(
                "Ethernet DMA operation mode. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sr",
                    description: Some(
                        "SR.",
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
                    name: "fugf",
                    description: Some(
                        "FUGF.",
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
                    name: "fef",
                    description: Some(
                        "FEF.",
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
                    name: "st",
                    description: Some(
                        "ST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftf",
                    description: Some(
                        "FTF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsf",
                    description: Some(
                        "TSF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcefd",
                    description: Some(
                        "DTCEFD.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmardlar",
            extends: None,
            description: Some(
                "Ethernet DMA receive descriptor list address. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdlar",
                    description: Some(
                        "Start of receive list.",
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
            name: "Dmarpdr",
            extends: None,
            description: Some(
                "EHERNET DMA receive poll demand. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rpdr",
                    description: Some(
                        "Receive poll demand.",
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
            name: "Dmasr",
            extends: None,
            description: Some(
                "Ethernet DMA status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ts",
                    description: Some(
                        "Transmit status.",
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
                    name: "tpss",
                    description: Some(
                        "Transmit process stopped. status.",
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
                    name: "tbus",
                    description: Some(
                        "Transmit buffer unavailable. status.",
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
                    name: "tjts",
                    description: Some(
                        "Transmit jabber timeout. status.",
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
                    name: "ros",
                    description: Some(
                        "Receive overflow status.",
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
                    name: "tus",
                    description: Some(
                        "Transmit underflow status.",
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
                    name: "rs",
                    description: Some(
                        "Receive status.",
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
                    name: "rbus",
                    description: Some(
                        "Receive buffer unavailable. status.",
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
                    name: "rpss",
                    description: Some(
                        "Receive process stopped. status.",
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
                    name: "pwts",
                    description: Some(
                        "Receive watchdog timeout. status.",
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
                Field {
                    name: "ets",
                    description: Some(
                        "Early transmit status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phylink_sr",
                    description: Some(
                        "read interrupt source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ers",
                    description: Some(
                        "Early receive status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ais",
                    description: Some(
                        "Abnormal interrupt summary.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nis",
                    description: Some(
                        "Normal interrupt summary.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rps",
                    description: Some(
                        "Receive process state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tps",
                    description: Some(
                        "Transmit process state.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ebs",
                    description: Some(
                        "Error bits status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmcs",
                    description: Some(
                        "MMC status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dmatdlar",
            extends: None,
            description: Some(
                "Ethernet DMA transmit descriptor list. address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdlar",
                    description: Some(
                        "Start of transmit list.",
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
            name: "Dmatpdr",
            extends: None,
            description: Some(
                "Ethernet DMA transmit poll demand. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpdr",
                    description: Some(
                        "Transmit poll demand.",
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
    ],
    enums: &[],
};
                