use tock_registers::{register_bitfields, RegisterLongName};

pub trait I2cRegister: RegisterLongName {
    const ADDR: u8;
}

register_bitfields! [
    // First parameter is the register width. Can be u8, u16, u32, or u64.
    u8,

    pub InputSourceControl [
        HighZ OFFSET(7) NUMBITS(1) [],
        InputVoltageLimit OFFSET(3) NUMBITS(4) [
            VINDPM_3_88V = 0b0000,
            VINDPM_3_96V = 0b0001,
            VINDPM_4_04V = 0b0010,
            VINDPM_4_12V = 0b0011,
            VINDPM_4_20V = 0b0100,
            VINDPM_4_28V = 0b0101,
            VINDPM_4_36V = 0b0110,
            VINDPM_4_44V = 0b0111,
            VINDPM_4_52V = 0b1000,
            VINDPM_4_60V = 0b1001,
            VINDPM_4_68V = 0b1010,
            VINDPM_4_76V = 0b1011,
            VINDPM_4_84V = 0b1100,
            VINDPM_4_92V = 0b1101,
            VINDPM_5_00V = 0b1110,
            VINDPM_5_08V = 0b1111,
        ],
        InputCurrentLimit OFFSET(0) NUMBITS(3) [
            IINLIM_100mA = 0b000,
            IINLIM_150mA = 0b001,
            IINLIM_500mA = 0b010,
            IINLIM_900mA = 0b011,
            IINLIM_1200mA = 0b100,
            IINLIM_1500mA = 0b101,
            IINLIM_2000mA = 0b110,
            IINLIM_3000mA = 0b111,
        ]
    ],

    pub PowerOnConfiguration [
        RegisterReset OFFSET(7) NUMBITS(1) [],
        WatchdogReset OFFSET(6) NUMBITS(1) [],
        ChargerConfiguration OFFSET(4) NUMBITS(2) [
            Disabled = 0b00,
            ChargeBattery = 0b01,
            Otg = 0b10,
        ],
        MinSystemVoltageLimit OFFSET(1) NUMBITS(3) [],
        BoostModeCurrentLimit OFFSET(0) NUMBITS(1) [
            BOOST_LIM_500mA = 0,
            BOOST_LIM_1300mA = 1,
        ],
    ],

    pub SystemStatus [
        VBusStatus OFFSET(6) NUMBITS(2) [
            /// Unknown (no input, or DPDM detection incomplete)
            Unknown = 0b00,
            UsbHost = 0b01,
            Adapter = 0b10,
            Otg = 0b11,
        ]
    ]
];

impl I2cRegister for InputSourceControl::Register {
    const ADDR: u8 = 0x00;
}

impl I2cRegister for PowerOnConfiguration::Register {
    const ADDR: u8 = 0x01;
}

// pub struct InputSourceControl {}

// impl RegisterLongName for InputSourceControl {}

// register_bitmasks!(u8, InputSourceControl, [
//     HighZ OFFSET(7) NUMBITS(1) [
//         Disabled = 0,
//         Enabled = 1
//     ],
//     InputVoltageLimit OFFSET(3) NUMBITS(4) [
//         VINDPM_3_88V = 0b0000,
//         VINDPM_3_96V = 0b0001,
//         VINDPM_4_04V = 0b0010,
//         VINDPM_4_12V = 0b0011,
//         VINDPM_4_20V = 0b0100,
//         VINDPM_4_28V = 0b0101,
//         VINDPM_4_36V = 0b0110,
//         VINDPM_4_44V = 0b0111,
//         VINDPM_4_52V = 0b1000,
//         VINDPM_4_60V = 0b1001,
//         VINDPM_4_68V = 0b1010,
//         VINDPM_4_76V = 0b1011,
//         VINDPM_4_84V = 0b1100,
//         VINDPM_4_92V = 0b1101,
//         VINDPM_5_00V = 0b1110,
//         VINDPM_5_08V = 0b1111,
//     ],
//     InputCurrentLimit OFFSET(0) NUMBITS(3) [
//         IINLIM_100mA = 0b000,
//         IINLIM_150mA = 0b001,
//         IINLIM_500mA = 0b010,
//         IINLIM_900mA = 0b011,
//         IINLIM_1200mA = 0b100,
//         IINLIM_1500mA = 0b101,
//         IINLIM_2000mA = 0b110,
//         IINLIM_3000mA = 0b111,
//     ]
// ]);


