#[doc = "Register `SM_INSTR` reader"]
pub type R = crate::R<SM_INSTR_SPEC>;
#[doc = "Register `SM_INSTR` writer"]
pub type W = crate::W<SM_INSTR_SPEC>;
#[doc = "Field `SM0_INSTR` reader - "]
pub type SM0_INSTR_R = crate::FieldReader<u16>;
#[doc = "Field `SM0_INSTR` writer - "]
pub type SM0_INSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm0_instr(&self) -> SM0_INSTR_R {
        SM0_INSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sm0_instr(&mut self) -> SM0_INSTR_W<SM_INSTR_SPEC> {
        SM0_INSTR_W::new(self, 0)
    }
}
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution.  

You can [`read`](crate::Reg::read) this register and get [`sm_instr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sm_instr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_INSTR_SPEC;
impl crate::RegisterSpec for SM_INSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_instr::R`](R) reader structure"]
impl crate::Readable for SM_INSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_instr::W`](W) writer structure"]
impl crate::Writable for SM_INSTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SM_INSTR to value 0"]
impl crate::Resettable for SM_INSTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
