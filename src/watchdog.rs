#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog control\\n The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.\\n The watchdog can be triggered in software."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    pub load: LOAD,
    #[doc = "0x08 - Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    pub reason: REASON,
    #[doc = "0x0c - Scratch register. Information persists through soft reset of the chip."]
    pub scratch0: SCRATCH0,
    #[doc = "0x10 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch1: SCRATCH1,
    #[doc = "0x14 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch2: SCRATCH2,
    #[doc = "0x18 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch3: SCRATCH3,
    #[doc = "0x1c - Scratch register. Information persists through soft reset of the chip."]
    pub scratch4: SCRATCH4,
    #[doc = "0x20 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch5: SCRATCH5,
    #[doc = "0x24 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch6: SCRATCH6,
    #[doc = "0x28 - Scratch register. Information persists through soft reset of the chip."]
    pub scratch7: SCRATCH7,
    #[doc = "0x2c - Controls the tick generator"]
    pub tick: TICK,
}
#[doc = "Watchdog control\\n The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.\\n The watchdog can be triggered in software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Watchdog control\\n The rst_wdsel register determines which subsystems are reset when the watchdog is triggered.\\n The watchdog can be triggered in software."]
pub mod ctrl;
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1).\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
pub mod load;
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reason](reason) module"]
pub type REASON = crate::Reg<u32, _REASON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REASON;
#[doc = "`read()` method returns [reason::R](reason::R) reader structure"]
impl crate::Readable for REASON {}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
pub mod reason;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch0](scratch0) module"]
pub type SCRATCH0 = crate::Reg<u32, _SCRATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH0;
#[doc = "`read()` method returns [scratch0::R](scratch0::R) reader structure"]
impl crate::Readable for SCRATCH0 {}
#[doc = "`write(|w| ..)` method takes [scratch0::W](scratch0::W) writer structure"]
impl crate::Writable for SCRATCH0 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch0;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch1](scratch1) module"]
pub type SCRATCH1 = crate::Reg<u32, _SCRATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH1;
#[doc = "`read()` method returns [scratch1::R](scratch1::R) reader structure"]
impl crate::Readable for SCRATCH1 {}
#[doc = "`write(|w| ..)` method takes [scratch1::W](scratch1::W) writer structure"]
impl crate::Writable for SCRATCH1 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch1;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch2](scratch2) module"]
pub type SCRATCH2 = crate::Reg<u32, _SCRATCH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH2;
#[doc = "`read()` method returns [scratch2::R](scratch2::R) reader structure"]
impl crate::Readable for SCRATCH2 {}
#[doc = "`write(|w| ..)` method takes [scratch2::W](scratch2::W) writer structure"]
impl crate::Writable for SCRATCH2 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch2;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch3](scratch3) module"]
pub type SCRATCH3 = crate::Reg<u32, _SCRATCH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH3;
#[doc = "`read()` method returns [scratch3::R](scratch3::R) reader structure"]
impl crate::Readable for SCRATCH3 {}
#[doc = "`write(|w| ..)` method takes [scratch3::W](scratch3::W) writer structure"]
impl crate::Writable for SCRATCH3 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch3;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch4](scratch4) module"]
pub type SCRATCH4 = crate::Reg<u32, _SCRATCH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH4;
#[doc = "`read()` method returns [scratch4::R](scratch4::R) reader structure"]
impl crate::Readable for SCRATCH4 {}
#[doc = "`write(|w| ..)` method takes [scratch4::W](scratch4::W) writer structure"]
impl crate::Writable for SCRATCH4 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch4;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch5](scratch5) module"]
pub type SCRATCH5 = crate::Reg<u32, _SCRATCH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH5;
#[doc = "`read()` method returns [scratch5::R](scratch5::R) reader structure"]
impl crate::Readable for SCRATCH5 {}
#[doc = "`write(|w| ..)` method takes [scratch5::W](scratch5::W) writer structure"]
impl crate::Writable for SCRATCH5 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch5;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch6](scratch6) module"]
pub type SCRATCH6 = crate::Reg<u32, _SCRATCH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH6;
#[doc = "`read()` method returns [scratch6::R](scratch6::R) reader structure"]
impl crate::Readable for SCRATCH6 {}
#[doc = "`write(|w| ..)` method takes [scratch6::W](scratch6::W) writer structure"]
impl crate::Writable for SCRATCH6 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch6;
#[doc = "Scratch register. Information persists through soft reset of the chip.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch7](scratch7) module"]
pub type SCRATCH7 = crate::Reg<u32, _SCRATCH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH7;
#[doc = "`read()` method returns [scratch7::R](scratch7::R) reader structure"]
impl crate::Readable for SCRATCH7 {}
#[doc = "`write(|w| ..)` method takes [scratch7::W](scratch7::W) writer structure"]
impl crate::Writable for SCRATCH7 {}
#[doc = "Scratch register. Information persists through soft reset of the chip."]
pub mod scratch7;
#[doc = "Controls the tick generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tick](tick) module"]
pub type TICK = crate::Reg<u32, _TICK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TICK;
#[doc = "`read()` method returns [tick::R](tick::R) reader structure"]
impl crate::Readable for TICK {}
#[doc = "`write(|w| ..)` method takes [tick::W](tick::W) writer structure"]
impl crate::Writable for TICK {}
#[doc = "Controls the tick generator"]
pub mod tick;
