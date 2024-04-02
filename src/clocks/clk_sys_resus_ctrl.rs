#[doc = "Register `CLK_SYS_RESUS_CTRL` reader"]
pub type R = crate::R<CLK_SYS_RESUS_CTRL_SPEC>;
#[doc = "Register `CLK_SYS_RESUS_CTRL` writer"]
pub type W = crate::W<CLK_SYS_RESUS_CTRL_SPEC>;
#[doc = "Field `TIMEOUT` reader - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub type TIMEOUT_R = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENABLE` reader - Enable resus"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable resus"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCE` reader - Force a resus, for test purposes only"]
pub type FRCE_R = crate::BitReader;
#[doc = "Field `FRCE` writer - Force a resus, for test purposes only"]
pub type FRCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` reader - For clearing the resus after the fault that triggered it has been corrected"]
pub type CLEAR_R = crate::BitReader;
#[doc = "Field `CLEAR` writer - For clearing the resus after the fault that triggered it has been corrected"]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn frce(&self) -> FRCE_R {
        FRCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<CLK_SYS_RESUS_CTRL_SPEC> {
        TIMEOUT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CLK_SYS_RESUS_CTRL_SPEC> {
        ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn frce(&mut self) -> FRCE_W<CLK_SYS_RESUS_CTRL_SPEC> {
        FRCE_W::new(self, 12)
    }
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CLK_SYS_RESUS_CTRL_SPEC> {
        CLEAR_W::new(self, 16)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`clk_sys_resus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_sys_resus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SYS_RESUS_CTRL_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sys_resus_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_sys_resus_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_SYS_RESUS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_SYS_RESUS_CTRL to value 0xff"]
impl crate::Resettable for CLK_SYS_RESUS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
