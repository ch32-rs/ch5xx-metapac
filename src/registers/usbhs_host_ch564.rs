
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "UsbhsHost",
            extends: None,
            description: Some(
                "USBHS Host mode functions' registers.",
            ),
            items: &[
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "USB host Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "USB host interrupt enable register.",
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
                    name: "dev_addr",
                    description: Some(
                        "USB host device address register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DevAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "control",
                    description: Some(
                        "USB host control register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Control",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "USB HOST interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_st",
                    description: Some(
                        "USB host interrupt status.",
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
                        "USB host miscellaneous status.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "MisSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpm",
                    description: Some(
                        "USB host power management data register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "split",
                    description: Some(
                        "USB host SPLIT register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Split",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "frame",
                    description: Some(
                        "USB host frame register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Frame",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_len",
                    description: Some(
                        "USB host send length register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_len",
                    description: Some(
                        "USB host receive length register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_max_len",
                    description: Some(
                        "USB host receives the maximum length register.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxMaxLen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rx_dma",
                    description: Some(
                        "DMA receive address register.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tx_dma",
                    description: Some(
                        "DMA send address register.",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "TxDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "port_ctrl",
                    description: Some(
                        "USB host port control register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "USB host port Configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "USB host port interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "USB host port test mode register.",
                    ),
                    array: None,
                    byte_offset: 0x33,
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
                        "USB host port status register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PortStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "root_bc_ctrl",
                    description: Some(
                        "USB host BC charging control register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some(
                "USB host Configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "root_link_reset",
                    description: Some(
                        "USB connection controller module resets.",
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
                    name: "root_all_clr",
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
                    name: "root_phy_suspendm",
                    description: Some(
                        "PHY suspend,close utmi clock.",
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
                    name: "root_sof_en",
                    description: Some(
                        "sof packet sending function enabled.",
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
                        "forced full speed FS.",
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
            name: "Control",
            extends: None,
            description: Some(
                "USB host control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid_mask",
                    description: Some(
                        "transaction token packet PID.",
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
                    name: "endp_mask",
                    description: Some(
                        "transaction token packet endpoint number.",
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
                    name: "tog_mask",
                    description: Some(
                        "send data PID.",
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
                        "DATA cache control bit.",
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
                        "host enables the transaction.",
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
                        "valid to send LMP packet.",
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
                        "valid to send SPLIT packet.",
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
                        "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE).",
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
                        "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets.",
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
                        "DATA packets are not expected after IN token packets and. are used for  high-speed SPLIT packets.",
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
                        "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets.",
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
                        "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets.",
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
            name: "DevAddr",
            extends: None,
            description: Some(
                "USB host device address register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dev_addr",
                    description: Some(
                        "device address.",
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
            name: "Frame",
            extends: None,
            description: Some(
                "USB host frame register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "frame_no",
                    description: Some(
                        "the frame number.",
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
                        "microfame number.",
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
                        "SOF count enabled.",
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
                        "the SOF count is cleared.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "USB host interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wkup_act_ie",
                    description: Some(
                        "Wakeup interrupt enabled.",
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
                        "bus recovery interrupt was enabled.",
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
                    name: "usb_act_ie",
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
                    name: "sof_act_ie",
                    description: Some(
                        "sof packet sending interruption is enabled.",
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
                        "send pause interrupt enable.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "USB HOST interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wkup_if",
                    description: Some(
                        "Wakeup interrupt flag.",
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
                        "bus recovery interrupt flag.",
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
                    name: "usb_act_if",
                    description: Some(
                        "USB transaction transfer completion interrupt flag.",
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
                        "sof packet delivert completion interrupt flag.",
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
                        "send pause interrupt flag.",
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
            name: "IntSt",
            extends: None,
            description: Some(
                "USB host interrupt status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "this_pid_mask",
                    description: Some(
                        "received PID.",
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
                        "A bit of indicates that the port received a wakeup signal.",
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
            name: "Lpm",
            extends: None,
            description: Some(
                "USB host power management data register.",
            ),
            bit_size: 32,
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
            ],
        },
        FieldSet {
            name: "MisSt",
            extends: None,
            description: Some(
                "USB host miscellaneous status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "sof_free",
                    description: Some(
                        "port enabled state.",
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
                        "The state of the USB SOF packet is indicated as follows.",
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
                    name: "usb_wakeup",
                    description: Some(
                        "USB bus wakes up.",
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
                        "J on USB bus.",
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
                        "SE0 on USB bus.",
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
            name: "PortCfg",
            extends: None,
            description: Some(
                "USB host port Configuration register.",
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
                        "the 15k resistance drop-down function is enable in host. mode.",
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
                "USB host port control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "set_port_reset",
                    description: Some(
                        "PORT send reset.",
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
                        "the PORT is in a suspended state.",
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
                        "the PORT exits the suspended state.",
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
                        "the PORT goes to sleep(LPM).",
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
                        "PORT exits the enabled state and enters the DISABLED state.",
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
                        "bring the PORT into the port state.",
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
                        "the PORT exits the sleep state (LPM).",
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
                        "wake-up time control.",
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
                        "BUS reset time selection.",
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
                "USB host port interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "port_connect_ie",
                    description: Some(
                        "port connection state change interrupt enabled.",
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
                        "port enable state change interrupt enabled.",
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
                        "port pause state change interrupt enabled.",
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
                        "port reset state change interrupt enabled.",
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
                        "port sleep state change interrupt enabled.",
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
                "USB host port status register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "port_c0nnect",
                    description: Some(
                        "port connection state.",
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
                        "port enabled.",
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
                        "port pause state.",
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
                        "port reset status.",
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
                        "port sleep.",
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
                        "whether the port connection speed is low.",
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
                        "whether the port connection speed is hign.",
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
                        "whether the port is in test mode.",
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
            name: "PortTestCt",
            extends: None,
            description: Some(
                "USB host port test mode register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "test_j",
                    description: Some(
                        "test output J.",
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
                        "test output K.",
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
                        "test mode was enabled.",
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
                "USB host BC charging control register.",
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
                        "UDP pin BC protocol comparator enables.",
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
                        "UDM pin BC protocol comparator enables.",
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
                        "Automatic mode enables.",
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
                        "UDP pin BC protocol source voltage enabled.",
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
                        "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE.",
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
            name: "RxDma",
            extends: None,
            description: Some(
                "DMA receive address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_dma",
                    description: Some(
                        "Received address.",
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
            name: "RxLen",
            extends: None,
            description: Some(
                "USB host receive length register.",
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
            name: "RxMaxLen",
            extends: None,
            description: Some(
                "USB host receives the maximum length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rx_max_len",
                    description: Some(
                        "receives the maximum length.",
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
            name: "Split",
            extends: None,
            description: Some(
                "USB host SPLIT register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "split_data",
                    description: Some(
                        "SPLIT management data.",
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
            name: "TxDma",
            extends: None,
            description: Some(
                "DMA send address register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_dma",
                    description: Some(
                        "Send address.",
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
            name: "TxLen",
            extends: None,
            description: Some(
                "USB host send length register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_len",
                    description: Some(
                        "send data length.",
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
    ],
    enums: &[],
};
                