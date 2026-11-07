
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "I2c",
            extends: None,
            description: Some(
                "I2C register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl1",
                    description: Some(
                        "RW, I2C control 1.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ctrl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl2",
                    description: Some(
                        "RW, I2C control 2.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ctrl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oaddr1",
                    description: Some(
                        "RW, I2C own address register 1.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Oaddr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "oaddr2",
                    description: Some(
                        "RW, I2C own address register 2.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Oaddr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "datar",
                    description: Some(
                        "RW, I2C data register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Datar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "star1",
                    description: Some(
                        "RO, I2C stauts register 1.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Star1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "star2",
                    description: Some(
                        "RO, I2C status register 2.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Star2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckcfgr",
                    description: Some(
                        "RW, I2C clock control register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ckcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rtr",
                    description: Some(
                        "RW, I2C trise register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Rtr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ckcfgr",
            extends: None,
            description: Some(
                "RW, I2C clock control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ccr",
                    description: Some(
                        "RW, Controls the SCL clock in Fm/Sm mode (Master mode).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "duty",
                    description: Some(
                        "RW, Fm mode duty cycle: 0=L/H=2, 1=L/H=16/9.",
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
                    name: "f_s",
                    description: Some(
                        "RW, I2C master mode selection: 0=standard mode, 1=fast mode.",
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
            ],
        },
        FieldSet {
            name: "Ctrl1",
            extends: None,
            description: Some(
                "RW, I2C control 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pe",
                    description: Some(
                        "RW, Peripheral enable.",
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
                    name: "smbus",
                    description: Some(
                        "RW, SMBUS mode: 0=I2C mode, 1=SMBUS mode.",
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
                    name: "smbtype",
                    description: Some(
                        "RW, SMBus type: 0=Device, 1=Host.",
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
                    name: "ebarp",
                    description: Some(
                        "RW, ARP enable.",
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
                    name: "enpec",
                    description: Some(
                        "RW, PEC ebable.",
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
                    name: "engc",
                    description: Some(
                        "RW, General call enable.",
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
                    name: "nostretch",
                    description: Some(
                        "RW, Clock stretching disable (Slave mode).",
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
                    name: "start",
                    description: Some(
                        "RW, Start generation: master mode: 0=no start, 1=repeated start; slave mode: 0=no start, 1=start at bus free.",
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
                    name: "stop",
                    description: Some(
                        "RW, Stop generation: master mode: 0=no stop, 1=stop after the current byte transfer or after the current Start condition is sent; slave mode: 0=no stop, 1=Release the SCL and SDA lines after the current byte transfer.",
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
                    name: "ack",
                    description: Some(
                        "RW, Acknowledge enable.",
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
                    name: "pos",
                    description: Some(
                        "RW, Acknowledge/PEC Position (for data reception).",
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
                    name: "pec",
                    description: Some(
                        "RW, Packet error checking: 0=No PEC transfer, 1=PEC transfer (in Tx or Rx mode).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alert",
                    description: Some(
                        "RW, SMBus alert: 0=Releases SMBA pin high, 1=Drives SMBA pin low.",
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
                    name: "swrst",
                    description: Some(
                        "RW, Software reset.",
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
            ],
        },
        FieldSet {
            name: "Ctrl2",
            extends: None,
            description: Some(
                "RW, I2C control 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "freq",
                    description: Some(
                        "RW, Peripheral clock frequency, The minimum allowed frequency is 2 MHz,the maximum frequency is 36 MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iterren",
                    description: Some(
                        "RW, Error interrupt enable.",
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
                    name: "itevten",
                    description: Some(
                        "RW, Event interrupt enable.",
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
                    name: "itbufen",
                    description: Some(
                        "RW, Buffer interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "Datar",
            extends: None,
            description: Some(
                "RW, I2C data register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "datar",
                    description: Some(
                        "RW, I2C data register.",
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
            name: "Oaddr1",
            extends: None,
            description: Some(
                "RW, I2C own address register 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "add0",
                    description: Some(
                        "RW, bit0 of address in 10-bit addressing mode.",
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
                    name: "add7_1",
                    description: Some(
                        "RW, bit[7:1] of address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "add9_8",
                    description: Some(
                        "RW, bit[9:8] of address in 10-bit addressing mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addmode",
                    description: Some(
                        "RW, Addressing mode (slave mode): 0=7-bit slave address, 1=10-bit slave address.",
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
            ],
        },
        FieldSet {
            name: "Oaddr2",
            extends: None,
            description: Some(
                "RW, I2C own address register 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "endual",
                    description: Some(
                        "RW, Dual addressing mode enable.",
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
                    name: "add2",
                    description: Some(
                        "RW, bit[7:1] of address2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rtr",
            extends: None,
            description: Some(
                "RW, I2C trise register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "trise",
                    description: Some(
                        "RW, Maximum rise time in Fm/Sm mode (Master mode).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Star1",
            extends: None,
            description: Some(
                "RO, I2C stauts register 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "sb",
                    description: Some(
                        "RW0, Start bit flag (Master mode).",
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
                    name: "addr",
                    description: Some(
                        "RW0, Address sent (master mode)/matched (slave mode) flag.",
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
                    name: "btf",
                    description: Some(
                        "RO, Byte transfer finished flag.",
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
                    name: "add10",
                    description: Some(
                        "RO, 10-bit header sent flag (Master mode).",
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
                    name: "stopf",
                    description: Some(
                        "RO, Stop detection flag (slave mode).",
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
                    name: "rx_ne",
                    description: Some(
                        "RO, Data register not empty flag (receivers).",
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
                    name: "tx_e",
                    description: Some(
                        "RO, Data register empty flag (transmitters).",
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
                    name: "berr",
                    description: Some(
                        "RW0, Bus error flag.",
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
                    name: "arlo",
                    description: Some(
                        "RW0, Arbitration lost flag (master mode).",
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
                    name: "af",
                    description: Some(
                        "RW0, Acknowledge failure flag.",
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
                    name: "ovr",
                    description: Some(
                        "RW0, Overrun/Underrun flag.",
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
                    name: "pecerr",
                    description: Some(
                        "RW0, PEC Error flag in reception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timeout",
                    description: Some(
                        "RW0, Timeout or Tlow error flag.",
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
                    name: "smbalert",
                    description: Some(
                        "RW0, SMBus alert flag.",
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
            ],
        },
        FieldSet {
            name: "Star2",
            extends: None,
            description: Some(
                "RO, I2C status register 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "msl",
                    description: Some(
                        "RO, Mode statu: 0=Slave mode, 1=Master mode.",
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
                    name: "busy",
                    description: Some(
                        "RO, Bus busy flag.",
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
                    name: "tra",
                    description: Some(
                        "RO, Trans flag: 0=data bytes received, 1=data bytes transmitted.",
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
                    name: "gencall",
                    description: Some(
                        "RO, General call address (Slave mode) received flag.",
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
                    name: "smbdefault",
                    description: Some(
                        "RO, SMBus device default address (Slave mode) received flag.",
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
                    name: "smbhost",
                    description: Some(
                        "RO, SMBus host header (Slave mode) received flag.",
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
                    name: "dualf",
                    description: Some(
                        "RO, Dual flag (Slave mode): 0=Received address matched with OAR1, 1=Received address matched with OAR2.",
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
                    name: "pecx",
                    description: Some(
                        "RO, Packet error checking register.",
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
            ],
        },
    ],
    enums: &[],
};
                