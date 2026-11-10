use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbhs",
            extends: None,
            description: Some(
                "USBHS register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "USB base control.",
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
                    name: "uhost_ctrl",
                    description: Some(
                        "USB host control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhostCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB interrupt enable.",
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
                    name: "frame_no",
                    description: Some(
                        "USB frame number register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "FrameNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "suspend",
                    description: Some(
                        "USB suspend register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Suspend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "spd_type",
                    description: Some(
                        "USB actual speed register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SpdType",
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
                    byte_offset: 0x9,
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
                    name: "int_fg",
                    description: Some(
                        "USB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0xa,
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
                    byte_offset: 0xb,
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
                    name: "r6_usb_rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "R6UsbRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_1_mod",
                    description: Some(
                        "endpoint 1(9) 4(8,12) mode.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep41Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_3_mod",
                    description: Some(
                        "Endpoint 2 and 3 device mode control.",
                    ),
                    array: None,
                    byte_offset: 0x11,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep23Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_ep_mod",
                    description: Some(
                        "Host endpoint mode control.",
                    ),
                    array: None,
                    byte_offset: 0x11,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhEpMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_6_mod",
                    description: Some(
                        "endpoint 5(13) 6(14) mode.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep56Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_mod",
                    description: Some(
                        "endpoint 7(15) mode.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_rt_dma",
                    description: Some(
                        "endpoint 0 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Uep0RtDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_rx_dma",
                    description: Some(
                        "endpoint 1 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "uep_rx_dma",
                    description: Some(
                        "Device endpoint receive DMA address. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x18,
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
                    name: "uep2_rx_dma",
                    description: Some(
                        "Device uep2 rx dma register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "uh_rx_dma",
                    description: Some(
                        "Host uh rx dma register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_dma",
                    description: Some(
                        "endpoint 3 DMA buffer address;host tx endpoint buffer high address.",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                        "endpoint 4 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                        "endpoint 5 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x28,
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
                        "endpoint 6 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                        "endpoint 7 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                        "endpoint 1 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                    name: "uep_tx_dma",
                    description: Some(
                        "Device endpoint transmit DMA address. Array index 0..6 maps to endpoint 1..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 7,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x34,
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
                    name: "uep2_tx_dma",
                    description: Some(
                        "endpoint 2 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x38,
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
                        "Device uep3 tx dma register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
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
                    name: "uh_tx_dma",
                    description: Some(
                        "Host uh tx dma register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_dma",
                    description: Some(
                        "endpoint 4 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                        "endpoint 5 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                        "endpoint 4 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                        "endpoint 7 DMA TX buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                        "endpoint 0 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_max_len",
                    description: Some(
                        "Device endpoint maximum receive length. Array index 0..7 maps to endpoint 0..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UepMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_max_len",
                    description: Some(
                        "Device uep2 max len register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_max_len",
                    description: Some(
                        "Host uh rx max len register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhRxMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep4MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 receive max length.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7MaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x70,
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
                    name: "uep_t_len",
                    description: Some(
                        "Device endpoint transmit length. Array index 0..7 maps to endpoint 0..7.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x70,
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
                    name: "uep0_tx_ctrl",
                    description: Some(
                        "endpoint 0 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x72,
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
                        "endpoint 0 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x73,
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
                        "endpoint 1 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x74,
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
                        "endpoint 1 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x76,
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
                        "endpoint 1 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x77,
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
                        "Device uep2 t len register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
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
                    name: "uh_ep_pid",
                    description: Some(
                        "Host uh ep pid register.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhEpPid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_tx_ctrl",
                    description: Some(
                        "endpoint 2 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x7a,
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
                        "Device uep2 rx ctrl register.",
                    ),
                    array: None,
                    byte_offset: 0x7b,
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
                    name: "uh_rx_ctrl",
                    description: Some(
                        "Host uh rx ctrl register.",
                    ),
                    array: None,
                    byte_offset: 0x7b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_len",
                    description: Some(
                        "Device uep3 t len register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
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
                    name: "uh_tx_len",
                    description: Some(
                        "Host uh tx len register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhTxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_tx_ctrl",
                    description: Some(
                        "Device uep3 tx ctrl register.",
                    ),
                    array: None,
                    byte_offset: 0x7e,
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
                    name: "uh_tx_ctrl",
                    description: Some(
                        "Host uh tx ctrl register.",
                    ),
                    array: None,
                    byte_offset: 0x7e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_rx_ctrl",
                    description: Some(
                        "endpoint 3 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x7f,
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
                        "Device uep4 t len register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
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
                    name: "uh_split_data",
                    description: Some(
                        "Host uh split data register.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "UhSplitData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_tx_ctrl",
                    description: Some(
                        "endpoint 4 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x82,
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
                        "endpoint 4 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x83,
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
                        "endpoint 5 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x84,
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
                        "endpoint 5 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x86,
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
                        "endpoint 5 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x87,
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
                        "endpoint 6 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x88,
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
                        "endpoint 6 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x8a,
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
                        "endpoint 6 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x8b,
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
                        "endpoint 7 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
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
                        "endpoint 7 tx control.",
                    ),
                    array: None,
                    byte_offset: 0x8e,
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
                        "endpoint 7 rx control.",
                    ),
                    array: None,
                    byte_offset: 0x8f,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "USB base control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_en",
                    description: Some(
                        "DMA enable and DMA interrupt enable for USB.",
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
                    name: "clr_all",
                    description: Some(
                        "force clear FIFO and count of USB.",
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
                    name: "reset_sie",
                    description: Some(
                        "force reset USB SIE, need software clear.",
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
                    name: "int_busy",
                    description: Some(
                        "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid.",
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
                    name: "dev_pu_en",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
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
                    name: "sptp_mask",
                    description: Some(
                        "enable USB low speed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mode",
                    description: Some(
                        "enable USB host mode: 0=device mode, 1=host mode.",
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
                    name: "addr_mask",
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
            name: "FrameNo",
            extends: None,
            description: Some(
                "USB frame number register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "frame_no",
                    description: Some(
                        "USB frame number.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "USB interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ie_busrst",
                    description: Some(
                        "Enable the device bus reset interrupt.",
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
                    name: "ie_busrst_rb_ie_detect",
                    description: Some(
                        "enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode.",
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
                    name: "ie_detect",
                    description: Some(
                        "Enable the host device detection interrupt.",
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
                    name: "ie_trans",
                    description: Some(
                        "enable interrupt for USB transfer completion.",
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
                    name: "ie_suspend",
                    description: Some(
                        "enable interrupt for USB suspend or resume event.",
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
                    name: "ie_sof",
                    description: Some(
                        "enable interrupt for host SOF timer action for USB host mode.",
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
                    name: "ie_fifoov",
                    description: Some(
                        "enable interrupt for FIFO overflow.",
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
                    name: "ie_setupact",
                    description: Some(
                        "Setup packet end interrupt.",
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
                    name: "ie_isoact",
                    description: Some(
                        "Synchronous transmission received control token packet interrupt.",
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
                    name: "ie_dev_nak",
                    description: Some(
                        "enable interrupt for NAK responded for USB device mode.",
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
                "USB interrupt flag.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_busrst",
                    description: Some(
                        "Device bus reset interrupt flag; write 1 to clear.",
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
                    name: "if_busrst_rb_if_detect",
                    description: Some(
                        "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear.",
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
                    name: "if_detect",
                    description: Some(
                        "Host device detection interrupt flag; write 1 to clear.",
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
                    name: "if_transfer",
                    description: Some(
                        "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "if_suspend",
                    description: Some(
                        "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "if_hst_sof",
                    description: Some(
                        "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear.",
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
                    name: "if_fifoov",
                    description: Some(
                        "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear.",
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
                    name: "if_setuoact",
                    description: Some(
                        "RO, Setup transaction end interrupt flag.",
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
                    name: "if_isoact",
                    description: Some(
                        "RO, Synchronous transmission received control token packet interrupt flag.",
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
                    name: "dev_endp_mask",
                    description: Some(
                        "Device current transfer endpoint number.",
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
                    name: "host_res_mask",
                    description: Some(
                        "Host transfer handshake response PID; zero indicates no response.",
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
                    name: "host_res_mask_rb_dev_endp_mask",
                    description: Some(
                        "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode.",
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
                    name: "dev_token_mask",
                    description: Some(
                        "RO, bit mask of current token PID code received for USB device mode.",
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
                    name: "st_togok",
                    description: Some(
                        "RO, indicate current USB transfer toggle is OK.",
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
                    name: "st_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received for USB device mode.",
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
            name: "MisSt",
            extends: None,
            description: Some(
                "USB miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "split_en",
                    description: Some(
                        "RO,indicate host allow SPLIT packet.",
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
                    name: "attach",
                    description: Some(
                        "RO, indicate device attached status on USB host.",
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
                    name: "usbbus_suspend",
                    description: Some(
                        "RO, indicate USB suspend status.",
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
                    name: "usbbus_reset",
                    description: Some(
                        "RO, indicate USB bus reset status.",
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
                    name: "fifo_rdy",
                    description: Some(
                        "RO, indicate USB receiving FIFO ready status (not empty).",
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
                    name: "sie_free",
                    description: Some(
                        "RO, indicate USB SIE free status.",
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
                    name: "sof_act",
                    description: Some(
                        "RO, indicate host SOF timer action status for USB host.",
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
                    name: "sof_pres",
                    description: Some(
                        "RO, indicate host SOF timer presage status.",
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
            name: "R6UsbRxLen",
            extends: None,
            description: Some(
                "USB receiving length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "length of received bytes.",
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
            name: "SpdType",
            extends: None,
            description: Some(
                "USB actual speed register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "usbspeed_mask",
                    description: Some(
                        "USB actual speed.",
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
            name: "Suspend",
            extends: None,
            description: Some(
                "USB suspend register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dev_wakeup",
                    description: Some(
                        "Remote wake-up control bit.",
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
            ],
        },
        FieldSet {
            name: "Uep0MaxLen",
            extends: None,
            description: Some(
                "endpoint 0 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_max_len",
                    description: Some(
                        "endpoint 0 receive max length.",
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
            name: "Uep0RtDma",
            extends: None,
            description: Some(
                "endpoint 0 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep0_rt_dma",
                    description: Some(
                        "endpoint 0 DMA buffer address.",
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
            name: "Uep0RxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep0TLen",
            extends: None,
            description: Some(
                "endpoint 0 transmittal length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_t_len",
                    description: Some(
                        "endpoint 0 transmittal length.",
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
            name: "Uep0TxCtrl",
            extends: None,
            description: Some(
                "endpoint 0 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep1MaxLen",
            extends: None,
            description: Some(
                "endpoint 1 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_max_len",
                    description: Some(
                        "endpoint 1 receive max length.",
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
                "endpoint 1 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep1RxDma",
            extends: None,
            description: Some(
                "endpoint 1 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_rx_dma",
                    description: Some(
                        "endpoint 1 DMA buffer address.",
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
            name: "Uep1TLen",
            extends: None,
            description: Some(
                "endpoint 1 transmittal length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_t_len",
                    description: Some(
                        "endpoint 1 transmittal length.",
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
                "endpoint 1 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep1TxDma",
            extends: None,
            description: Some(
                "endpoint 1 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep1_tx_dma",
                    description: Some(
                        "endpoint 1 DMA TX buffer address.",
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
            name: "Uep23Mod",
            extends: None,
            description: Some(
                "Endpoint 2 and 3 device mode control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep2_buf_mod",
                    description: Some(
                        "Buffer mode of USB endpoint 2(10).",
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
                    name: "uep2_tx_en",
                    description: Some(
                        "enable USB endpoint 2(10) transmittal (IN).",
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
                    name: "uep2_rx_en",
                    description: Some(
                        "enable USB endpoint 2(10) receiving (OUT).",
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
                    name: "uep3_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 3(11).",
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
                    name: "uep3_tx_en",
                    description: Some(
                        "Enable USB endpoint 3(11) transmittal (IN).",
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
                    name: "uep3_rx_en",
                    description: Some(
                        "enable USB endpoint 3(11) receiving (OUT).",
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
            name: "Uep2MaxLen",
            extends: None,
            description: Some(
                "Device uep2 max len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_max_len",
                    description: Some(
                        "Device uep2 max len.",
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
                "Device uep2 rx ctrl register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "Handshake response type for device endpoint reception (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "Prepare no response for device endpoint reception.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "Expected data toggle for device endpoint reception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "Enable automatic toggle after successful endpoint reception.",
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
            name: "Uep2RxDma",
            extends: None,
            description: Some(
                "Device uep2 rx dma register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_rx_dma",
                    description: Some(
                        "Device uep2 rx dma.",
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
            name: "Uep2TLen",
            extends: None,
            description: Some(
                "Device uep2 t len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_t_len",
                    description: Some(
                        "endpoint 2 transmittal length.",
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
                "endpoint 2 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep2TxDma",
            extends: None,
            description: Some(
                "endpoint 2 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep2_tx_dma",
                    description: Some(
                        "endpoint 2 DMA TX buffer address.",
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
                "endpoint 3 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_max_len",
                    description: Some(
                        "endpoint 3 receive max length.",
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
                "endpoint 3 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep3RxDma",
            extends: None,
            description: Some(
                "endpoint 3 DMA buffer address;host tx endpoint buffer high address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_rx_dma",
                    description: Some(
                        "endpoint 3 DMA buffer address.",
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
            name: "Uep3TLen",
            extends: None,
            description: Some(
                "Device uep3 t len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_t_len",
                    description: Some(
                        "Device uep3 t len.",
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
                "Device uep3 tx ctrl register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "Handshake response type for device endpoint transmission (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "Expect no response for device endpoint transmission.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "Prepared data toggle for device endpoint transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "Enable automatic toggle after successful endpoint transmission.",
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
            name: "Uep3TxDma",
            extends: None,
            description: Some(
                "Device uep3 tx dma register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep3_tx_dma",
                    description: Some(
                        "Device uep3 tx dma.",
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
            name: "Uep41Mod",
            extends: None,
            description: Some(
                "endpoint 1(9) 4(8,12) mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep4_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 4(8,12).",
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
                    name: "uep4_tx_en",
                    description: Some(
                        "enable USB endpoint 4(8,12) transmittal (IN).",
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
                    name: "uep4_rx_en",
                    description: Some(
                        "enable USB endpoint 4(8,12) receiving (OUT).",
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
                    name: "uep1_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 1(9).",
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
                    name: "uep1_tx_en",
                    description: Some(
                        "enable USB endpoint 1(9) transmittal (IN).",
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
                    name: "uep1_rx_en",
                    description: Some(
                        "enable USB endpoint 1(9) receiving (OUT).",
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
            name: "Uep4MaxLen",
            extends: None,
            description: Some(
                "endpoint 4 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_max_len",
                    description: Some(
                        "endpoint 4 receive max length.",
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
                "endpoint 4 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep4RxDma",
            extends: None,
            description: Some(
                "endpoint 4 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_rx_dma",
                    description: Some(
                        "endpoint 4 DMA buffer address.",
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
            name: "Uep4TLen",
            extends: None,
            description: Some(
                "Device uep4 t len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep4_t_len",
                    description: Some(
                        "Device uep4 t len.",
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
                "endpoint 4 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep4TxDma",
            extends: None,
            description: Some(
                "endpoint 4 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep4_tx_dma",
                    description: Some(
                        "endpoint 4 DMA TX buffer address.",
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
            name: "Uep56Mod",
            extends: None,
            description: Some(
                "endpoint 5(13) 6(14) mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep5_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 5(13).",
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
                    name: "uep5_tx_en",
                    description: Some(
                        "enable USB endpoint 5(13) transmittal (IN).",
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
                    name: "uep5_rx_en",
                    description: Some(
                        "enable USB endpoint 5(13) receiving (OUT).",
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
                    name: "uep6_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 6(14).",
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
                    name: "uep6_tx_en",
                    description: Some(
                        "enable USB endpoint 6(14) transmittal (IN).",
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
                    name: "uep6_rx_en",
                    description: Some(
                        "enable USB endpoint 6(14) receiving (OUT).",
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
            name: "Uep5MaxLen",
            extends: None,
            description: Some(
                "endpoint 5 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_max_len",
                    description: Some(
                        "endpoint 5 receive max length.",
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
                "endpoint 5 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep5RxDma",
            extends: None,
            description: Some(
                "endpoint 5 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_rx_dma",
                    description: Some(
                        "endpoint 5 DMA buffer address.",
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
            name: "Uep5TLen",
            extends: None,
            description: Some(
                "endpoint 5 transmittal length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_t_len",
                    description: Some(
                        "endpoint 5 transmittal length.",
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
                "endpoint 5 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep5TxDma",
            extends: None,
            description: Some(
                "endpoint 5 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep5_tx_dma",
                    description: Some(
                        "endpoint 5 DMA TX buffer address.",
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
                "endpoint 6 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_max_len",
                    description: Some(
                        "endpoint 6 receive max length.",
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
                "endpoint 6 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep6RxDma",
            extends: None,
            description: Some(
                "endpoint 6 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_rx_dma",
                    description: Some(
                        "endpoint 6 DMA buffer address.",
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
            name: "Uep6TLen",
            extends: None,
            description: Some(
                "endpoint 6 transmittal length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_t_len",
                    description: Some(
                        "endpoint 6 transmittal length.",
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
                "endpoint 6 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep6TxDma",
            extends: None,
            description: Some(
                "endpoint 4 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep6_tx_dma",
                    description: Some(
                        "endpoint 6 DMA TX buffer address.",
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
                "endpoint 7 receive max length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_max_len",
                    description: Some(
                        "endpoint 7 receive max length.",
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
            name: "Uep7Mod",
            extends: None,
            description: Some(
                "endpoint 7(15) mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep7_buf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 7(15).",
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
                    name: "uep7_tx_en",
                    description: Some(
                        "enable USB endpoint 7(15) transmittal (IN).",
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
                    name: "uep7_rx_en",
                    description: Some(
                        "enable USB endpoint 7(15) receiving (OUT).",
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
            name: "Uep7RxCtrl",
            extends: None,
            description: Some(
                "endpoint 7 rx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_rres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_rres_no",
                    description: Some(
                        "prepared no response.",
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
                    name: "uep_r_tog_mask",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_r_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint.",
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
            name: "Uep7RxDma",
            extends: None,
            description: Some(
                "endpoint 7 DMA buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_rx_dma",
                    description: Some(
                        "endpoint 7 DMA buffer address.",
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
            name: "Uep7TLen",
            extends: None,
            description: Some(
                "endpoint 7 transmittal length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_t_len",
                    description: Some(
                        "endpoint 7 transmittal length.",
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
                "endpoint 7 tx control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_tres_mask",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X transmittal (IN).",
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
                    name: "uep_tres_no",
                    description: Some(
                        "expected no response.",
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
                    name: "uep_t_tog_mask",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uep_t_autotog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 0.",
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
            name: "Uep7TxDma",
            extends: None,
            description: Some(
                "endpoint 7 DMA TX buffer address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep7_tx_dma",
                    description: Some(
                        "endpoint 7 DMA TX buffer address.",
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
            name: "UepMaxLen",
            extends: None,
            description: Some(
                "Device endpoint maximum receive length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_max_len",
                    description: Some(
                        "Maximum receive length.",
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
            name: "UepRxDma",
            extends: None,
            description: Some(
                "Device endpoint receive DMA address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_rx_dma",
                    description: Some(
                        "Receive buffer address.",
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
            name: "UepTLen",
            extends: None,
            description: Some(
                "Device endpoint transmit length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_len",
                    description: Some(
                        "Transmit length.",
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
            name: "UepTxDma",
            extends: None,
            description: Some(
                "Device endpoint transmit DMA address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_tx_dma",
                    description: Some(
                        "Transmit buffer address.",
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
            name: "UhEpMod",
            extends: None,
            description: Some(
                "Host endpoint mode control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_rx_en",
                    description: Some(
                        "Enable USB host IN endpoint receiving.",
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
                    name: "uh_tx_en",
                    description: Some(
                        "Enable USB host OUT endpoint transmittal.",
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
            ],
        },
        FieldSet {
            name: "UhEpPid",
            extends: None,
            description: Some(
                "Host uh ep pid register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_epnum_mask",
                    description: Some(
                        "The endpoint number of the target of this operation.",
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
                    name: "uh_token_mask",
                    description: Some(
                        "The token PID packet identification of this USB transfer transaction.",
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
            ],
        },
        FieldSet {
            name: "UhRxCtrl",
            extends: None,
            description: Some(
                "Host uh rx ctrl register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_rres_mask",
                    description: Some(
                        "Handshake response type for host reception (IN).",
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
                    name: "uh_rres_no",
                    description: Some(
                        "Prepare no response for host reception.",
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
                    name: "uh_r_tog_mask",
                    description: Some(
                        "Expected data toggle for host reception (IN).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_r_autotog",
                    description: Some(
                        "Enable automatic toggle after successful host reception.",
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
                    name: "uh_rdata_no",
                    description: Some(
                        "expect no data packet, for high speed hub in host mode.",
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
            ],
        },
        FieldSet {
            name: "UhRxDma",
            extends: None,
            description: Some(
                "Host uh rx dma register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_dma",
                    description: Some(
                        "Host uh rx dma.",
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
            name: "UhRxMaxLen",
            extends: None,
            description: Some(
                "Host uh rx max len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_rx_max_len",
                    description: Some(
                        "Host uh rx max len.",
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
            name: "UhSplitData",
            extends: None,
            description: Some(
                "Host uh split data register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_split_data",
                    description: Some(
                        "Host uh split data.",
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
            name: "UhTxCtrl",
            extends: None,
            description: Some(
                "Host uh tx ctrl register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_tres_mask",
                    description: Some(
                        "Expected handshake response type for host transmission (SETUP/OUT).",
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
                    name: "uh_tres_no",
                    description: Some(
                        "Expect no response for host transmission.",
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
                    name: "uh_t_tog_mask",
                    description: Some(
                        "Prepared data toggle for host transmission (SETUP/OUT).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uh_t_autotog",
                    description: Some(
                        "Enable automatic toggle after successful host transmission.",
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
                    name: "uh_tdata_no",
                    description: Some(
                        "prepared no data packet, for high speed hub in host mode.",
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
            ],
        },
        FieldSet {
            name: "UhTxDma",
            extends: None,
            description: Some(
                "Host uh tx dma register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_dma",
                    description: Some(
                        "Host uh tx dma.",
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
            name: "UhTxLen",
            extends: None,
            description: Some(
                "Host uh tx len register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uh_tx_len",
                    description: Some(
                        "Host uh tx len.",
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
            name: "UhostCtrl",
            extends: None,
            description: Some(
                "USB host control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uh_bus_reset",
                    description: Some(
                        "USB host send bus reset signal.",
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
                    name: "uh_bus_suspend",
                    description: Some(
                        "USB host send bus suspend signal.",
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
                    name: "uh_bus_resume",
                    description: Some(
                        "USB host suspend state and wake up device.",
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
                    name: "uh_autosof_en",
                    description: Some(
                        "Automatically generate sof packet enable control.",
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
    ],
    enums: &[],
};
