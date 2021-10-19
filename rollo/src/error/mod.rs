#[derive(Debug, Clone, Copy)]
pub enum Error {
    PacketSize,
    NumberConversion,
    ReadingPacket,
    DosProtection,
    TimeoutReading,
    Channel,
    PacketPayload,
    TlsAcceptTimeout,
    NoDelayError,
    TlsAccept,
}
