use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "UsbhsDev",
            extends: None,
            description: Some(
                "USB register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "base_mode",
                    description: Some(
                        "USB mode control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "BaseMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dev_ad",
                    description: Some(
                        "USB device address.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wake_ctrl",
                    description: Some(
                        "USB remote wake up register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "WakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "test_mode",
                    description: Some(
                        "USB test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "TestMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpm_data",
                    description: Some(
                        "USB power management register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "LpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB interrupt status.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fram_no",
                    description: Some(
                        "USB frame number.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "FramNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bus",
                    description: Some(
                        "USB bus.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Bus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_en",
                    description: Some(
                        "USB endpoint sends the enable register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTxEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_en",
                    description: Some(
                        "USB endpoint receive the enable registers.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRxEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_tog_auto",
                    description: Some(
                        "USB endpoint sends the auto-filp enable register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTTogAuto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_tog_auto",
                    description: Some(
                        "USB endpoint receive the auto-filp enable register.",
                    ),
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRTogAuto",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_burst",
                    description: Some(
                        "USB endpoint sends a burst register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTBurst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_burst_mode",
                    description: Some(
                        "USB endpoint send the mode register.",
                    ),
                    array: None,
                    byte_offset: 0x19,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTBurstMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_burst",
                    description: Some(
                        "USB endpoint receives the burst register.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRBurst",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_res_mode",
                    description: Some(
                        "USB endpoint reply mode register.",
                    ),
                    array: None,
                    byte_offset: 0x1b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRResMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_af_mode",
                    description: Some(
                        "USB endpoint muitiplexing register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepAfMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_dma",
                    description: Some(
                        "The start address register of the endpoint 0 buffer.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_dma",
                    description: Some(
                        "endpoint n (n=1..7) receives the start address register of the buffer. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepRxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_dma",
                    description: Some(
                        "endpoint n (n=1..7) sends the start address register of the buffer. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepTxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_max_len",
                    description: Some(
                        "endpoint 0 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_max_len",
                    description: Some(
                        "endpoint n (n=1..7) max length packet register. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UepMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_len",
                    description: Some(
                        "endpoint 0 acceptable length.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_len",
                    description: Some(
                        "endpoint n (n=1..7) receives the lenth register in a single pass. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint n (n=1..7). Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x82,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 send the length.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_tx_ctrl",
                    description: Some(
                        "endpoint 0 send control register.",
                    ),
                    array: None,
                    byte_offset: 0x9e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rx_ctrl",
                    description: Some(
                        "endpoint 0 send control register.",
                    ),
                    array: None,
                    byte_offset: 0x9f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_len",
                    description: Some(
                        "endpoint n (n=1..7) send the length register. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_ctrl",
                    description: Some(
                        "endpoint n (n=1..7) send control register. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_rx_ctrl",
                    description: Some(
                        "endpoint n (n=1..7) receive control. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0xa3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UepRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_t_iso",
                    description: Some(
                        "usb endpoint sends a synchronous mode enable register.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepTIso",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_r_iso",
                    description: Some(
                        "usb endpoint receives a synchronous mode enable register.",
                    ),
                    array: None,
                    byte_offset: 0xbe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepRIso",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "BaseMode",
            extends: None,
            description: Some(
                "USB mode control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "exp_spd",
                    description: Some(
                        "The desired speed mode of the device.",
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
            ],
        },
        FieldSet {
            name: "Bus",
            extends: None,
            description: Some(
                "USB bus.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "wakeup",
                    description: Some(
                        "USB wakeup.",
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
                    name: "dp_st",
                    description: Some(
                        "UDP status.",
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
                    name: "dm_st",
                    description: Some(
                        "UDM status.",
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
            name: "Ctrl",
            extends: None,
            description: Some(
                "USB base control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "link_reset",
                    description: Some(
                        "LINK layer reset,highly effective.",
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
                    name: "sie_reset",
                    description: Some(
                        "USB protocol processor reset.",
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
                    name: "all_clr",
                    description: Some(
                        "clear all interrupt flags.",
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
                    name: "phy_suspendm",
                    description: Some(
                        "USB PHY suspend.",
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
                    name: "dev_dma_en",
                    description: Some(
                        "DMA transfer enabled.",
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
                    name: "dev_en",
                    description: Some(
                        "USB device enabled.",
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
                    name: "dev_lpm_en",
                    description: Some(
                        "LPM enable.",
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
            name: "DevAd",
            extends: None,
            description: Some(
                "USB device address.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_usb_addr",
                    description: Some(
                        "bit mask for USB device address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FramNo",
            extends: None,
            description: Some(
                "USB frame number.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "frame_no",
                    description: Some(
                        "Received frame number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "micro_frame",
                    description: Some(
                        "Received micro frame number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEn",
            extends: None,
            description: Some(
                "USB interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rest_ie",
                    description: Some(
                        "USB bus reset interrupt enabled.",
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
                    name: "bus_susp_ie",
                    description: Some(
                        "USB bus pause interrupt enabled.",
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
                    name: "bus_sleep_ie",
                    description: Some(
                        "USB bus sleep interrupt enabled.",
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
                    name: "lpm_act_ie",
                    description: Some(
                        "LMP transfer end interrupt enabled.",
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
                    name: "rtx_act_ie",
                    description: Some(
                        "USB transfer end interrupt enabled.",
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
                    name: "rx_sof_ie",
                    description: Some(
                        "Receive SOF packet interrupt enable.",
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
                    name: "link_rdy_ie",
                    description: Some(
                        "USB connection interrupt enable.",
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
                    name: "fifo_over_ie",
                    description: Some(
                        "USB Overflow interrupt enable.",
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
            name: "IntFg",
            extends: None,
            description: Some(
                "USB interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uif_bus_rst",
                    description: Some(
                        "USB bus reset interrupt flag.",
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
                    name: "uif_suspend",
                    description: Some(
                        "USB bus suspend interrupt flag.",
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
                    name: "uif_bus_sleep",
                    description: Some(
                        "USB bus sleep interrupt flag.",
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
                    name: "uif_lpm_act",
                    description: Some(
                        "LPM transmission end interrupt flag.",
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
                    name: "uif_rtx_act",
                    description: Some(
                        "USB transmission end interrupt flag.",
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
                    name: "uif_rx_sof",
                    description: Some(
                        "Receive SOF packet interrupt flag.",
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
                    name: "uif_link_rdy",
                    description: Some(
                        "USB connection interrupt flag.",
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
                    name: "uif_fifo_ov",
                    description: Some(
                        "USB Overflow interrupt flag.",
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
            name: "IntSt",
            extends: None,
            description: Some(
                "USB interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uis_ep_id_mask",
                    description: Some(
                        "The endpoint number at which the data transfer occurs.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uis_ep_dir",
                    description: Some(
                        "Endpoint data transmission direction.",
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
            ],
        },
        FieldSet {
            name: "LpmData",
            extends: None,
            description: Some(
                "USB power management register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "lpm_data",
                    description: Some(
                        "power management data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpm_busy",
                    description: Some(
                        "power management busy.",
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
            name: "MisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ums_ready",
                    description: Some(
                        "USB connection status.",
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
                    name: "ums_suspend",
                    description: Some(
                        "USB suspend status.",
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
                    name: "ums_sleep",
                    description: Some(
                        "USB sleep status.",
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
                    name: "ums_free",
                    description: Some(
                        "USB free status.",
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
                    name: "ums_susp_req",
                    description: Some(
                        "USB suspends the request.",
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
                    name: "ums_hs_mod",
                    description: Some(
                        "whether the host is high-speed.",
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
            name: "TestMode",
            extends: None,
            description: Some(
                "USB test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "test_j",
                    description: Some(
                        "test mode,output J.",
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
                    name: "test_k",
                    description: Some(
                        "test mode,output K.",
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
                    name: "test_pkt",
                    description: Some(
                        "test mode,output a packet.",
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
                    name: "test_se0nak",
                    description: Some(
                        "test mode,output SEO.",
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
                    name: "test_en",
                    description: Some(
                        "test mode enable.",
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
            name: "Uep0Dma",
            extends: None,
            description: Some(
                "The start address register of the endpoint 0 buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep0_dma",
                    description: Some(
                        "The start address of the endpoint 0 buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep0MaxLen",
            extends: None,
            description: Some(
                "endpoint 0 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep0_max_len",
                    description: Some(
                        "endpoint 0 max acceptable offset length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep0RxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 0 has control over the received response.",
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
                    name: "uep_tog_mask",
                    description: Some(
                        "the reception of endpoint 0 expects a synchronous trigger. bit.",
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
                    name: "setup_is",
                    description: Some(
                        "whether endpoint 0 receives a SETUP transaction.",
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
                    name: "endp_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint 0 for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint 0 receives the end of NAK flag.",
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
                    name: "endp_r_done",
                    description: Some(
                        "endpoint 0 receives the end flag.",
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
            name: "Uep0RxLen",
            extends: None,
            description: Some(
                "endpoint 0 acceptable length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_rx_len",
                    description: Some(
                        "endpoint 0 acceptable data length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep0TLen",
            extends: None,
            description: Some(
                "endpoint 0 send the length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 send the length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep0TxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 0 control of the send response to IN transactions.",
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
                    name: "ep_tog_mask",
                    description: Some(
                        "endpoint 0 synchronous trigger bit for the sender to. prepare.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint 0 sends the end flag.",
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
                    name: "endp_t_done",
                    description: Some(
                        "endpoint 0 sends the end of NAK flag.",
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
            name: "UepAfMode",
            extends: None,
            description: Some(
                "USB endpoint muitiplexing register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ep_t_af",
                    description: Some(
                        "1 to 7 endpoint muitiplexing enables.",
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
            name: "UepMaxLen",
            extends: None,
            description: Some(
                "endpoint n max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_max_len",
                    description: Some(
                        "endpoint n max acceptable offset length.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRBurst",
            extends: None,
            description: Some(
                "USB endpoint receives the burst register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_burst_en",
                    description: Some(
                        "0 to 7 endpoint receive burst enable.",
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
            name: "UepRIso",
            extends: None,
            description: Some(
                "usb endpoint receives a synchronous mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_iso",
                    description: Some(
                        "down endpoint(OUT) synchronization mode enabled.",
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
            name: "UepRResMode",
            extends: None,
            description: Some(
                "USB endpoint reply mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mode",
                    description: Some(
                        "0 to 7 endpoint reply mode.",
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
            name: "UepRSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint n.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_size",
                    description: Some(
                        "the length of the total received data at endpoint n.",
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
            name: "UepRTogAuto",
            extends: None,
            description: Some(
                "USB endpoint receive the auto-filp enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_tog_auto",
                    description: Some(
                        "0 to 7 endpoint synchronization triggers bit auto-filp. enable.",
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
            name: "UepRxCtrl",
            extends: None,
            description: Some(
                "endpoint n receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint n has control over the received response.",
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
                    name: "uep_tog_mask",
                    description: Some(
                        "the reception of endpoint n expects a synchronous trigger. bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "endp_r_tog_match",
                    description: Some(
                        "received synchronization trigger bit matches the desired. synchronization trigger bit state.",
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
                    name: "uep_r_nak_tog",
                    description: Some(
                        "endpoint n for the received return NAK,packet type.",
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
                    name: "uep_r_nak_act",
                    description: Some(
                        "endpoint n receives the end of NAK flag.",
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
                    name: "endp_r_done",
                    description: Some(
                        "endpoint n receives the end flag.",
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
            name: "UepRxDma",
            extends: None,
            description: Some(
                "endpoint n receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_dma",
                    description: Some(
                        "endpoint receives the start address register of the buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepRxEn",
            extends: None,
            description: Some(
                "USB endpoint receive the enable registers.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_rx_en",
                    description: Some(
                        "Endpoint 0 to 15 receive enable.",
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
            name: "UepRxLen",
            extends: None,
            description: Some(
                "endpoint n receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_rx_len",
                    description: Some(
                        "the length of data received by endpoint n in a single pass.",
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
            name: "UepTBurst",
            extends: None,
            description: Some(
                "USB endpoint sends a burst register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_en",
                    description: Some(
                        "0 to 7 endpoint send burst enable.",
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
            name: "UepTBurstMode",
            extends: None,
            description: Some(
                "USB endpoint send the mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_en",
                    description: Some(
                        "0 to 7 endpoint send the mode enable.",
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
            name: "UepTIso",
            extends: None,
            description: Some(
                "usb endpoint sends a synchronous mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_iso",
                    description: Some(
                        "upload endpoint(IN) synchronization mode enabled.",
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
            name: "UepTLen",
            extends: None,
            description: Some(
                "endpoint n send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_len",
                    description: Some(
                        "endpoint n send the length.",
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
            name: "UepTTogAuto",
            extends: None,
            description: Some(
                "USB endpoint sends the auto-filp enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_tog_auto",
                    description: Some(
                        "0 to 7 endpoint synchronization triggers bit auto-filp. enable.",
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
            name: "UepTxCtrl",
            extends: None,
            description: Some(
                "endpoint n send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint n control of the send response to IN transactions.",
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
                    name: "uep_tog_mask",
                    description: Some(
                        "endpoint n synchronous trigger bit for the sender to. prepare.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_nak_act",
                    description: Some(
                        "endpoint n sends the end flag.",
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
                    name: "endp_t_done",
                    description: Some(
                        "endpoint n sends the end of NAK flag.",
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
            name: "UepTxDma",
            extends: None,
            description: Some(
                "endpoint n sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_dma",
                    description: Some(
                        "endpoint sends the start address register of the buffer.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UepTxEn",
            extends: None,
            description: Some(
                "USB endpoint sends the enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_en",
                    description: Some(
                        "Endpoint 0 to 15 sends enable.",
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
            name: "WakeCtrl",
            extends: None,
            description: Some(
                "USB remote wake up register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rmt_wake",
                    description: Some(
                        "remote wake up.",
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
