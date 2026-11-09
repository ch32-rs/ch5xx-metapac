use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Usb",
            extends: None,
            description: Some(
                "USB register.",
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
                    name: "udev_ctrl__uhost_ctrl",
                    description: Some(
                        "USB device physical prot control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "UdevCtrl_uhostCtrl",
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
                    name: "mis_st",
                    description: Some(
                        "USB miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0x5,
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
                    byte_offset: 0x6,
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
                    byte_offset: 0x7,
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
                    name: "rx_len",
                    description: Some(
                        "USB receiving length.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_1_mod",
                    description: Some(
                        "endpoint 4/1 mode.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "uep2_3_mod__uh_ep_mod",
                    description: Some(
                        "endpoint 2_3 mode;host endpoint mode.",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep23Mod_uhEpMod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep567_mod",
                    description: Some(
                        "endpoint 5/6/7 mode.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep567Mod",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_dma",
                    description: Some(
                        "endpoint 0 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep0Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_dma",
                    description: Some(
                        "endpoint 1 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep1Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_dma__uh_rx_dma",
                    description: Some(
                        "endpoint 2 DMA buffer address; host RX endpoint buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep2Dma_uhRxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_dma__uh_tx_dma",
                    description: Some(
                        "endpoint 3 DMA buffer address; host TX endpoint buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep3Dma_uhTxDma",
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
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep0_ctrl",
                    description: Some(
                        "endpoint 0 control.",
                    ),
                    array: None,
                    byte_offset: 0x22,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep0Ctrl",
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
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep1_ctrl__uh_setup",
                    description: Some(
                        "endpoint 1 control;host aux setup.",
                    ),
                    array: None,
                    byte_offset: 0x26,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep1Ctrl_uhSetup",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_t_len_uh_ep_pid",
                    description: Some(
                        "endpoint 2 transmittal length;host endpoint and PID.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2TLenUhEpPid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep2_ctrl_uh_rx_ctrl",
                    description: Some(
                        "endpoint 2 control;host receiver endpoint control.",
                    ),
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep2CtrlUhRxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_t_len__uh_tx_len",
                    description: Some(
                        "endpoint 3 transmittal length;host transmittal endpoint transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3TLen_uhTxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep3_ctrl__uh_tx_ctrl",
                    description: Some(
                        "endpoint 3 control;host transmittal endpoint control.",
                    ),
                    array: None,
                    byte_offset: 0x2e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep3Ctrl_uhTxCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 transmittal length.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep4_ctrl",
                    description: Some(
                        "endpoint 4 control.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep4Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_dma",
                    description: Some(
                        "endpoint 5 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep5Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_dma",
                    description: Some(
                        "endpoint 6 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep6Dma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_dma",
                    description: Some(
                        "endpoint 7 DMA buffer address.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Uep7Dma",
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
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep5_ctrl",
                    description: Some(
                        "endpoint 5 control.",
                    ),
                    array: None,
                    byte_offset: 0x66,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep5Ctrl",
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
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep6_ctrl",
                    description: Some(
                        "endpoint 6 control.",
                    ),
                    array: None,
                    byte_offset: 0x6a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep6Ctrl",
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
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7TLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "uep7_ctrl",
                    description: Some(
                        "endpoint 7 control.",
                    ),
                    array: None,
                    byte_offset: 0x6e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Uep7Ctrl",
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
                    name: "uc_dma_en",
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
                    name: "uc_clr_all",
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
                    name: "uc_reset_sie",
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
                    name: "uc_int_busy",
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
                    name: "mask_uc_sys_ctrl",
                    description: Some(
                        "bit mask of USB system control.",
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
                    name: "uc_dev_pu_en",
                    description: Some(
                        "USB device enable and internal pullup resistance enable.",
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
                    name: "uc_low_speed",
                    description: Some(
                        "enable USB low speed: 0=12Mbps, 1=1.5Mbps.",
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
                    name: "uc_host_mode",
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
                Field {
                    name: "uda_gp_bit",
                    description: Some(
                        "general purpose bit.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "USB interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uie_bus_rst__uie_detect",
                    description: Some(
                        "enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode.",
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
                    name: "uie_transfer",
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
                    name: "uie_suspend",
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
                    name: "uie_hst_sof",
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
                    name: "uie_fifo_ov",
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
                    name: "uie_dev_nak",
                    description: Some(
                        "enable interrupt for NAK responded for USB device mode.",
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
                    name: "uie_dev_sof",
                    description: Some(
                        "enable interrupt for SOF received for USB device mode.",
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
                    name: "uif_bus_rst__uif_detect",
                    description: Some(
                        "RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear.",
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
                    name: "uif_transfer",
                    description: Some(
                        "RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "uif_suspend",
                    description: Some(
                        "RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear.",
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
                    name: "uif_hst_sof",
                    description: Some(
                        "RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear.",
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
                    name: "uif_fifo_ov",
                    description: Some(
                        "RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear.",
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
                    name: "u_sie_free",
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
                    name: "u_tog_ok",
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
                    name: "u_is_nak",
                    description: Some(
                        "RO, indicate current USB transfer is NAK received.",
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
                    name: "mask_uis_h_res__mask_uis_endp",
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
                    name: "mask_uis_token",
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
                    name: "uis_tog_ok",
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
                    name: "uis_setup_act",
                    description: Some(
                        "RO, indicates a SETUP token and 8-byte setup request were received in USB device mode.",
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
                    name: "ums_dev_attach",
                    description: Some(
                        "RO, indicate device attached status on USB host.",
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
                    name: "ums_dm_level",
                    description: Some(
                        "RO, indicate UDM level saved at device attached to USB host.",
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
                    name: "ums_suspend",
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
                    name: "ums_bus_reset",
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
                    name: "ums_r_fifo_rdy",
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
                    name: "ums_sie_free",
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
                    name: "ums_sof_act",
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
                    name: "ums_sof_pres",
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
            name: "RxLen",
            extends: None,
            description: Some(
                "USB receiving length.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rx_len",
                    description: Some(
                        "RO,USB receiving length.",
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
            name: "UdevCtrl_uhostCtrl",
            extends: None,
            description: Some(
                "USB device physical prot control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ud_port_en__uh_port_en",
                    description: Some(
                        "enable USB physical port I-O: 0=disable, 1=enable;enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached.",
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
                    name: "ud_gp_bit__uh_bus_reset",
                    description: Some(
                        "general purpose bit;control USB bus reset: 0=normal, 1=force bus reset.",
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
                    name: "ud_low_speed__uh_low_speed",
                    description: Some(
                        "enable USB physical port low speed: 0=full speed, 1=low speed;enable USB port low speed: 0=full speed, 1=low speed.",
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
                    name: "ud_dm_pin__uh_dm_pin",
                    description: Some(
                        "ReadOnly: indicate current UDM pin level.",
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
                    name: "ud_dp_pin__uh_dp_pin",
                    description: Some(
                        "ReadOnly: indicate current UDP pin level.",
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
                    name: "ud_pd_dis__uh_pd_dis",
                    description: Some(
                        "disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable.",
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
            name: "Uep0Ctrl",
            extends: None,
            description: Some(
                "endpoint 0 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "Automatic toggle is documented for endpoints 1, 2 and 3 only.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
                "endpoint 0 DMA buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep0_dma",
                    description: Some(
                        "RW,endpoint 0 DMA buffer address.",
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
                "endpoint 0 transmittal length.",
            ),
            bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep1Ctrl_uhSetup",
            extends: None,
            description: Some(
                "endpoint 1 control;host aux setup.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle.",
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
                    name: "uep_t_tog__uh_sof_en",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1;USB host automatic SOF enable.",
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
                    name: "uep_r_tog__uh_pre_pid_en",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;RB_UH_PRE_PID_EN;USB host PRE PID enable for low speed device via hub.",
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
            name: "Uep1Dma",
            extends: None,
            description: Some(
                "endpoint 1 DMA buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep1_dma",
                    description: Some(
                        "RW,endpoint 1 DMA buffer address.",
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
                "endpoint 1 transmittal length.",
            ),
            bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep23Mod_uhEpMod",
            extends: None,
            description: Some(
                "endpoint 2_3 mode;host endpoint mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep2_buf_mod__uh_ep_rbuf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 2;buffer mode of USB host IN endpoint.",
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
                        "enable USB endpoint 2 transmittal (IN).",
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
                    name: "uep2_rx_en__uh_ep_rx_en",
                    description: Some(
                        "enable USB endpoint 2 receiving (OUT);enable USB host IN endpoint receiving.",
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
                    name: "uep3_buf_mod__uh_ep_tbuf_mod",
                    description: Some(
                        "buffer mode of USB endpoint 3;buffer mode of USB host OUT endpoint.",
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
                    name: "uep3_tx_en__uh_ep_tx_en",
                    description: Some(
                        "enable USB endpoint 3 transmittal (IN);enable USB host OUT endpoint transmittal.",
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
                        "enable USB endpoint 3 receiving (OUT).",
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
            name: "Uep2CtrlUhRxCtrl",
            extends: None,
            description: Some(
                "endpoint 2 control;host receiver endpoint control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "mask_uh_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog__uh_r_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle;enable automatic toggle after successful transfer completion: 0=manual toggle, 1=automatic toggle.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog__uh_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1;expected data toggle flag of host receiving (IN): 0=DATA0, 1=DATA1.",
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
            name: "Uep2Dma_uhRxDma",
            extends: None,
            description: Some(
                "endpoint 2 DMA buffer address; host RX endpoint buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep2_dma",
                    description: Some(
                        "RW,endpoint 2 DMA buffer address; host RX endpoint buffer address.",
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
            name: "Uep2TLenUhEpPid",
            extends: None,
            description: Some(
                "endpoint 2 transmittal length;host endpoint and PID.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uh_endp",
                    description: Some(
                        "bit mask of endpoint number for USB host transfer.",
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
                    name: "uep2_t_len",
                    description: Some(
                        "endpoint 2 transmittal length;.",
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
                    name: "mask_uh_token",
                    description: Some(
                        "bit mask of token PID for USB host transfer.",
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
            name: "Uep3Ctrl_uhTxCtrl",
            extends: None,
            description: Some(
                "endpoint 3 control;host transmittal endpoint control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "uh_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog_rb_uh_t_auto_tog",
                    description: Some(
                        "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle.",
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
                    name: "uep_t_tog_rb_uh_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
            name: "Uep3Dma_uhTxDma",
            extends: None,
            description: Some(
                "endpoint 3 DMA buffer address; host TX endpoint buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep3_dma",
                    description: Some(
                        "RW, endpoint 3 DMA buffer address; host TX endpoint buffer address.",
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
            name: "Uep3TLen_uhTxLen",
            extends: None,
            description: Some(
                "endpoint 3 transmittal length;host transmittal endpoint transmittal length.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep3_t_len__uh_tx_len",
                    description: Some(
                        "endpoint 1 transmittal length.",
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
            name: "Uep41Mod",
            extends: None,
            description: Some(
                "endpoint 4/1 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep4_tx_en",
                    description: Some(
                        "enable USB endpoint 4 transmittal (IN).",
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
                        "enable USB endpoint 4 receiving (OUT).",
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
                        "buffer mode of USB endpoint 1.",
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
                        "enable USB endpoint 1 transmittal (IN).",
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
                        "enable USB endpoint 1 receiving (OUT).",
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
            name: "Uep4Ctrl",
            extends: None,
            description: Some(
                "endpoint 4 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "Automatic toggle is documented for endpoints 1, 2 and 3 only.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
            name: "Uep4TLen",
            extends: None,
            description: Some(
                "endpoint 4 transmittal length.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep4_t_len",
                    description: Some(
                        "endpoint 4 transmittal length.",
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
            name: "Uep567Mod",
            extends: None,
            description: Some(
                "endpoint 5/6/7 mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "uep5_tx_en",
                    description: Some(
                        "enable USB endpoint 5 transmittal (IN).",
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
                    name: "uep5_rx_en",
                    description: Some(
                        "enable USB endpoint 5 receiving (OUT).",
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
                    name: "uep6_tx_en",
                    description: Some(
                        "enable USB endpoint 6 transmittal (IN).",
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
                    name: "uep6_rx_en",
                    description: Some(
                        "enable USB endpoint 6 receiving (OUT).",
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
                    name: "uep7_tx_en",
                    description: Some(
                        "enable USB endpoint 7 transmittal (IN).",
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
                    name: "uep7_rx_en",
                    description: Some(
                        "enable USB endpoint 7 receiving (OUT).",
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
            name: "Uep5Ctrl",
            extends: None,
            description: Some(
                "endpoint 5 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "Automatic toggle is documented for endpoints 1, 2 and 3 only.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
            name: "Uep5Dma",
            extends: None,
            description: Some(
                "endpoint 5 DMA buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep5_dma",
                    description: Some(
                        "RW, endpoint 5 DMA buffer address.",
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
                "endpoint 5 transmittal length.",
            ),
            bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep6Ctrl",
            extends: None,
            description: Some(
                "endpoint 6 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "Automatic toggle is documented for endpoints 1, 2 and 3 only.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
            name: "Uep6Dma",
            extends: None,
            description: Some(
                "endpoint 6 DMA buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep6_dma",
                    description: Some(
                        "RW, endpoint 6 DMA buffer address.",
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
                "endpoint 6 transmittal length.",
            ),
            bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Uep7Ctrl",
            extends: None,
            description: Some(
                "endpoint 7 control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "mask_uep_t_res",
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
                    name: "mask_uep_r_res",
                    description: Some(
                        "bit mask of handshake response type for USB endpoint X receiving (OUT).",
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
                    name: "uep_auto_tog",
                    description: Some(
                        "Automatic toggle is documented for endpoints 1, 2 and 3 only.",
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
                    name: "uep_t_tog",
                    description: Some(
                        "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1.",
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
                    name: "uep_r_tog",
                    description: Some(
                        "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1.",
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
            name: "Uep7Dma",
            extends: None,
            description: Some(
                "endpoint 7 DMA buffer address.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uep7_dma",
                    description: Some(
                        "RW, endpoint 7 DMA buffer address.",
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
                "endpoint 7 transmittal length.",
            ),
            bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
