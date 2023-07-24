#[doc = "Register `CLK_SYS_RESUS_CTRL` reader"]
pub struct R(crate::R<CLK_SYS_RESUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SYS_RESUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SYS_RESUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SYS_RESUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SYS_RESUS_CTRL` writer"]
pub struct W(crate::W<CLK_SYS_RESUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SYS_RESUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_SYS_RESUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SYS_RESUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub type TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT` writer - This is expressed as a number of clk_ref cycles  
 and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_SYS_RESUS_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENABLE` reader - Enable resus"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable resus"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SYS_RESUS_CTRL_SPEC, bool, O>;
#[doc = "Field `FRCE` reader - Force a resus, for test purposes only"]
pub type FRCE_R = crate::BitReader<bool>;
#[doc = "Field `FRCE` writer - Force a resus, for test purposes only"]
pub type FRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SYS_RESUS_CTRL_SPEC, bool, O>;
#[doc = "Field `CLEAR` reader - For clearing the resus after the fault that triggered it has been corrected"]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - For clearing the resus after the fault that triggered it has been corrected"]
pub type CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SYS_RESUS_CTRL_SPEC, bool, O>;
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
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 8 - Enable resus"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<8> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 12 - Force a resus, for test purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn frce(&mut self) -> FRCE_W<12> {
        FRCE_W::new(self)
    }
    #[doc = "Bit 16 - For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<16> {
        CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_sys_resus_ctrl](index.html) module"]
pub struct CLK_SYS_RESUS_CTRL_SPEC;
impl crate::RegisterSpec for CLK_SYS_RESUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_sys_resus_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_SYS_RESUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_sys_resus_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_SYS_RESUS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_SYS_RESUS_CTRL to value 0xff"]
impl crate::Resettable for CLK_SYS_RESUS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
