use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usbhs",
            extends: None,
            description: Some(
                "USB register.",
            ),
            items: &[
                BlockItem {
                    name: "usb2_ctrl",
                    description: Some(
                        "RW, USBHS control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_base_mode",
                    description: Some(
                        "RW, USBHS control register.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2BaseMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_int_en",
                    description: Some(
                        "RW, USBHS interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_dev_ad",
                    description: Some(
                        "RW, USBHS device address register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2DevAd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_wake_ctrl",
                    description: Some(
                        "RW1Z, USBHS remote wake-up register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2WakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_test_mode",
                    description: Some(
                        "RW, USBHS test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2TestMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_lpm_data",
                    description: Some(
                        "RW, USBHS power management register.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Usb2LpmData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_int_fg",
                    description: Some(
                        "RW1Z, USBHS interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2IntFg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_int_st",
                    description: Some(
                        "RO, USBHS Interrupt Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2IntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_mis_st",
                    description: Some(
                        "RO, USBHS Miscellaneous Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "Usb2MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_fram_no",
                    description: Some(
                        "RO, USBHS frame register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Usb2FramNo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usb2_bus",
                    description: Some(
                        "RO, USBHS bus status memory.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Usb2Bus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep_tx_en",
                    description: Some(
                        "RW, USBHS endpoint sends enable register.",
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
                        "RW, USBHS endpoint receive enable register.",
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
                        "RW, USBHS endpoint receives automatic flip enable register.",
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
                        "RW, USBHS endpoint receives automatic flip enable register.",
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
                        "RW, 0~7 endpoints burst send enable.",
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
                        "RW, USBHS Endpoint Send Mode Register.",
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
                        "RW, 0~7 endpoints burst receive enable.",
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
                        "RW, USBHS Endpoint Receive Mode Register.",
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
                        "RW, USBHS Endpoint Reuse Register.",
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
                        "RW, Starting address register of endpoint 0 buffer.",
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
                        "RW, Endpoint n receive buffer start address (n=1~7).",
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
                        "RW, Endpoint n transmit buffer start address (n=1~7).",
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
                        "RW, Endpoint 0 maximum packet length.",
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
                        "RW, Endpoint n maximum packet length (n=1~7).",
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
                        "RW, Endpoint 0 received length.",
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
                        "RW, Endpoint n received length (n=1~7).",
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
                    name: "uep0_t_len",
                    description: Some(
                        "RW, Endpoint 0 transmit length.",
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
                        "RW, Endpoint 0 transmit control.",
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
                        "RW, Endpoint 0 receive control.",
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
                        "RW, Endpoint n transmit length (n=1~7).",
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
                        "RW, Endpoint n transmit control (n=1~7).",
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
                        "RW, Endpoint n receive control (n=1~7).",
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
                        "RW, USBHS endpoint sends synchronization mode enable register.",
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
                        "RW, USBHS endpoint receiving synchronization mode enable register.",
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
                BlockItem {
                    name: "uh_cfg",
                    description: Some(
                        "RW, USBHS Host Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_en",
                    description: Some(
                        "RW, USBHS host interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x102,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_dev_addr",
                    description: Some(
                        "RW, USBHS host device address register.",
                    ),
                    array: None,
                    byte_offset: 0x103,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhDevAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_control",
                    description: Some(
                        "RW, USBHS Host Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhControl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_flag",
                    description: Some(
                        "RW1Z, USBHS host interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_int_st",
                    description: Some(
                        "RW, USBHS host interrupt status register.",
                    ),
                    array: None,
                    byte_offset: 0x109,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhIntSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_mis_st",
                    description: Some(
                        "RW, USBHS host miscellaneous status register.",
                    ),
                    array: None,
                    byte_offset: 0x10a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UhMisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_lpm",
                    description: Some(
                        "RW, USBHS Host Power Management Data Register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhLpm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_split",
                    description: Some(
                        "RW, USBHS Host SPLIT Register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhSplit",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_frame",
                    description: Some(
                        "RW, USBHS Host Frame Register.",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhFrame",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_tx_len",
                    description: Some(
                        "RW, USBHS host sends length register.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhTxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_len",
                    description: Some(
                        "RW, USBHS host receives length register.",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_max_len",
                    description: Some(
                        "RW, USBHS host receives maximum length register.",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UhRxMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uh_rx_dma",
                    description: Some(
                        "RW, DMA receiving address register.",
                    ),
                    array: None,
                    byte_offset: 0x124,
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
                    name: "uh_tx_dma",
                    description: Some(
                        "RW, DMA sending address register.",
                    ),
                    array: None,
                    byte_offset: 0x128,
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
                    name: "port_ctrl",
                    description: Some(
                        "WO, USBHS host port control register.",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PortCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_cfg",
                    description: Some(
                        "RO, USBHS host port configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "PortCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_int_en",
                    description: Some(
                        "RO, USBHS host port configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "PortIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_test_ct",
                    description: Some(
                        "RW, USBHS host port test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x133,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PortTestCt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_status",
                    description: Some(
                        "RO, USBHS Host Port Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "PortStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_status_chg",
                    description: Some(
                        "RW1Z, USBHS Host Port Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PortStatusChg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "root_bc_ctrl",
                    description: Some(
                        "RW, USBHS host BC charging control register.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RootBcCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll_ctrl",
                    description: Some(
                        "RW, USBHS PLL Clock Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PllCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "PllCtrl",
            extends: None,
            description: Some(
                "RW, USBHS PLL Clock Control Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pll_cksel",
                    description: Some(
                        "Selection of internal reference clock source for USBHS.",
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
                    name: "pll_lowpow",
                    description: Some(
                        "USBHS PLL low-power mode (can only be enabled when USB20 application is at fixed low full speed, cannot be enabled when not at fixed low full speed).",
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
                    name: "pll_en",
                    description: Some(
                        "USBHS PLL Enable.",
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
            ],
        },
        FieldSet {
            name: "PortCfg",
            extends: None,
            description: Some(
                "RO, USBHS host port configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "host_mode",
                    description: Some(
                        "USB port mode selection.",
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
                    name: "port_rpd",
                    description: Some(
                        "Enable 15K resistor pull-down in host mode.",
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
            name: "PortCtrl",
            extends: None,
            description: Some(
                "WO, USBHS host port control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set_port_reset",
                    description: Some(
                        "Port sending reset.",
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
                    name: "set_port_susp",
                    description: Some(
                        "PORT enters suspended state.",
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
                    name: "clr_port_susp",
                    description: Some(
                        "PORT exits suspended state.",
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
                    name: "set_port_sleep",
                    description: Some(
                        "PORT enters SLEEP state (LPM).",
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
                    name: "clr_port_en",
                    description: Some(
                        "PORT exits the enabled state and enters the disabled state.",
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
                    name: "clr_port_connect",
                    description: Some(
                        "Put the port into port state.",
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
                    name: "clr_port_sleep",
                    description: Some(
                        "PORT exits SLEEP state (LPM).",
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
                    name: "port_sleep_besl",
                    description: Some(
                        "Wake up time control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bus_rst_long",
                    description: Some(
                        "Bus reset time selection.",
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
            name: "PortIntEn",
            extends: None,
            description: Some(
                "RO, USBHS host port configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port_connect_ie",
                    description: Some(
                        "Port connection status change interrupt enable.",
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
                    name: "port_en_ie",
                    description: Some(
                        "Port enable status change interrupt enable.",
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
                    name: "port_susp_ie",
                    description: Some(
                        "Port pause status change interrupt enable.",
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
                    name: "port_reset_ie",
                    description: Some(
                        "Port reset status change interrupt enable.",
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
                    name: "port_slp_ie",
                    description: Some(
                        "Port sleep state change interrupt enable.",
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
            name: "PortStatus",
            extends: None,
            description: Some(
                "RO, USBHS Host Port Status Register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "port_c0nnect",
                    description: Some(
                        "Port connection status.",
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
                    name: "port_en",
                    description: Some(
                        "Port Enable.",
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
                    name: "port_susp",
                    description: Some(
                        "Port paused state.",
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
                    name: "port_restting",
                    description: Some(
                        "Port reset status.",
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
                    name: "port_slp",
                    description: Some(
                        "Port sleep.",
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
                    name: "port_low_spd",
                    description: Some(
                        "Is the port connection speed low.",
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
                    name: "port_high_spd",
                    description: Some(
                        "Is the port connection speed high.",
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
                    name: "port_test",
                    description: Some(
                        "Is the port in test mode.",
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
            ],
        },
        FieldSet {
            name: "PortStatusChg",
            extends: None,
            description: Some(
                "RW1Z, USBHS Host Port Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port_connect_if",
                    description: Some(
                        "Port connection status changes, write 1 to reset to zero.",
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
                    name: "port_en_if",
                    description: Some(
                        "Port enable status changes, write 1 to reset to zero.",
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
                    name: "port_susp_if",
                    description: Some(
                        "Port pause status changes, write 1 to reset to zero.",
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
                    name: "port_reset_if",
                    description: Some(
                        "Port reset status changes, write 1 to reset to zero.",
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
                    name: "port_slp_if",
                    description: Some(
                        "Port sleep status changes, write 1 to reset to zero.",
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
            name: "PortTestCt",
            extends: None,
            description: Some(
                "RW, USBHS host port test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "test_j",
                    description: Some(
                        "Test output J.",
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
                        "Test output K.",
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
                    name: "test_force_en",
                    description: Some(
                        "Enable Test Mode.",
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
            ],
        },
        FieldSet {
            name: "RootBcCtrl",
            extends: None,
            description: Some(
                "RW, USBHS host BC charging control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "udp_bc_cmpo",
                    description: Some(
                        "UDP pin BC protocol comparator status.",
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
                    name: "udm_bc_cmpo",
                    description: Some(
                        "UDM pin BC protocol comparator status.",
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
                    name: "udp_bc_cmpe",
                    description: Some(
                        "UDP pin BC protocol comparator enabled.",
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
                    name: "udm_bc_cmpe",
                    description: Some(
                        "UDM pin BC protocol comparator enabled.",
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
                    name: "bc_auto_mode",
                    description: Some(
                        "Automatic mode enabled.",
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
                    name: "udp_bc_vsrc",
                    description: Some(
                        "UDP pin BC protocol source voltage enable.",
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
                    name: "udm_bc_vsrc",
                    description: Some(
                        "UDM pin BC protocol source voltage enabled.",
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
                    name: "udm_vsrc_act",
                    description: Some(
                        "In automatic mode, this bit represents the UDM output VBC_SRC, otherwise it is controlled by UDM-BC_CMPE.",
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
            name: "Uep0Dma",
            extends: None,
            description: Some(
                "RW, Starting address register of endpoint 0 buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "Endpoint 0 buffer address, aligned to 4 bytes.",
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
                "RW, Endpoint 0 maximum packet length.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_len",
                    description: Some(
                        "Maximum packet length in bytes.",
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
                "RW, Endpoint 0 receives control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask10",
                    description: Some(
                        "Endpoint 0 controls the received response.",
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
                    name: "uep_tog_mask1",
                    description: Some(
                        "Expected synchronization trigger bit for receiving endpoint 0.",
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
                        "Is the transaction received by endpoint 0 a SETUP transaction.",
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
                        "The received synchronization trigger bit matches the expected synchronization trigger bit.",
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
                        "Endpoint 0 returns NAK for received data packet type.",
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
                        "Endpoint 0 receives NAK end flag, write 0 to clear.",
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
                        "Endpoint 0 receives end flag, write 0 to clear.",
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
                "RW, Endpoint 0 received length.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "Number of bytes received on endpoint 0.",
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
            name: "Uep0TLen",
            extends: None,
            description: Some(
                "RW, Endpoint 0 sends length register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_t_len60",
                    description: Some(
                        "Endpoint 0 sends data length.",
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
                "RW, Endpoint 0 sends control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask10",
                    description: Some(
                        "Endpoint 0 controls the response sent.",
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
                    name: "uep_tog_mask10",
                    description: Some(
                        "Expected synchronization trigger bit for endpoint 0.",
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
                        "Endpoint 0 sends NAK end flag, write 0 to clear.",
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
                        "Endpoint 0 sends end flag, write 0 to clear.",
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
                "RW, USBHS Endpoint Reuse Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uep_t_af60",
                    description: Some(
                        "1-7 endpoint reuse enabled.",
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
                "RW, Endpoint n maximum packet length (n=1~7).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "max_len",
                    description: Some(
                        "Maximum packet length in bytes.",
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
            name: "UepRBurst",
            extends: None,
            description: Some(
                "RW, 0~7 endpoints burst receive enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_burst_en70",
                    description: Some(
                        "0~7 endpoints burst receive enable.",
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
                "RW, USBHS endpoint receiving synchronization mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_iso60",
                    description: Some(
                        "Enable the synchronization mode of the downstream endpoint (OUT).",
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
                "RW, USBHS Endpoint Receive Mode Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_r_res_mode70",
                    description: Some(
                        "0~7 endpoints burst receive enable.",
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
                "RW, USBHS endpoint receives automatic flip enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_r_tog_auto70",
                    description: Some(
                        "0~7 endpoint synchronization trigger bit automatic flip enable.",
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
                "RW, Endpoint n receives control registers(n=1~7).",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask10",
                    description: Some(
                        "Endpoint n controls the received response(n=1~7).",
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
                    name: "uep_tog_mask10",
                    description: Some(
                        "Expected synchronization trigger bit for receiving endpoint n(n=1~7).",
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
                        "The synchronization trigger bit received by endpoint n matches the expected synchronization trigger bit(n=1~7).",
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
                        "Endpoint n receives the return NAK and packet type(n=1~7).",
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
                        "Endpoint n receives NAK end flag, writes 0 to clear(n=1~7).",
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
                        "Endpoint n receives the end flag, write 0 to clear(n=1~7).",
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
                "RW, Endpoint n receive buffer start address (n=1~7).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "Receive buffer address, aligned to 4 bytes.",
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
                "RW, USBHS endpoint receive enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_rx_en150",
                    description: Some(
                        "0~15 endpoints receive enable.",
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
                "RW, Endpoint n received length (n=1~7).",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "Number of bytes received on endpoint n.",
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
                "RW, 0~7 endpoints burst send enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_en70",
                    description: Some(
                        "0~7 endpoints burst send enable.",
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
                "RW, USBHS Endpoint Send Mode Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep_t_burst_en70",
                    description: Some(
                        "0~7 endpoints burst send enable.",
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
                "RW, USBHS endpoint sends synchronization mode enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_iso60",
                    description: Some(
                        "Upload endpoint (IN) synchronization mode enabled.",
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
                "RW, Endpoint n transmit length (n=1~7).",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "t_len",
                    description: Some(
                        "Number of bytes to transmit on endpoint n.",
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
                "RW, USBHS endpoint receives automatic flip enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_t_tog_auto70",
                    description: Some(
                        "0~7 endpoint synchronization trigger bit automatic flip enable.",
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
                "RW, Endpoint n sends data length(n=1~7).",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "res_mask10",
                    description: Some(
                        "Endpoint n controls the response sent(n=1~7).",
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
                    name: "uep_tog_mask10",
                    description: Some(
                        "Expected synchronization trigger bit for endpoint n's sending(n=1~7).",
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
                        "Endpoint n sends NAK end flag, write 0 to clear(n=1~7).",
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
                        "Endpoint n sends an end flag, write 0 to clear(n=1~7).",
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
                "RW, Endpoint n transmit buffer start address (n=1~7).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "address",
                    description: Some(
                        "Transmit buffer address, aligned to 4 bytes.",
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
                "RW, USBHS endpoint sends enable register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep_tx_en150",
                    description: Some(
                        "0~15 endpoints send enable.",
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
            name: "UhCfg",
            extends: None,
            description: Some(
                "RW, USBHS Host Configuration Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "root_link_reset",
                    description: Some(
                        "USB connection control module reset.",
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
                    name: "root_sie_reset",
                    description: Some(
                        "USB protocol processor reset, software reset required.",
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
                    name: "root_all_clr",
                    description: Some(
                        "Clear interrupt flag and FIFO, software zeroing is required.",
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
                    name: "root_phy_suspendm",
                    description: Some(
                        "PHY hangs, turn off the UTMI clock of the controller.",
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
                    name: "root_dma_en",
                    description: Some(
                        "Enable DMA transmission.",
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
                    name: "root_sof_en",
                    description: Some(
                        "Enable SOF packet transmission.",
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
                    name: "root_force_fs",
                    description: Some(
                        "Force full speed FS.",
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
                    name: "root_lpm_en",
                    description: Some(
                        "Enable LPM.",
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
            name: "UhControl",
            extends: None,
            description: Some(
                "RW, USBHS Host Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid_mask30",
                    description: Some(
                        "Transaction token package PID.",
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
                    name: "endp_mask30",
                    description: Some(
                        "Transaction token package endpoint number.",
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
                    name: "tog_mask10",
                    description: Some(
                        "Sent data PID.",
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
                    name: "buf_mode",
                    description: Some(
                        "Data cache control bit.",
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
                    name: "host_action",
                    description: Some(
                        "HOST enables transaction execution, and this bit is automatically reset to zero after the transaction is completed.",
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
                    name: "lpm_valid",
                    description: Some(
                        "Sending LPM package is valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "split_valid",
                    description: Some(
                        "Sending SPLIT package is valid.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "down_lowspd",
                    description: Some(
                        "When the port is operating at full speed and needs to send low-speed packets (PRE), this bit must be enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tx_no_data",
                    description: Some(
                        "There is no data packet after the OUT/SETUP token packet, used for high-speed SPLIT packets.",
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
                    name: "rx_no_data",
                    description: Some(
                        "Expected data packet after IN token packet, used for high-speed SPLIT packet.",
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
                    name: "tx_no_res",
                    description: Some(
                        "OUT/SETUP - DATA does not expect a response, used for synchronous transmission or high-speed SPLIT packets.",
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
                    name: "rx_no_res",
                    description: Some(
                        "IN-DATA no response, used for synchronous transmission or high-speed SPLIT packets.",
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
            name: "UhDevAddr",
            extends: None,
            description: Some(
                "RW, USBHS host device address register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dev_addr70",
                    description: Some(
                        "Device Address.",
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
            name: "UhFrame",
            extends: None,
            description: Some(
                "RW, USBHS Host Frame Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frame_no",
                    description: Some(
                        "frame number.",
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
                    name: "micro_frame_hs",
                    description: Some(
                        "Micro frame number.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sof_cnt_en",
                    description: Some(
                        "SOF Count Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sof_cnt_clr",
                    description: Some(
                        "SOF count reset to zero.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhIntEn",
            extends: None,
            description: Some(
                "RW, USBHS host interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wkup_act_ie",
                    description: Some(
                        "Wake up interrupt enable.",
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
                    name: "resume_act_ie",
                    description: Some(
                        "Bus recovery interrupt enable.",
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
                    name: "act_ie",
                    description: Some(
                        "USB end or transfer completion interrupt enable.",
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
                    name: "sof_act_ie",
                    description: Some(
                        "Enable SOF packet sending interrupt.",
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
                    name: "tx_halt_ie",
                    description: Some(
                        "Enable sending pause interrupt.",
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
                    name: "fifo_ov_ie",
                    description: Some(
                        "FIFO overflow interrupt enable.",
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
            name: "UhIntFlag",
            extends: None,
            description: Some(
                "RW1Z, USBHS host interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wkup_if",
                    description: Some(
                        "Wake up interrupt flag, write 1 to reset to zero.",
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
                    name: "resume_act_if",
                    description: Some(
                        "Bus recovery interrupt flag, write 1 to reset to zero.",
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
                    name: "act_if",
                    description: Some(
                        "USB transaction transfer completion interrupt flag, write 1 to reset to zero.",
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
                    name: "sof_act_if",
                    description: Some(
                        "SOF packet sending completion interrupt flag, write 1 to reset to zero.",
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
                    name: "tx_halt_if",
                    description: Some(
                        "Send abort interrupt flag.",
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
                    name: "fifo_ov_if",
                    description: Some(
                        "FIFO overflow interrupt flag.",
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
            name: "UhIntSt",
            extends: None,
            description: Some(
                "RW, USBHS host interrupt status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "this_pid_mask30",
                    description: Some(
                        "Received PID.",
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
                    name: "port_rx_resume",
                    description: Some(
                        "This bit is 1, indicating that the port has received a wake-up signal.",
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
            name: "UhLpm",
            extends: None,
            description: Some(
                "RW, USBHS Host Power Management Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpm_data",
                    description: Some(
                        "Power management data.",
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
            ],
        },
        FieldSet {
            name: "UhMisSt",
            extends: None,
            description: Some(
                "RW, USBHS host miscellaneous status register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sof_free",
                    description: Some(
                        "Port enabled status.",
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
                    name: "sof_pre",
                    description: Some(
                        "The USB bus SOF packet indicates a status of.",
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
                    name: "sof_st",
                    description: Some(
                        "USB bus SOF status.",
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
                    name: "wakeup",
                    description: Some(
                        "USB bus wake-up.",
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
                    name: "linestate_mask",
                    description: Some(
                        "USB bus status.",
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
                    name: "bus_j",
                    description: Some(
                        "J on the USB bus.",
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
                    name: "bus_se0",
                    description: Some(
                        "SEO on USB bus.",
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
            name: "UhRxDma",
            extends: None,
            description: Some(
                "RW, DMA receiving address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_dma",
                    description: Some(
                        "Receiving Address.",
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
            name: "UhRxLen",
            extends: None,
            description: Some(
                "RW, USBHS host receives length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "Received data length.",
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
            ],
        },
        FieldSet {
            name: "UhRxMaxLen",
            extends: None,
            description: Some(
                "RW, USBHS host receives maximum length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_rx_max_len",
                    description: Some(
                        "Received data max length.",
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
            ],
        },
        FieldSet {
            name: "UhSplit",
            extends: None,
            description: Some(
                "RW, USBHS Host SPLIT Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "split_data",
                    description: Some(
                        "SPLIT_DATA.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "UhTxDma",
            extends: None,
            description: Some(
                "RW, DMA sending address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uh_tx_dma_hs",
                    description: Some(
                        "Sending Address.",
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
                "RW, USBHS host sends length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_len",
                    description: Some(
                        "Sending data length.",
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
            ],
        },
        FieldSet {
            name: "Usb2BaseMode",
            extends: None,
            description: Some(
                "RW, USBHS control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "exp_spd10",
                    description: Some(
                        "Expected speed mode of the device.",
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
            name: "Usb2Bus",
            extends: None,
            description: Some(
                "RO, USBHS bus status memory.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "wakeup",
                    description: Some(
                        "USB wake-up (highly effective).",
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
            name: "Usb2Ctrl",
            extends: None,
            description: Some(
                "RW, USBHS control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "link_reset",
                    description: Some(
                        "LINK layer reset, highly effective.",
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
                        "USB protocol processor reset, including endpoint related registers, highly effective.",
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
                        "Clear all interrupt flags.",
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
                        "USB PHY suspended.",
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
                        "DMA transfer enable.",
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
                        "USB device enablement.",
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
                        "LPM Enable.",
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
            name: "Usb2DevAd",
            extends: None,
            description: Some(
                "RW, USBHS device address register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_usb_addr60",
                    description: Some(
                        "USB device address.",
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
            name: "Usb2FramNo",
            extends: None,
            description: Some(
                "RO, USBHS frame register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "frame_no100",
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
                    name: "micro_frame20",
                    description: Some(
                        "Received microframe number.",
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
            name: "Usb2IntEn",
            extends: None,
            description: Some(
                "RW, USBHS interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bus_rest_ie",
                    description: Some(
                        "USB bus reset interrupt enable.",
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
                        "USB bus pause interrupt enable.",
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
                        "USB bus sleep interrupt enable.",
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
                        "LPM transmission end interrupt enable.",
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
                        "USB Transfer End Interrupt Enable.",
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
                        "USB connection interruption enabled.",
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
                        "FIFO overflow interrupt.",
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
            name: "Usb2IntFg",
            extends: None,
            description: Some(
                "RW1Z, USBHS interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uif_bus_rst",
                    description: Some(
                        "USB bus reset interrupt flag bit, write 1 to reset to zero.",
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
                    name: "uif_suspend_hs",
                    description: Some(
                        "USB bus hangs interrupt flag, write 1 to reset to zero.",
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
                        "USB bus sleep interrupt flag, write 1 to reset to zero.",
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
                        "LPM transmission end interrupt flag, write 1 to reset to zero.",
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
                        "USB transfer end interrupt flag bit, cleared by RB-UMS_SHMOD bit.",
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
                        "Receive SOF packet interrupt flag, write 1 to reset to zero.",
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
                        "USB connection interruption flag bit, write 1 to reset to zero.",
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
                    name: "uif_fifo_ov_hs",
                    description: Some(
                        "FIFO overflow interrupt flag bit, write 1 to reset to zero.",
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
            name: "Usb2IntSt",
            extends: None,
            description: Some(
                "RO, USBHS Interrupt Status Register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uis_ep_id_mask20",
                    description: Some(
                        "Endpoint number where data transmission occurs.",
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
            name: "Usb2LpmData",
            extends: None,
            description: Some(
                "RW, USBHS power management register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "lpm_data100",
                    description: Some(
                        "Power management data.",
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
                        "Power management busy.",
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
            name: "Usb2MisSt",
            extends: None,
            description: Some(
                "RO, USBHS Miscellaneous Status Register.",
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
                    name: "ums_suspend_hs",
                    description: Some(
                        "USB suspended status.",
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
                        "USB sleep state.",
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
                    name: "ums_sie_free_hs",
                    description: Some(
                        "USB idle state.",
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
                        "USB suspend request.",
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
                        "Is the host high-speed.",
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
            name: "Usb2TestMode",
            extends: None,
            description: Some(
                "RW, USBHS test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "test_j",
                    description: Some(
                        "Test mode, output J.",
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
                        "Test mode, output K.",
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
                        "Test mode, output a package.",
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
                        "Test mode, output SE0.",
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
                        "Enable Test Mode.",
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
            name: "Usb2WakeCtrl",
            extends: None,
            description: Some(
                "RW1Z, USBHS remote wake-up register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rmt_wake",
                    description: Some(
                        "Remote wake-up, hardware auto reset.",
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
