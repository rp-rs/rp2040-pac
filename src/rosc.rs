#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ring Oscillator control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength"]
    pub freqa: FREQA,
    #[doc = "0x08 - For a detailed description see freqa register"]
    pub freqb: FREQB,
    #[doc = "0x0c - Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode"]
    pub dormant: DORMANT,
    #[doc = "0x10 - Controls the output divider"]
    pub div: DIV,
    #[doc = "0x14 - Controls the phase shifted output"]
    pub phase: PHASE,
    #[doc = "0x18 - Ring Oscillator Status"]
    pub status: STATUS,
    #[doc = "0x1c - This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
    pub randombit: RANDOMBIT,
}
#[doc = "Ring Oscillator control  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Ring Oscillator control"]
pub mod ctrl;
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [freqa](freqa) module"]
pub type FREQA = crate::Reg<u32, _FREQA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQA;
#[doc = "`read()` method returns [freqa::R](freqa::R) reader structure"]
impl crate::Readable for FREQA {}
#[doc = "`write(|w| ..)` method takes [freqa::W](freqa::W) writer structure"]
impl crate::Writable for FREQA {}
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage  
 The drive strength has 4 levels determined by the number of bits set  
 Increasing the number of bits set increases the drive strength and increases the oscillation frequency  
 0 bits set is the default drive strength  
 1 bit set doubles the drive strength  
 2 bits set triples drive strength  
 3 bits set quadruples drive strength"]
pub mod freqa;
#[doc = "For a detailed description see freqa register  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [freqb](freqb) module"]
pub type FREQB = crate::Reg<u32, _FREQB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQB;
#[doc = "`read()` method returns [freqb::R](freqb::R) reader structure"]
impl crate::Readable for FREQB {}
#[doc = "`write(|w| ..)` method takes [freqb::W](freqb::W) writer structure"]
impl crate::Writable for FREQB {}
#[doc = "For a detailed description see freqa register"]
pub mod freqb;
#[doc = "Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dormant](dormant) module"]
pub type DORMANT = crate::Reg<u32, _DORMANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DORMANT;
#[doc = "`read()` method returns [dormant::R](dormant::R) reader structure"]
impl crate::Readable for DORMANT {}
#[doc = "`write(|w| ..)` method takes [dormant::W](dormant::W) writer structure"]
impl crate::Writable for DORMANT {}
#[doc = "Ring Oscillator pause control  
 This is used to save power by pausing the ROSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 Warning: setup the irq before selecting dormant mode"]
pub mod dormant;
#[doc = "Controls the output divider  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "Controls the output divider"]
pub mod div;
#[doc = "Controls the phase shifted output  

This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [phase](phase) module"]
pub type PHASE = crate::Reg<u32, _PHASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PHASE;
#[doc = "`read()` method returns [phase::R](phase::R) reader structure"]
impl crate::Readable for PHASE {}
#[doc = "`write(|w| ..)` method takes [phase::W](phase::W) writer structure"]
impl crate::Writable for PHASE {}
#[doc = "Controls the phase shifted output"]
pub mod phase;
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [randombit](randombit) module"]
pub type RANDOMBIT = crate::Reg<u32, _RANDOMBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOMBIT;
#[doc = "`read()` method returns [randombit::R](randombit::R) reader structure"]
impl crate::Readable for RANDOMBIT {}
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
pub mod randombit;
#[doc = "Ring Oscillator Status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Ring Oscillator Status"]
pub mod status;
