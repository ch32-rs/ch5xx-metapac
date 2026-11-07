
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
                        "UART_FULL modem control.",
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
                        "UART_FULL interrupt enable.",
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
                        "UART_FULL FIFO control.",
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
                        "UART_FULL line control.",
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
                        "UART_FULL interrupt identification.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Iir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsr",
                    description: Some(
                        "UART_FULL line status.",
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
                        "UART_FULL modem status.",
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
                        "UART_FULL receiver buffer, receiving byte.",
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
                        "UART_FULL transmitter holding, transmittal byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "Thr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfc",
                    description: Some(
                        "UART_FULL receiver FIFO count.",
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
                        "UART_FULL transmitter FIFO count.",
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
                        "UART_FULL divisor latch.",
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
                        "UART_FULL pre-divisor latch byte.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DivFull",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adr",
                    description: Some(
                        "UART_FULL slave address.",
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
                        "UART_NOMODEM modem control.",
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
                        "UART_NOMODEM interrupt enable.",
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
                        "UART_NOMODEM FIFO control.",
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
                        "UART_NOMODEM line control.",
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
                        "UART_NOMODEM interrupt identification.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Iir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsr",
                    description: Some(
                        "UART_NOMODEM line status.",
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
                        "UART_NOMODEM receiver buffer, receiving byte.",
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
                        "UART_NOMODEM transmitter holding, transmittal byte.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "Thr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfc",
                    description: Some(
                        "UART_NOMODEM receiver FIFO count.",
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
                        "UART_NOMODEM transmitter FIFO count.",
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
                        "UART_NOMODEM divisor latch.",
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
                        "UART_NOMODEM pre-divisor latch byte.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DivNomodem",
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
                "UART_FULL slave address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "adr",
                    description: Some(
                        "UART_FULL slave address.",
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
            name: "DivFull",
            extends: None,
            description: Some(
                "UART_FULL pre-divisor latch byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "adr",
                    description: Some(
                        "UART pre-divisor latch byte.",
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
            name: "DivNomodem",
            extends: None,
            description: Some(
                "UART_NOMODEM pre-divisor latch byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "UART pre-divisor latch byte.",
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
                "UART_FULL divisor latch.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "dl",
                    description: Some(
                        "UART divisor latch.",
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
                "UART_FULL FIFO control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "fcr_fifo_en",
                    description: Some(
                        "UART FIFO enable.",
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
                        "clear UART receiver FIFO, high action, auto clear.",
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
                        "clear UART transmitter FIFO, high action, auto clear.",
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
                        "UART receiver FIFO trigger level.",
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
                "UART_FULL interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ier_recv_rdy",
                    description: Some(
                        "UART interrupt enable for receiver data ready.",
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
                        "UART interrupt enable for THR empty.",
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
                        "UART interrupt enable for receiver line status.",
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
                        "UART_FULL interrupt enable for modem status change.",
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
                        "UART_FULL DTR/TNOW output pin enable.",
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
                        "UART_FULL RTS output pin enable.",
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
                        "UART TXD pin enable.",
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
                        "UART software reset control, high action, auto clear.",
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
                "UART_NOMODEM interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ier_recv_rdy",
                    description: Some(
                        "UART interrupt enable for receiver data ready.",
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
                        "UART interrupt enable for THR empty.",
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
                        "UART interrupt enable for receiver line status.",
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
                        "UART TXD pin enable.",
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
                        "UART software reset control, high action, auto clear.",
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
            name: "Iir",
            extends: None,
            description: Some(
                "UART_FULL interrupt identification.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "iir_no_int",
                    description: Some(
                        "UART no interrupt flag.",
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
                    name: "iir_int_mask",
                    description: Some(
                        "UART interrupt flag bit mask.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iir_fifo_id",
                    description: Some(
                        "UART FIFO enabled flag.",
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
            name: "Lcr",
            extends: None,
            description: Some(
                "UART_FULL line control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "lcr_word_sz",
                    description: Some(
                        "UART word bit length.",
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
                        "UART stop bit length.",
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
                        "UART parity enable.",
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
                        "UART parity mode.",
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
                        "UART break control enable.",
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
                    name: "lcr_dlab_rb_lcr_gp_bit",
                    description: Some(
                        "UART reserved bit _UART general purpose bit.",
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
                "UART_FULL line status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "lsr_data_rdy",
                    description: Some(
                        "UART receiver fifo data ready status.",
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
                        "UART receiver overrun error.",
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
                        "UART receiver frame error.",
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
                        "UART receiver frame error.",
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
                        "UART receiver break error.",
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
                        "UART transmitter fifo empty status.",
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
                        "UART transmitter all empty status.",
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
                        "indicate error in UART receiver fifo.",
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
                "UART_FULL modem control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mcr_dtr",
                    description: Some(
                        "UART_FULL control DTR.",
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
                        "UART_FULL control RTS.",
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
                        "UART_FULL control OUT1.",
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
                    name: "mcr_out2",
                    description: Some(
                        "UART control OUT2.",
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
                        "UART_FULL enable local loop back.",
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
                        "UART_FULL enable autoflow control.",
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
                        "UART_FULL enable TNOW output on DTR pin.",
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
                        "UART_FULL enable half-duplex.",
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
                "UART_NOMODEM modem control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mcr_out2",
                    description: Some(
                        "UART_NOMODEM control OUT2.",
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
                "UART_FULL modem status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "msr_cts_chg",
                    description: Some(
                        "UART_FULL CTS changed status, high action.",
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
                        "UART_FULL DSR changed status, high action.",
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
                        "UART_FULL RI changed status, high action.",
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
                        "UART_FULL DCD changed status, high action.",
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
                        "UART_FULL CTS action status.",
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
                        "UART_FULL DSR action status.",
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
                        "UART_FULL RI action status.",
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
                        "UART_FULL DCD action status.",
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
                "UART receiver buffer, receiving byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rbr",
                    description: Some(
                        "UART receiver buffer, receiving byte.",
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
                "UART_FULL receiver FIFO count.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rfc",
                    description: Some(
                        "UART receiver FIFO count.",
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
                "UART_FULL transmitter FIFO count.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tfc",
                    description: Some(
                        "UART transmitter FIFO count.",
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
            name: "Thr",
            extends: None,
            description: Some(
                "UART transmitter holding, transmittal byte.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "thr",
                    description: Some(
                        "UART transmitter holding, transmittal byte.",
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
                