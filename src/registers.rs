use tock_registers::{register_bitfields, RegisterLongName};

pub trait I2cRegister: RegisterLongName {
    const ADDR: u8;
}

register_bitfields! [
    // First parameter is the register width. Can be u8, u16, u32, or u64.
    u8,

    /// Input Source Control Register
    pub REG00 [
        EN_HIZ OFFSET(7) NUMBITS(1) [],
        VINDPM OFFSET(3) NUMBITS(4) [
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
        IINLIM OFFSET(0) NUMBITS(3) [
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

    /// Power-On Configuration Register
    pub REG01 [
        RegisterReset OFFSET(7) NUMBITS(1) [],
        WatchdogReset OFFSET(6) NUMBITS(1) [],
        CHG_CONFIG OFFSET(4) NUMBITS(2) [
            Disabled = 0b00,
            ChargeBattery = 0b01,
            Otg = 0b10,
        ],
        SYS_MIN OFFSET(1) NUMBITS(3) [],
        BOOST_LIM OFFSET(0) NUMBITS(1) [
            BOOST_LIM_500mA = 0,
            BOOST_LIM_1300mA = 1,
        ],
    ],

    /// Charge Current Control Register
    pub REG02 [
        /// Fast Charge Current Limit
        ICHG OFFSET(2) NUMBITS(6) [],
        FORCE_20PCT OFFSET(0) NUMBITS(1) [],
    ],

    /// Pre-Charge/Termination Current Control Register
    pub REG03 [
        /// Pre-Charge Curent Limit
        IPRECHG OFFSET(4) NUMBITS(4) [],
        /// Termination Current Limit
        ITERM OFFSET(0) NUMBITS(4) [],
    ],

    /// Charge Voltage Control Register
    pub REG04 [
        /// Charge Voltage Limit
        VREG OFFSET(2) NUMBITS(6) [],
        /// Battery Precharge to Fast Charge Threshold
        BATLOWV OFFSET(1) NUMBITS(1) [
            BATLOWV_2_8V = 0,
            BATLOWV_3_0V = 1,
        ],
        /// Battery Recharge Threshold (below battery regulation voltage)
        VRECHG OFFSET(0) NUMBITS(1) [
            VRECHG_100mV = 0,
            VRECHG_300mV = 1,
        ],
    ],

    /// Charge Termination/Timer Control Register
    pub REG05 [
        /// Charging Termination Enable
        EN_TERM OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        /// Termination Indicator Threshold
        TERM_STAT OFFSET(6) NUMBITS(1) [
            /// STAT pin high when ITERM matches
            MATCH_ITERM = 0,
            /// STAT pin high before actual termination when charge current below 800 mA
            BEFORE_TERM = 1,
        ],
        /// I2C Watchdog Timer Setting
        WATCHDOG OFFSET(4) NUMBITS(2) [
            Disabled = 0b00,
            Timer_40s = 0b01,
            Timer_80s = 0b10,
            Timer_160s = 0b11,
        ],
        /// Charging Safety Timer Enable
        EN_TIMER OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1,
        ],
        /// Fast Charge Timer Setting
        CHG_TIMER OFFSET(1) NUMBITS(2) [
            Timer_5h = 0b00,
            Timer_8h = 0b01,
            Timer_12h = 0b10,
            Timer_20h = 0b11,
        ]
    ],

    /// Thermal Regulation Control Register
    pub REG06 [
        /// Thermal Regulation Threshold
        TREG OFFSET(0) NUMBITS(2) [
            TREG_60C = 0b00,
            TREG_80C = 0b01,
            TREG_100C = 0b10,
            TREG_120C = 0b11,
        ]
    ],

    /// Misc Operation Control Register
    pub REG07 [
        /// Set default input current limit from PSEL/OTG pins
        DPDM_EN OFFSET(7) NUMBITS(1) [
            /// Not in D+/D– detection
            NoDetection = 0,
            /// Force D+/D– detection
            ForceDetection = 1,
        ],
        /// Safety Timer Setting during Input DPM and Thermal Regulation
        TMR2X_EN OFFSET(6) NUMBITS(1) [
            /// Safety timer not slowed by 2X during input DPM or thermal regulation
            Disabled = 0,
            /// Safety timer slowed by 2X during input DPM or thermal regulation
            Enabled = 1,
        ],
        /// Force BATFET Off
        BATFET_Disable OFFSET(5) NUMBITS(1) [
            AllowQ4On = 0,
            TurnOffQ4 = 1,
        ],
        /// CHRG_FAULT IRT control
        INT_MASK1 OFFSET(1) NUMBITS(1) [
            /// No INT during CHRG_FAULT
            Disabled = 0,
            /// INT on CHRG_FAULT
            Enabled = 1,
        ],
        /// BAT_FAULT INT control
        INT_MASK0 OFFSET(0) NUMBITS(1) [
            /// No INT during BAT_FAULT
            Disabled = 0,
            /// INT on BAT_FAULT
            Enabled = 1,
        ],
    ],

    /// System Status Register
    pub REG08 [
        VBUS_STAT OFFSET(6) NUMBITS(2) [
            /// Unknown (no input, or DPDM detection incomplete)
            Unknown = 0b00,
            UsbHost = 0b01,
            Adapter = 0b10,
            Otg = 0b11,
        ],
        CHRG_STAT OFFSET(4) NUMBITS(2) [
            NotCharging = 0b00,
            PreCharge = 0b01,
            FastCharging = 0b10,
            ChargeTerminationDone = 0b11,
        ],
        DPM_STAT OFFSET(3) NUMBITS(1) [
            NotDpm = 0,
            /// VINDPM or IINDPM
            Dpm = 1,
        ],
        PG_STAT OFFSET(2) NUMBITS(1) [
            NotPowerGood = 0,
            PowerGood = 1,
        ],
        THERM_STAT OFFSET(3) NUMBITS(1) [
            Normal = 0,
            ThermalRegulation = 1,
        ],
        VSYS_STAT OFFSET(3) NUMBITS(1) [
            /// Not in VSYSMIN regulation (BAT > VSYSMIN)
            NoRegulation = 0,
            /// – In VSYSMIN regulation (BAT < VSYSMIN)
            Regulation = 1,
        ],
    ],

    /// Fault Register
    pub REG09 [
        WATCHDOG_FAULT OFFSET(7) NUMBITS(1) [
            Normal = 0,
            TimerExpired = 1,
        ],
        BOOST_FAULT OFFSET(6) NUMBITS(1) [
            Normal = 0,
            /// VBUS overloaded (OCP), or VBUS OVP in boost mode
            Fault = 1,
        ],
        CHRG_FAULT OFFSET(4) NUMBITS(2) [
            Normal = 0b00,
            InputFault = 0b01,
            ThermalShutdown = 0b10,
            SafetyTimer = 0b11,
        ],
        BAT_FAULT OFFSET(3) NUMBITS(1) [],
        NTC_FAULT OFFSET(0) NUMBITS(3) [
            Normal = 0b000,
            Cold = 0b101,
            Hot = 0b110,
        ],
    ],

    /// Vender / Part / Revision Status Register
    pub REG0A [
        PN OFFSET(3) NUMBITS(3) [
            DEFAULT_PN = 0b101,
        ],
        TS_PROFILE OFFSET(2) NUMBITS(1) [],
        DEV_REG OFFSET(0) NUMBITS(2) [],
    ]
];

impl I2cRegister for REG00::Register {
    const ADDR: u8 = 0x00;
}

impl I2cRegister for REG01::Register {
    const ADDR: u8 = 0x01;
}

impl I2cRegister for REG02::Register {
    const ADDR: u8 = 0x02;
}

impl I2cRegister for REG03::Register {
    const ADDR: u8 = 0x03;
}

impl I2cRegister for REG04::Register {
    const ADDR: u8 = 0x04;
}

impl I2cRegister for REG05::Register {
    const ADDR: u8 = 0x05;
}

impl I2cRegister for REG06::Register {
    const ADDR: u8 = 0x06;
}

impl I2cRegister for REG07::Register {
    const ADDR: u8 = 0x07;
}

impl I2cRegister for REG08::Register {
    const ADDR: u8 = 0x08;
}

impl I2cRegister for REG09::Register {
    const ADDR: u8 = 0x09;
}

impl I2cRegister for REG0A::Register {
    const ADDR: u8 = 0x0A;
}
