
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