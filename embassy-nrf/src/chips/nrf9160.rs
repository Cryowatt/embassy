pub use nrf9160_pac as pac;

/// The maximum buffer size that the EasyDMA can send/recv in one operation.
pub const EASY_DMA_SIZE: usize = (1 << 12) - 1;
pub const FORCE_COPY_BUFFER_SIZE: usize = 1024;

embassy_hal_common::peripherals! {
    // RTC
    RTC0,
    RTC1,

    // WDT
    WDT,

    // UARTE
    UARTE0,
    UARTE1,
    UARTE2,
    UARTE3,

    // TWI
    TWI0,
    TWI1,
    TWI2,
    TWI3,

    // SPI
    SPI0,
    SPI1,
    SPI2,
    SPI3,

    // SAADC
    SAADC,

    // PWM
    PWM0,
    PWM1,
    PWM2,
    PWM3,

    // TIMER
    TIMER0,
    TIMER1,
    TIMER2,

    // GPIOTE
    GPIOTE_CH0,
    GPIOTE_CH1,
    GPIOTE_CH2,
    GPIOTE_CH3,
    GPIOTE_CH4,
    GPIOTE_CH5,
    GPIOTE_CH6,
    GPIOTE_CH7,

    // PPI
    PPI_CH0,
    PPI_CH1,
    PPI_CH2,
    PPI_CH3,
    PPI_CH4,
    PPI_CH5,
    PPI_CH6,
    PPI_CH7,
    PPI_CH8,
    PPI_CH9,
    PPI_CH10,
    PPI_CH11,
    PPI_CH12,
    PPI_CH13,
    PPI_CH14,
    PPI_CH15,

    PPI_GROUP0,
    PPI_GROUP1,
    PPI_GROUP2,
    PPI_GROUP3,
    PPI_GROUP4,
    PPI_GROUP5,

    // GPIO port 0
    P0_00,
    P0_01,
    P0_02,
    P0_03,
    P0_04,
    P0_05,
    P0_06,
    P0_07,
    P0_08,
    P0_09,
    P0_10,
    P0_11,
    P0_12,
    P0_13,
    P0_14,
    P0_15,
    P0_16,
    P0_17,
    P0_18,
    P0_19,
    P0_20,
    P0_21,
    P0_22,
    P0_23,
    P0_24,
    P0_25,
    P0_26,
    P0_27,
    P0_28,
    P0_29,
    P0_30,
    P0_31,
}

impl_uarte!(UARTE0, UARTE0_NS, UARTE0_SPIM0_SPIS0_TWIM0_TWIS0);
impl_uarte!(UARTE1, UARTE1_NS, UARTE1_SPIM1_SPIS1_TWIM1_TWIS1);
impl_uarte!(UARTE2, UARTE2_NS, UARTE2_SPIM2_SPIS2_TWIM2_TWIS2);
impl_uarte!(UARTE3, UARTE3_NS, UARTE3_SPIM3_SPIS3_TWIM3_TWIS3);

impl_spim!(SPI0, SPIM0_NS, UARTE0_SPIM0_SPIS0_TWIM0_TWIS0);
impl_spim!(SPI1, SPIM1_NS, UARTE1_SPIM1_SPIS1_TWIM1_TWIS1);
impl_spim!(SPI2, SPIM2_NS, UARTE2_SPIM2_SPIS2_TWIM2_TWIS2);
impl_spim!(SPI3, SPIM3_NS, UARTE3_SPIM3_SPIS3_TWIM3_TWIS3);

impl_twim!(TWI0, TWIM0_NS, UARTE0_SPIM0_SPIS0_TWIM0_TWIS0);
impl_twim!(TWI1, TWIM1_NS, UARTE1_SPIM1_SPIS1_TWIM1_TWIS1);
impl_twim!(TWI2, TWIM2_NS, UARTE2_SPIM2_SPIS2_TWIM2_TWIS2);
impl_twim!(TWI3, TWIM3_NS, UARTE3_SPIM3_SPIS3_TWIM3_TWIS3);

impl_pwm!(PWM0, PWM0_NS, PWM0);
impl_pwm!(PWM1, PWM1_NS, PWM1);
impl_pwm!(PWM2, PWM2_NS, PWM2);
impl_pwm!(PWM3, PWM3_NS, PWM3);

impl_timer!(TIMER0, TIMER0_NS, TIMER0);
impl_timer!(TIMER1, TIMER1_NS, TIMER1);
impl_timer!(TIMER2, TIMER2_NS, TIMER2);

