/// Version
#[derive(Debug)]
pub enum CorrelationVectorVersion {
    V1,
    V2,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum SpinEntropy {
    /// Do not generate entropy as part of the spin value.
    None = 0,

    /// Generate entropy using 8 bits.
    One = 1,

    /// Generate entropy using 16 bits.
    Two = 2,

    /// Generate entropy using 24 bits.
    Three = 3,

    /// Generate entropy using 32 bits.
    Four = 4,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum SpinCounterInterval
{
    /// The coarse interval drops the 24 least significant bits in DateTime.Ticks
    /// resulting in a counter that increments every 1.67 seconds.
    Coarse,

    /// The fine interval drops the 16 least significant bits in DateTime.Ticks
    /// resulting in a counter that increments every 6.5 milliseconds.
    Fine,
}

impl Default for SpinCounterInterval {
    fn default() -> SpinCounterInterval { SpinCounterInterval::Fine }
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum SpinCounterPeriodicity
{
    /// Do not store a counter as part of the spin value.
    None,

    /// The short periodicity stores the counter using 16 bits.
    Short,

    /// The medium periodicity stores the counter using 24 bits.
    Medium,

    /// The long periodicity stores the counter using 32 bits.
    Long,
}

impl Default for SpinCounterPeriodicity {
    fn default() -> SpinCounterPeriodicity { SpinCounterPeriodicity::None }
}