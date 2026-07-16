//! Interrupt Hardware Abstraction

use aero_types::AeroResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterruptPriority(pub u8);

impl InterruptPriority {
    pub const CRITICAL: Self = InterruptPriority(0);
    pub const HIGH: Self = InterruptPriority(32);
    pub const MEDIUM: Self = InterruptPriority(128);
    pub const LOW: Self = InterruptPriority(192);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptTrigger {
    Rising,
    Falling,
    Both,
    Level,
}

pub struct InterruptHandler {
    irq: u8,
    priority: InterruptPriority,
    enabled: bool,
}

impl InterruptHandler {
    pub fn new(irq: u8, priority: InterruptPriority) -> AeroResult<Self> {
        Ok(Self {
            irq,
            priority,
            enabled: false,
        })
    }

    pub fn set_priority(&mut self, priority: InterruptPriority) -> AeroResult<()> {
        Self::set_irq_priority(self.irq, priority)?;
        self.priority = priority;
        Ok(())
    }

    pub fn enable(&mut self) -> AeroResult<()> {
        Self::enable_irq(self.irq)?;
        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self) -> AeroResult<()> {
        Self::disable_irq(self.irq)?;
        self.enabled = false;
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn clear_pending(&self) -> AeroResult<()> {
        Self::clear_irq_pending(self.irq)?;
        Ok(())
    }

    pub fn set_trigger(&self, trigger: InterruptTrigger) -> AeroResult<()> {
        Self::set_irq_trigger(self.irq, trigger)?;
        Ok(())
    }

    #[inline(always)]
    fn enable_irq(irq: u8) -> AeroResult<()> {
        let _ = irq;
        Ok(())
    }

    #[inline(always)]
    fn disable_irq(irq: u8) -> AeroResult<()> {
        let _ = irq;
        Ok(())
    }

    #[inline(always)]
    fn set_irq_priority(irq: u8, _priority: InterruptPriority) -> AeroResult<()> {
        let _ = irq;
        Ok(())
    }

    #[inline(always)]
    fn clear_irq_pending(irq: u8) -> AeroResult<()> {
        let _ = irq;
        Ok(())
    }

    #[inline(always)]
    fn set_irq_trigger(irq: u8, _trigger: InterruptTrigger) -> AeroResult<()> {
        let _ = irq;
        Ok(())
    }
}

pub fn init() {}
