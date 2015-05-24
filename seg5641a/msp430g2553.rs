#![allow(dead_code)]

macro_rules! port {
    ($name:ident = $value:expr) => {
        pub const $name: *mut u16 = $value as *mut u16;
    };

    ($name:ident = $value:expr; 8) => {
        pub const $name: *mut u8 = $value as *mut u8;
    };

    ($name:ident = $value:expr; 16) => {
        pub const $name: *mut u16 = $value as *mut u16;
    };
}

macro_rules! val {
    ($name:ident = $value:expr) => {
        pub const $name: u16 = $value as u16;
    };

    ($name:ident = $value:expr; 8) => {
        pub const $name: u8 = $value as u8;
    };

    ($name:ident = $value:expr; 16) => {
        pub const $name: u16 = $value as u16;
    };
}

/************************************************************
* STANDARD BITS
************************************************************/
val!( BIT0 = 0x0001 );
val!( BIT1 = 0x0002 );
val!( BIT2 = 0x0004 );
val!( BIT3 = 0x0008 );
val!( BIT4 = 0x0010 );
val!( BIT5 = 0x0020 );
val!( BIT6 = 0x0040 );
val!( BIT7 = 0x0080 );
val!( BIT8 = 0x0100 );
val!( BIT9 = 0x0200 );
val!( BITA = 0x0400 );
val!( BITB = 0x0800 );
val!( BITC = 0x1000 );
val!( BITD = 0x2000 );
val!( BITE = 0x4000 );
val!( BITF = 0x8000 );

/************************************************************
* DIGITAL I/O Port1/2 Pull up / Pull down Resistors
************************************************************/
port!( P1IN   = 0x0020; 8 );  /* Port 1 Input */
port!( P1OUT  = 0x0021; 8 );  /* Port 1 Output */
port!( P1DIR  = 0x0022; 8 );  /* Port 1 Direction */
port!( P1IFG  = 0x0023; 8 );  /* Port 1 Interrupt Flag */
port!( P1IES  = 0x0024; 8 );  /* Port 1 Interrupt Edge Select */
port!( P1IE   = 0x0025; 8 );  /* Port 1 Interrupt Enable */
port!( P1SEL  = 0x0026; 8 );  /* Port 1 Selection */
port!( P1SEL2 = 0x0041; 8 );  /* Port 1 Selection 2 */
port!( P1REN  = 0x0027; 8 );  /* Port 1 Resistor Enable */

port!( P2IN   = 0x0028; 8 );  /* Port 2 Input */
port!( P2OUT  = 0x0029; 8 );  /* Port 2 Output */
port!( P2DIR  = 0x002A; 8 );  /* Port 2 Direction */
port!( P2IFG  = 0x002B; 8 );  /* Port 2 Interrupt Flag */
port!( P2IES  = 0x002C; 8 );  /* Port 2 Interrupt Edge Select */
port!( P2IE   = 0x002D; 8 );  /* Port 2 Interrupt Enable */
port!( P2SEL  = 0x002E; 8 );  /* Port 2 Selection */
port!( P2SEL2 = 0x0042; 8 );  /* Port 2 Selection 2 */
port!( P2REN  = 0x002F; 8 );  /* Port 2 Resistor Enable */

/************************************************************
* WATCHDOG TIMER
************************************************************/
port!( WDTCTL = 0x0120 );

val!( WDTIS0   = 0x0001 );
val!( WDTIS1   = 0x0002 );
val!( WDTSSEL  = 0x0004 );
val!( WDTCNTCL = 0x0008 );
val!( WDTTMSEL = 0x0010 );
val!( WDTNMI   = 0x0020 );
val!( WDTNMIES = 0x0040 );
val!( WDTHOLD  = 0x0080 );

val!( WDTPW    = 0x5A00 );