impl_pin!(P0_00, 0, 0);
impl_pin!(P0_01, 0, 1);
impl_pin!(P0_02, 0, 2);
impl_pin!(P0_03, 0, 3);
impl_pin!(P0_04, 0, 4);
impl_pin!(P0_05, 0, 5);
impl_pin!(P0_06, 0, 6);
impl_pin!(P0_07, 0, 7);
impl_pin!(P0_08, 0, 8);
impl_pin!(P0_09, 0, 9);
impl_pin!(P0_10, 0, 10);
impl_pin!(P0_11, 0, 11);
impl_pin!(P0_12, 0, 12);
impl_pin!(P0_13, 0, 13);
impl_pin!(P0_14, 0, 14);
impl_pin!(P0_15, 0, 15);
impl_pin!(P0_16, 0, 16);
impl_pin!(P0_17, 0, 17);
impl_pin!(P0_18, 0, 18);
impl_pin!(P0_19, 0, 19);
impl_pin!(P0_20, 0, 20);
impl_pin!(P0_21, 0, 21);
impl_pin!(P0_22, 0, 22);
impl_pin!(P0_23, 0, 23);
impl_pin!(P0_24, 0, 24);
impl_pin!(P0_25, 0, 25);
impl_pin!(P0_26, 0, 26);
impl_pin!(P0_27, 0, 27);
impl_pin!(P0_28, 0, 28);
impl_pin!(P0_29, 0, 29);
impl_pin!(P0_30, 0, 30);
impl_pin!(P0_31, 0, 31);

impl_ppi_channel!(PPI_CH0, 0, configurable);
impl_ppi_channel!(PPI_CH1, 1, configurable);
impl_ppi_channel!(PPI_CH2, 2, configurable);
impl_ppi_channel!(PPI_CH3, 3, configurable);
impl_ppi_channel!(PPI_CH4, 4, configurable);
impl_ppi_channel!(PPI_CH5, 5, configurable);
impl_ppi_channel!(PPI_CH6, 6, configurable);
impl_ppi_channel!(PPI_CH7, 7, configurable);
impl_ppi_channel!(PPI_CH8, 8, configurable);
impl_ppi_channel!(PPI_CH9, 9, configurable);
impl_ppi_channel!(PPI_CH10, 10, configurable);
impl_ppi_channel!(PPI_CH11, 11, configurable);
impl_ppi_channel!(PPI_CH12, 12, configurable);
impl_ppi_channel!(PPI_CH13, 13, configurable);
impl_ppi_channel!(PPI_CH14, 14, configurable);
impl_ppi_channel!(PPI_CH15, 15, configurable);

impl_saadc_input!(P0_13, ANALOGINPUT0);
impl_saadc_input!(P0_14, ANALOGINPUT1);
impl_saadc_input!(P0_15, ANALOGINPUT2);
impl_saadc_input!(P0_16, ANALOGINPUT3);
impl_saadc_input!(P0_17, ANALOGINPUT4);
impl_saadc_input!(P0_18, ANALOGINPUT5);
impl_saadc_input!(P0_19, ANALOGINPUT6);
impl_saadc_input!(P0_20, ANALOGINPUT7);

pub mod irqs {
    use crate::pac::Interrupt as InterruptEnum;
    use embassy_macros::interrupt_declare as declare;

    declare!(SPU);
    declare!(CLOCK_POWER);
    declare!(UARTE0_SPIM0_SPIS0_TWIM0_TWIS0);
    declare!(UARTE1_SPIM1_SPIS1_TWIM1_TWIS1);
    declare!(UARTE2_SPIM2_SPIS2_TWIM2_TWIS2);
    declare!(UARTE3_SPIM3_SPIS3_TWIM3_TWIS3);
    declare!(GPIOTE0);
    declare!(SAADC);
    declare!(TIMER0);
    declare!(TIMER1);
    declare!(TIMER2);
    declare!(RTC0);
    declare!(RTC1);
    declare!(WDT);
    declare!(EGU0);
    declare!(EGU1);
    declare!(EGU2);
    declare!(EGU3);
    declare!(EGU4);
    declare!(EGU5);
    declare!(PWM0);
    declare!(PWM1);
    declare!(PWM2);
    declare!(PDM);
    declare!(PWM3);
    declare!(I2S);
    declare!(IPC);
    declare!(FPU);
    declare!(GPIOTE1);
    declare!(KMU);
    declare!(CRYPTOCELL);
}
