#[doc = "Register `VREG` reader"]
pub type R = crate::R<VREG_SPEC>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VREG_SPEC>;
#[doc = "Field `EN` reader - enable  
 0=not enabled, 1=enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable  
 0=not enabled, 1=enabled"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIZ` reader - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
pub type HIZ_R = crate::BitReader;
#[doc = "Field `HIZ` writer - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
pub type HIZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSEL` reader - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
pub type VSEL_R = crate::FieldReader;
#[doc = "Field `VSEL` writer - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
pub type VSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ROK` reader - regulation status  
 0=not in regulation, 1=in regulation"]
pub type ROK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - regulation status  
 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable  
 0=not enabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<VREG_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - high impedance mode select  
 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<VREG_SPEC, 1> {
        HIZ_W::new(self)
    }
    #[doc = "Bits 4:7 - output voltage select  
 0000 to 0101 - 0.80V  
 0110 - 0.85V  
 0111 - 0.90V  
 1000 - 0.95V  
 1001 - 1.00V  
 1010 - 1.05V  
 1011 - 1.10V (default)  
 1100 - 1.15V  
 1101 - 1.20V  
 1110 - 1.25V  
 1111 - 1.30V"]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<VREG_SPEC, 4> {
        VSEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage regulator control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREG to value 0xb1"]
impl crate::Resettable for VREG_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
