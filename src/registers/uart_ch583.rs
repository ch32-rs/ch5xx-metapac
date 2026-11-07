
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "UartFull",
            extends: None,
            description: Some(
                "UART_FULL register.",
            ),
            items: &[
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "RW, UART_FULL modem control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "McrFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "RW, UART_FULL interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IerFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fcr",
                    description: Some(
                        "RW, UART_FULL FIFO control.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcr",
                    description: Some(
                        "RW, UART_FULL line control.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Lcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iir",
                    description: Some(
                        "RO, UART_FULL interrupt identification.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IirFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsr",
                    description: Some(
                        "RO, UART_FULL line status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Lsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msr",
                    description: Some(
                        "RO, UART_FULL modem status.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Msr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rbr",
                    description: Some(
                        "RO, UART_FULL receiver buffer, receiving byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Rbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "thr",
                    description: Some(
                        "WO, UART_FULL transmitter holding, transmittal byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "ThrFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfc",
                    description: Some(
                        "RO, UART_FULL receiver FIFO count.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Rfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tfc",
                    description: Some(
                        "RO, UART_FULL transmitter FIFO count.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Tfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dl",
                    description: Some(
                        "RW, UART_FULL divisor latch.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "div",
                    description: Some(
                        "RW, UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Div",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adr",
                    description: Some(
                        "RW, UART_FULL slave address: 0xFF=disable, other=enable.",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Adr",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "UartNomodem",
            extends: None,
            description: Some(
                "UART_NOMODEM register.",
            ),
            items: &[
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "RW, UART_NOMODEM modem control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "McrNomodem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "RW, UART_NOMODEM interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IerNomodem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fcr",
                    description: Some(
                        "RW, UART_NOMODEM FIFO control.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Fcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcr",
                    description: Some(
                        "RW, UART_NOMODEM line control.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Lcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iir",
                    description: Some(
                        "RO, UART_NOMODEM interrupt identification.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IirNomodem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsr",
                    description: Some(
                        "RO, UART_NOMODEM line status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Lsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rbr",
                    description: Some(
                        "RO, UART_NOMODEM receiver buffer, receiving byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Rbr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "thr",
                    description: Some(
                        "WO, UART_NOMODEM transmitter holding, transmittal byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "ThrNomodem",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfc",
                    description: Some(
                        "RO, UART_NOMODEM receiver FIFO count.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Rfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tfc",
                    description: Some(
                        "RO, UART_NOMODEM transmitter FIFO count.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Tfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dl",
                    description: Some(
                        "RW, UART_NOMODEM divisor latch.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Dl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "div",
                    description: Some(
                        "RW, UART_NOMODEM pre-divisor latch byte, only low 7 bit, from 1 to 128.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Div",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adr",
            extends: None,
            description: Some(
                "RW, UART_FULL slave address: 0xFF=disable, other=enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "adr",
                    description: Some(
                        "RW,UART_FULL slave address: 0xFF=disable, other=enable.",
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
            name: "Div",
            extends: None,
            description: Some(
                "RW, UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "RW,UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128.",
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
            name: "Dl",
            extends: None,
            description: Some(
                "RW, UART_FULL divisor latch.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dl",
                    description: Some(
                        "RW, UART_FULL divisor latch.",
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
            name: "Fcr",
            extends: None,
            description: Some(
                "RW, UART_FULL FIFO control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fcr_fifo_en",
                    description: Some(
                        "RW, UART FIFO enable.",
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
                    name: "fcr_rx_fifo_clr",
                    description: Some(
                        "WZ, clear UART receiver FIFO, high action, auto clear.",
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
                    name: "fcr_tx_fifo_clr",
                    description: Some(
                        "WZ, clear UART transmitter FIFO, high action, auto clear.",
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
                    name: "fcr_fifo_trig",
                    description: Some(
                        "RW, UART receiver FIFO trigger level: 00-1byte, 01-2bytes, 10-4bytes, 11-7bytes.",
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
            ],
        },
        FieldSet {
            name: "IerFull",
            extends: None,
            description: Some(
                "RW, UART_FULL interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ier_recv_rdy",
                    description: Some(
                        "RW, UART interrupt enable for receiver data ready.",
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
                    name: "ier_thr_empty",
                    description: Some(
                        "RW, UART interrupt enable for THR empty.",
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
                    name: "ier_line_stat",
                    description: Some(
                        "RW, UART interrupt enable for receiver line status.",
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
                    name: "ier_modem_chg",
                    description: Some(
                        "RW, UART_FULL interrupt enable for modem status change.",
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
                    name: "ier_dtr_en",
                    description: Some(
                        "RW, UART_FULL DTR/TNOW output pin enable.",
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
                    name: "ier_rts_en",
                    description: Some(
                        "RW, UART_FULL RTS output pin enable.",
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
                    name: "ier_txd_en",
                    description: Some(
                        "RW, UART TXD pin enable.",
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
                    name: "ier_reset",
                    description: Some(
                        "WZ, UART software reset control, high action, auto clear.",
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
            ],
        },
        FieldSet {
            name: "IerNomodem",
            extends: None,
            description: Some(
                "RW, UART_NOMODEM interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ier_recv_rdy",
                    description: Some(
                        "RW, UART interrupt enable for receiver data ready.",
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
                    name: "ier_thr_empty",
                    description: Some(
                        "RW, UART interrupt enable for THR empty.",
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
                    name: "ier_line_stat",
                    description: Some(
                        "RW, UART interrupt enable for receiver line status.",
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
                    name: "ier_txd_en",
                    description: Some(
                        "RW, UART TXD pin enable.",
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
                    name: "ier_reset",
                    description: Some(
                        "WZ, UART software reset control, high action, auto clear.",
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
            ],
        },
        FieldSet {
            name: "IirFull",
            extends: None,
            description: Some(
                "RO, UART_FULL interrupt identification.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "iir_int_mask",
                    description: Some(
                        "RO, UART interrupt flag bit mask.",
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
                    name: "iir_no_int",
                    description: Some(
                        "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt.",
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
                    name: "iir_fifo_id",
                    description: Some(
                        "RO, UART FIFO enabled flag.",
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
            ],
        },
        FieldSet {
            name: "IirNomodem",
            extends: None,
            description: Some(
                "RO, UART_NOMODEM interrupt identification.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "iir_int_mask",
                    description: Some(
                        "RO, UART interrupt flag bit mask.",
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
                    name: "iir_no_int",
                    description: Some(
                        "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt.",
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
                    name: "iir_fifo_id",
                    description: Some(
                        "RO, UART FIFO enabled flag.",
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
            ],
        },
        FieldSet {
            name: "Lcr",
            extends: None,
            description: Some(
                "RW, UART_FULL line control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "lcr_word_sz",
                    description: Some(
                        "RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lcr_stop_bit",
                    description: Some(
                        "RW, UART stop bit length: 0-1bit, 1-2bit.",
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
                    name: "lcr_par_en",
                    description: Some(
                        "RW, UART parity enable.",
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
                    name: "lcr_par_mod",
                    description: Some(
                        "RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lcr_break_en",
                    description: Some(
                        "RW, UART break control enable.",
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
                    name: "lcr_dlab",
                    description: Some(
                        "RW, UART divisor latch access bit.",
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
                    name: "lcr_gp_bit",
                    description: Some(
                        "RW, UART general purpose bit.",
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
            ],
        },
        FieldSet {
            name: "Lsr",
            extends: None,
            description: Some(
                "RO, UART_FULL line status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "lsr_data_rdy",
                    description: Some(
                        "RO, UART receiver fifo data ready status.",
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
                    name: "lsr_over_err",
                    description: Some(
                        "RZ, UART receiver overrun error.",
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
                    name: "lsr_par_err",
                    description: Some(
                        "RZ, UART receiver parity error.",
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
                    name: "lsr_frame_err",
                    description: Some(
                        "RZ, UART receiver frame error.",
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
                    name: "lsr_break_err",
                    description: Some(
                        "RZ, UART receiver break error.",
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
                    name: "lsr_tx_fifo_emp",
                    description: Some(
                        "RO, UART transmitter fifo empty status.",
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
                    name: "lsr_tx_all_emp",
                    description: Some(
                        "RO, UART transmitter all empty status.",
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
                    name: "lsr_err_rx_fifo",
                    description: Some(
                        "RO, indicate error in UART receiver fifo.",
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
            ],
        },
        FieldSet {
            name: "McrFull",
            extends: None,
            description: Some(
                "RW, UART_FULL modem control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mcr_dtr",
                    description: Some(
                        "RW, UART_FULL control DTR.",
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
                    name: "mcr_rts",
                    description: Some(
                        "RW, UART_FULL control RTS.",
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
                    name: "mcr_out1",
                    description: Some(
                        "RW, UART_FULL control OUT1.",
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
                    name: "mcr_int_oe",
                    description: Some(
                        "RW, UART interrupt output enable.",
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
                    name: "mcr_out2",
                    description: Some(
                        "RW, UART control OUT2.",
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
                    name: "mcr_loop",
                    description: Some(
                        "RW, UART_FULL enable local loop back.",
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
                    name: "mcr_au_flow_en",
                    description: Some(
                        "RW, UART_FULL enable autoflow control.",
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
                    name: "mcr_tnow",
                    description: Some(
                        "RW, UART_FULL enable TNOW output on DTR pin.",
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
                    name: "mcr_half",
                    description: Some(
                        "RW, UART_FULL enable half-duplex.",
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
            ],
        },
        FieldSet {
            name: "McrNomodem",
            extends: None,
            description: Some(
                "RW, UART_NOMODEM modem control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mcr_int_oe",
                    description: Some(
                        "RW, UART interrupt output enable.",
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
                    name: "mcr_out2",
                    description: Some(
                        "RW, UART control OUT2.",
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
            ],
        },
        FieldSet {
            name: "Msr",
            extends: None,
            description: Some(
                "RO, UART_FULL modem status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "msr_cts_chg",
                    description: Some(
                        "RZ, UART_FULL CTS changed status, high action.",
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
                    name: "msr_dsr_chg",
                    description: Some(
                        "RZ, UART_FULL DSR changed status, high action.",
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
                    name: "msr_ri_chg",
                    description: Some(
                        "RZ, UART_FULL RI changed status, high action.",
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
                    name: "msr_dcd_chg",
                    description: Some(
                        "RZ, UART_FULL DCD changed status, high action.",
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
                    name: "msr_cts",
                    description: Some(
                        "RO, UART_FULL CTS action status.",
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
                    name: "msr_dsr",
                    description: Some(
                        "RO, UART_FULL DSR action statusv.",
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
                    name: "msr_ri",
                    description: Some(
                        "RO, UART_FULL RI action status.",
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
                    name: "msr_dcd",
                    description: Some(
                        "RO, UART_FULL DCD action status.",
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
            ],
        },
        FieldSet {
            name: "Rbr",
            extends: None,
            description: Some(
                "RO, UART_FULL receiver buffer, receiving byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rbr",
                    description: Some(
                        "RO, UART_FULL receiver buffer, receiving byte.",
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
            name: "Rfc",
            extends: None,
            description: Some(
                "RO, UART_FULL receiver FIFO count.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rfc",
                    description: Some(
                        "RO, UART_FULL receiver FIFO count.",
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
            name: "Tfc",
            extends: None,
            description: Some(
                "RO, UART_FULL transmitter FIFO count.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tfc",
                    description: Some(
                        "RO, UART_FULL transmitter FIFO count.",
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
            name: "ThrFull",
            extends: None,
            description: Some(
                "WO, UART_FULL transmitter holding, transmittal byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "thr",
                    description: Some(
                        "RO, UART_FULL transmitter holding, transmittal byte.",
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
            name: "ThrNomodem",
            extends: None,
            description: Some(
                "WO, UART_NOMODEM transmitter holding, transmittal byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rbr",
                    description: Some(
                        "WO, UART_NOMODEM transmitter holding, transmittal byte.",
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
    ],
    enums: &[],
};
                