
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
                    name: "uep1_rx_dma",
                    description: Some(
                        "endpoint 1 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_dma",
                    description: Some(
                        "endpoint 2 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_dma",
                    description: Some(
                        "endpoint 3 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_dma",
                    description: Some(
                        "endpoint 4 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_dma",
                    description: Some(
                        "endpoint 5 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_dma",
                    description: Some(
                        "endpoint 6 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_dma",
                    description: Some(
                        "endpoint 7 receives the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_dma",
                    description: Some(
                        "endpoint 1 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_dma",
                    description: Some(
                        "endpoint 2 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_dma",
                    description: Some(
                        "endpoint 3 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_dma",
                    description: Some(
                        "endpoint 4 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_dma",
                    description: Some(
                        "endpoint 5 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_dma",
                    description: Some(
                        "endpoint 6 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_dma",
                    description: Some(
                        "endpoint 7 sends the start address register of the buffer.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7TxDma",
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
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep1MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_max_len",
                    description: Some(
                        "endpoint 2 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep2MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep3MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep4MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep5MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep6MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 max length packet register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep7MaxLen",
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
                    name: "uep1_rx_len",
                    description: Some(
                        "endpoint 1 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 1.",
                    ),
                    array: None,
                    byte_offset: 0x82,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_len",
                    description: Some(
                        "endpoint 2 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 2.",
                    ),
                    array: None,
                    byte_offset: 0x86,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_len",
                    description: Some(
                        "endpoint 3 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 3.",
                    ),
                    array: None,
                    byte_offset: 0x8a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_len",
                    description: Some(
                        "endpoint 4 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 4.",
                    ),
                    array: None,
                    byte_offset: 0x8e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_len",
                    description: Some(
                        "endpoint 5 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 5.",
                    ),
                    array: None,
                    byte_offset: 0x92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_len",
                    description: Some(
                        "endpoint 6 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 6.",
                    ),
                    array: None,
                    byte_offset: 0x96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6RSize",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_len",
                    description: Some(
                        "endpoint 7 receives the lenth register in a single pass.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_r_size",
                    description: Some(
                        "the length register of the total received data at endpoint 7.",
                    ),
                    array: None,
                    byte_offset: 0x9a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7RSize",
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
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_tx_ctrl",
                    description: Some(
                        "endpoint 1 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xa2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_ctrl",
                    description: Some(
                        "endpoint 1 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xa3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_t_len",
                    description: Some(
                        "endpoint 2 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_ctrl",
                    description: Some(
                        "endpoint 2 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xa6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_rx_ctrl",
                    description: Some(
                        "endpoint 2 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xa7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_len",
                    description: Some(
                        "endpoint 3 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_ctrl",
                    description: Some(
                        "endpoint 3 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xaa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_ctrl",
                    description: Some(
                        "endpoint 3 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xab,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_ctrl",
                    description: Some(
                        "endpoint 4 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xae,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_rx_ctrl",
                    description: Some(
                        "endpoint 4 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xaf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_t_len",
                    description: Some(
                        "endpoint 5 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_tx_ctrl",
                    description: Some(
                        "endpoint 5 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xb2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_rx_ctrl",
                    description: Some(
                        "endpoint 5 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xb3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_t_len",
                    description: Some(
                        "endpoint 6 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_tx_ctrl",
                    description: Some(
                        "endpoint 6 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xb6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_rx_ctrl",
                    description: Some(
                        "endpoint 6 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xb7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6RxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_t_len",
                    description: Some(
                        "endpoint 7 send the length register.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_tx_ctrl",
                    description: Some(
                        "endpoint 7 send control register.",
                    ),
                    array: None,
                    byte_offset: 0xba,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_rx_ctrl",
                    description: Some(
                        "endpoint 7 receive control.",
                    ),
                    array: None,
                    byte_offset: 0xbb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7RxCtrl",
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
            name: "Uep1MaxLen",
            extends: None,
            description: Some(
                "endpoint 1 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 max acceptable offset length.",
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
            name: "Uep1RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 1.",
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
            name: "Uep1RxCtrl",
            extends: None,
            description: Some(
                "endpoint 1 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 1 has control over the received response.",
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
                        "the reception of endpoint 1 expects a synchronous trigger. bit.",
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
                        "endpoint 1 for the received return NAK,packet type.",
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
                        "endpoint 1 receives the end of NAK flag.",
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
                        "endpoint 1 receives the end flag.",
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
            name: "Uep1RxDma",
            extends: None,
            description: Some(
                "endpoint 1 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_rx_dma",
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
            name: "Uep1RxLen",
            extends: None,
            description: Some(
                "endpoint 1 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_rx_len",
                    description: Some(
                        "the length of data received by endpoint 1 in a single pass.",
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
            name: "Uep1TLen",
            extends: None,
            description: Some(
                "endpoint 1 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 send the length.",
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
            name: "Uep1TxCtrl",
            extends: None,
            description: Some(
                "endpoint 1 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 1 control of the send response to IN transactions.",
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
                        "endpoint 1 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 1 sends the end flag.",
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
                        "endpoint 1 sends the end of NAK flag.",
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
            name: "Uep1TxDma",
            extends: None,
            description: Some(
                "endpoint 1 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_tx_dma",
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
            name: "Uep2MaxLen",
            extends: None,
            description: Some(
                "endpoint 2 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_max_len",
                    description: Some(
                        "endpoint 2 max acceptable offset length.",
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
            name: "Uep2RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 2.",
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
            name: "Uep2RxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 2 has control over the received response.",
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
                        "the reception of endpoint 2 expects a synchronous trigger. bit.",
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
                        "endpoint 2 for the received return NAK,packet type.",
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
                        "endpoint 2 receives the end of NAK flag.",
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
                        "endpoint 2 receives the end flag.",
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
            name: "Uep2RxDma",
            extends: None,
            description: Some(
                "endpoint 2 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_rx_dma",
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
            name: "Uep2RxLen",
            extends: None,
            description: Some(
                "endpoint 2 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_rx_len",
                    description: Some(
                        "the length of data received by endpoint 2 in a single pass.",
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
            name: "Uep2TLen",
            extends: None,
            description: Some(
                "endpoint 2 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 2 send the length.",
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
            name: "Uep2TxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 2 control of the send response to IN transactions.",
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
                        "endpoint 2 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 2 sends the end flag.",
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
                        "endpoint 2 sends the end of NAK flag.",
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
            name: "Uep2TxDma",
            extends: None,
            description: Some(
                "endpoint 2 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_tx_dma",
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
            name: "Uep3MaxLen",
            extends: None,
            description: Some(
                "endpoint 3 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 max acceptable offset length.",
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
            name: "Uep3RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 3.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 3.",
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
            name: "Uep3RxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 3 has control over the received response.",
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
                        "the reception of endpoint 3 expects a synchronous trigger. bit.",
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
                        "endpoint 3 for the received return NAK,packet type.",
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
                        "endpoint 3 receives the end of NAK flag.",
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
                        "endpoint 3 receives the end flag.",
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
            name: "Uep3RxDma",
            extends: None,
            description: Some(
                "endpoint 3 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_rx_dma",
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
            name: "Uep3RxLen",
            extends: None,
            description: Some(
                "endpoint 3 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_rx_len",
                    description: Some(
                        "the length of data received by endpoint 3 in a single pass.",
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
            name: "Uep3TLen",
            extends: None,
            description: Some(
                "endpoint 3 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 3 send the length.",
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
            name: "Uep3TxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 3 control of the send response to IN transactions.",
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
                        "endpoint 3 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 3 sends the end flag.",
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
                        "endpoint 3 sends the end of NAK flag.",
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
            name: "Uep3TxDma",
            extends: None,
            description: Some(
                "endpoint 3 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_tx_dma",
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
            name: "Uep4MaxLen",
            extends: None,
            description: Some(
                "endpoint 4 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 max acceptable offset length.",
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
            name: "Uep4RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 4.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 4.",
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
            name: "Uep4RxCtrl",
            extends: None,
            description: Some(
                "endpoint 4 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 4 has control over the received response.",
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
                        "the reception of endpoint 4 expects a synchronous trigger. bit.",
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
                        "endpoint 4 for the received return NAK,packet type.",
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
                        "endpoint 4 receives the end of NAK flag.",
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
                        "endpoint 4 receives the end flag.",
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
            name: "Uep4RxDma",
            extends: None,
            description: Some(
                "endpoint 4 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_rx_dma",
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
            name: "Uep4RxLen",
            extends: None,
            description: Some(
                "endpoint 4 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_rx_len",
                    description: Some(
                        "the length of data received by endpoint 4 in a single pass.",
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
            name: "Uep4TLen",
            extends: None,
            description: Some(
                "endpoint 4 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 4 send the length.",
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
            name: "Uep4TxCtrl",
            extends: None,
            description: Some(
                "endpoint 4 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 4 control of the send response to IN transactions.",
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
                        "endpoint 4 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 4 sends the end flag.",
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
                        "endpoint 4 sends the end of NAK flag.",
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
            name: "Uep4TxDma",
            extends: None,
            description: Some(
                "endpoint 4 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_tx_dma",
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
            name: "Uep5MaxLen",
            extends: None,
            description: Some(
                "endpoint 5 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 max acceptable offset length.",
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
            name: "Uep5RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 5.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 5.",
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
            name: "Uep5RxCtrl",
            extends: None,
            description: Some(
                "endpoint 5 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 5 has control over the received response.",
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
                        "the reception of endpoint 5 expects a synchronous trigger. bit.",
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
                        "endpoint 5 for the received return NAK,packet type.",
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
                        "endpoint 5 receives the end of NAK flag.",
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
                        "endpoint 5 receives the end flag.",
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
            name: "Uep5RxDma",
            extends: None,
            description: Some(
                "endpoint 5 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_rx_dma",
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
            name: "Uep5RxLen",
            extends: None,
            description: Some(
                "endpoint 5 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_rx_len",
                    description: Some(
                        "the length of data received by endpoint 5 in a single pass.",
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
            name: "Uep5TLen",
            extends: None,
            description: Some(
                "endpoint 5 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 5 send the length.",
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
            name: "Uep5TxCtrl",
            extends: None,
            description: Some(
                "endpoint 5 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 5 control of the send response to IN transactions.",
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
                        "endpoint 5 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 5 sends the end flag.",
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
                        "endpoint 5 sends the end of NAK flag.",
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
            name: "Uep5TxDma",
            extends: None,
            description: Some(
                "endpoint 5 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_tx_dma",
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
            name: "Uep6MaxLen",
            extends: None,
            description: Some(
                "endpoint 6 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 max acceptable offset length.",
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
            name: "Uep6RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 6.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 6.",
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
            name: "Uep6RxCtrl",
            extends: None,
            description: Some(
                "endpoint 6 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 6 has control over the received response.",
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
                        "the reception of endpoint 6 expects a synchronous trigger. bit.",
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
                        "endpoint 6 for the received return NAK,packet type.",
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
                        "endpoint 6 receives the end of NAK flag.",
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
                        "endpoint 6 receives the end flag.",
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
            name: "Uep6RxDma",
            extends: None,
            description: Some(
                "endpoint 6 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_rx_dma",
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
            name: "Uep6RxLen",
            extends: None,
            description: Some(
                "endpoint 6 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_rx_len",
                    description: Some(
                        "the length of data received by endpoint 6 in a single pass.",
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
            name: "Uep6TLen",
            extends: None,
            description: Some(
                "endpoint 6 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 6 send the length.",
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
            name: "Uep6TxCtrl",
            extends: None,
            description: Some(
                "endpoint 6 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 6 control of the send response to IN transactions.",
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
                        "endpoint 6 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 6 sends the end flag.",
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
                        "endpoint 6 sends the end of NAK flag.",
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
            name: "Uep6TxDma",
            extends: None,
            description: Some(
                "endpoint 6 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_tx_dma",
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
            name: "Uep7MaxLen",
            extends: None,
            description: Some(
                "endpoint 7 max length packet register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 max acceptable offset length.",
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
            name: "Uep7RSize",
            extends: None,
            description: Some(
                "the length register of the total received data at endpoint 7.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_r_size",
                    description: Some(
                        "the length of the total received data at endpoint 7.",
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
            name: "Uep7RxCtrl",
            extends: None,
            description: Some(
                "endpoint 7 receive control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 7 has control over the received response.",
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
                        "the reception of endpoint 7 expects a synchronous trigger. bit.",
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
                        "endpoint 7 for the received return NAK,packet type.",
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
                        "endpoint 7 receives the end of NAK flag.",
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
                        "endpoint 7 receives the end flag.",
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
            name: "Uep7RxDma",
            extends: None,
            description: Some(
                "endpoint 7 receives the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_rx_dma",
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
            name: "Uep7RxLen",
            extends: None,
            description: Some(
                "endpoint 7 receives the lenth register in a single pass.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_rx_len",
                    description: Some(
                        "the length of data received by endpoint 7 in a single pass.",
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
            name: "Uep7TLen",
            extends: None,
            description: Some(
                "endpoint 7 send the length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_len",
                    description: Some(
                        "endpoint 7 send the length.",
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
            name: "Uep7TxCtrl",
            extends: None,
            description: Some(
                "endpoint 7 send control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask",
                    description: Some(
                        "endpoint 7 control of the send response to IN transactions.",
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
                        "endpoint 7 synchronous trigger bit for the sender to. prepare.",
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
                        "endpoint 7 sends the end flag.",
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
                        "endpoint 7 sends the end of NAK flag.",
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
            name: "Uep7TxDma",
            extends: None,
            description: Some(
                "endpoint 7 sends the start address register of the buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_tx_dma",
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
                