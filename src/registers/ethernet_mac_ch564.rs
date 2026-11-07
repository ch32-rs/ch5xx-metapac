
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EthernetMac",
            extends: None,
            description: Some(
                "Ethernet: media access control.",
            ),
            items: &[
                BlockItem {
                    name: "maccr",
                    description: Some(
                        "Ethernet MAC configuration register. (ETH_MACCR).",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macffr",
                    description: Some(
                        "Ethernet MAC frame filter register. (ETH_MACCFFR).",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macffr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machthr",
                    description: Some(
                        "Ethernet MAC hash table high. register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machthr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "machtlr",
                    description: Some(
                        "Ethernet MAC hash table low. register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Machtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiiar",
                    description: Some(
                        "Ethernet MAC MII address register. (ETH_MACMIIAR).",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiiar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macmiidr",
                    description: Some(
                        "Ethernet MAC MII data register. (ETH_MACMIIDR).",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macmiidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macfcr",
                    description: Some(
                        "Ethernet MAC flow control register. (ETH_MACFCR).",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macvlan",
                    description: Some(
                        "Ethernet MAC VLAN tag register. (ETH_MACVLAN).",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macvlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macrwuffr",
                    description: Some(
                        "Ethernet MAC remote wakeup frame filter. register (ETH_MACRWUFFR).",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "macpmtcsr",
                    description: Some(
                        "Ethernet MAC PMT control and status register. (ETH_MACPMTCSR).",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macpmtcsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macsr",
                    description: Some(
                        "Ethernet MAC interrupt status register. (ETH_MACSR).",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "macimr",
                    description: Some(
                        "Ethernet MAC interrupt mask register. (ETH_MACIMR).",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Macimr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0hr",
                    description: Some(
                        "Ethernet MAC address 0 high register. (ETH_MACA0HR).",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca0lr",
                    description: Some(
                        "Ethernet MAC address 0 low. register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca0lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1hr",
                    description: Some(
                        "Ethernet MAC address 1 high register. (ETH_MACA1HR).",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca1lr",
                    description: Some(
                        "Ethernet MAC address1 low. register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca1lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2hr",
                    description: Some(
                        "Ethernet MAC address 2 high register. (ETH_MACA2HR).",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca2lr",
                    description: Some(
                        "Ethernet MAC address 2 low. register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca2lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3hr",
                    description: Some(
                        "Ethernet MAC address 3 high register. (ETH_MACA3HR).",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3hr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "maca3lr",
                    description: Some(
                        "Ethernet MAC address 3 low. register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Maca3lr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "phy_cr",
                    description: Some(
                        "PHY configure register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PhyCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chksum_cr",
                    description: Some(
                        "CHKSUM configure register.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChksumCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ip_pdr",
                    description: Some(
                        "IP packet register.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpPdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chksum_hr",
                    description: Some(
                        "IP packet register.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChksumHr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chksum_pr",
                    description: Some(
                        "IP data register.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ChksumPr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChksumCr",
            extends: None,
            description: Some(
                "CHKSUM configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "byte_en1",
                    description: Some(
                        "Participate in the ip header checksum to. calculate byte control bits.",
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
                Field {
                    name: "byte_en2",
                    description: Some(
                        "Participate in the ip header checksum to. calculate byte control bits.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chksum_en",
                    description: Some(
                        "the software sets 1 at the beginning of frame calculation. and write 0  at the end of frame calculation.",
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
            ],
        },
        FieldSet {
            name: "ChksumHr",
            extends: None,
            description: Some(
                "IP packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chksum_head_result",
                    description: Some(
                        "IP header checksum.",
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
            ],
        },
        FieldSet {
            name: "ChksumPr",
            extends: None,
            description: Some(
                "IP data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chksum_head_result",
                    description: Some(
                        "IP header checksum.",
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
            ],
        },
        FieldSet {
            name: "IpPdr",
            extends: None,
            description: Some(
                "IP packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ip_payload",
                    description: Some(
                        "IP data written by software,need to be used with. ETH_CHKSUM_CR.",
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
            name: "Maca0hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 high register. (ETH_MACA0HR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0h",
                    description: Some(
                        "MAC address0 high.",
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
                    name: "mo",
                    description: Some(
                        "Always 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca0lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca0l",
                    description: Some(
                        "MAC address0 low.",
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
            name: "Maca1hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 1 high register. (ETH_MACA1HR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1h",
                    description: Some(
                        "MAC address1 high.",
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
                    name: "mbc",
                    description: Some(
                        "Mask byte control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca1lr",
            extends: None,
            description: Some(
                "Ethernet MAC address1 low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca1l",
                    description: Some(
                        "MAC address1 low.",
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
            name: "Maca2hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 high register. (ETH_MACA2HR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2h",
                    description: Some(
                        "Ethernet MAC address 2 high. register.",
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
                    name: "mbc",
                    description: Some(
                        "Mask byte control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca2lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca2l",
                    description: Some(
                        "MAC address2 low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca3hr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 high register. (ETH_MACA3HR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maca3h",
                    description: Some(
                        "MAC address3 high.",
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
                    name: "mbc",
                    description: Some(
                        "Mask byte control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sa",
                    description: Some(
                        "Source address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae",
                    description: Some(
                        "Address enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Maca3lr",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mbca3l",
                    description: Some(
                        "MAC address3 low.",
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
            name: "Maccr",
            extends: None,
            description: Some(
                "Ethernet MAC configuration register. (ETH_MACCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "re",
                    description: Some(
                        "Receiver enable.",
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
                    name: "te",
                    description: Some(
                        "Transmitter enable.",
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
                    name: "apcs",
                    description: Some(
                        "Automatic pad/CRC. stripping.",
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
                    name: "rd",
                    description: Some(
                        "Retry disable.",
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
                    name: "ipco",
                    description: Some(
                        "IPv4 checksum offload.",
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
                    name: "dm",
                    description: Some(
                        "Duplex mode.",
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
                    name: "fes",
                    description: Some(
                        "Fast Ethernet speed.",
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
                    name: "ifg",
                    description: Some(
                        "Interframe gap.",
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
                    name: "jd",
                    description: Some(
                        "Jabber disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wd",
                    description: Some(
                        "Watchdog disable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macfcr",
            extends: None,
            description: Some(
                "Ethernet MAC flow control register. (ETH_MACFCR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fcb",
                    description: Some(
                        "Flow control busy.",
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
                    name: "tfce",
                    description: Some(
                        "Transmit flow control. enable.",
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
                    name: "rfce",
                    description: Some(
                        "Receive flow control. enable.",
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
                    name: "upfd",
                    description: Some(
                        "Unicast pause frame detect.",
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
                    name: "pt",
                    description: Some(
                        "Pass control frames.",
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
            name: "Macffr",
            extends: None,
            description: Some(
                "Ethernet MAC frame filter register. (ETH_MACCFFR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pm",
                    description: Some(
                        "Promiscuous mode.",
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
                    name: "hu",
                    description: Some(
                        "Hash unicast.",
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
                    name: "hm",
                    description: Some(
                        "Hash multicast.",
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
                    name: "daif",
                    description: Some(
                        "Destination address inverse. filtering.",
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
                    name: "pam",
                    description: Some(
                        "Pass all multicast.",
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
                    name: "bfd",
                    description: Some(
                        "Broadcast frames disable.",
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
                    name: "pcf",
                    description: Some(
                        "Pass control frames.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saif",
                    description: Some(
                        "Source address inverse. filtering.",
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
                    name: "saf",
                    description: Some(
                        "Source address filter.",
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
                    name: "hpf",
                    description: Some(
                        "Hash or perfect filter.",
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
                    name: "ra",
                    description: Some(
                        "Receive all.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Machthr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table high. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hth",
                    description: Some(
                        "Hash table high.",
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
            name: "Machtlr",
            extends: None,
            description: Some(
                "Ethernet MAC hash table low. register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htl",
                    description: Some(
                        "Hash table low.",
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
            name: "Macimr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt mask register. (ETH_MACIMR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmtim",
                    description: Some(
                        "PMT interrupt mask.",
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
                    name: "tstim",
                    description: Some(
                        "Time stamp trigger interrupt. mask.",
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
            name: "Macmiiar",
            extends: None,
            description: Some(
                "Ethernet MAC MII address register. (ETH_MACMIIAR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mb",
                    description: Some(
                        "MII busy.",
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
                    name: "mw",
                    description: Some(
                        "MII write.",
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
                    name: "cr",
                    description: Some(
                        "Clock range.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mr",
                    description: Some(
                        "MII register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa",
                    description: Some(
                        "PHY address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macmiidr",
            extends: None,
            description: Some(
                "Ethernet MAC MII data register. (ETH_MACMIIDR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "md",
                    description: Some(
                        "MII data.",
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
            ],
        },
        FieldSet {
            name: "Macpmtcsr",
            extends: None,
            description: Some(
                "Ethernet MAC PMT control and status register. (ETH_MACPMTCSR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "Power down.",
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
                    name: "mpe",
                    description: Some(
                        "Magic Packet enable.",
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
                    name: "wfe",
                    description: Some(
                        "Wakeup frame enable.",
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
                    name: "phye",
                    description: Some(
                        "PMEB of PHY enable.",
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
                    name: "mpr",
                    description: Some(
                        "Magic packet received.",
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
                    name: "wfr",
                    description: Some(
                        "Wakeup frame received.",
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
                    name: "phyr",
                    description: Some(
                        "wake-up frame flag bit of PHY.",
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
                    name: "gu",
                    description: Some(
                        "Global unicast.",
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
                    name: "wffrpr",
                    description: Some(
                        "Wakeup frame filter register pointer. reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Macsr",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt status register. (ETH_MACSR).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pmts",
                    description: Some(
                        "PMT status.",
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
                    name: "mmcs",
                    description: Some(
                        "MMC status.",
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
                    name: "mmcrs",
                    description: Some(
                        "MMC receive status.",
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
                    name: "mmcts",
                    description: Some(
                        "MMC transmit status.",
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
                    name: "tsts",
                    description: Some(
                        "Time stamp trigger status.",
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
            name: "Macvlan",
            extends: None,
            description: Some(
                "Ethernet MAC VLAN tag register. (ETH_MACVLAN).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlanti",
                    description: Some(
                        "VLAN tag identifier (for receive. frames).",
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
                    name: "vlantc",
                    description: Some(
                        "12-bit VLAN tag comparison.",
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
            name: "PhyCr",
            extends: None,
            description: Some(
                "PHY configure register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rephyaddr",
                    description: Some(
                        "when the bit 7 set 1,bit4-0 serves as the address of the. PHY.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phyaddr_en",
                    description: Some(
                        "Reconfigure the PHY address.",
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
                    name: "speed",
                    description: Some(
                        "Even Ethernet speed selection.",
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
                    name: "dupl_ex",
                    description: Some(
                        "Even Ethernet mode selection.",
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
                    name: "phy_eee",
                    description: Some(
                        "PHY work mode selection.",
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
                Field {
                    name: "phy_pd",
                    description: Some(
                        "PHY pd mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "phy_rstn",
                    description: Some(
                        "Ethernet PHY global reset signal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
                