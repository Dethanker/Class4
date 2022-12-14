trait Duration {
    fn duration(&self) -> u64;
}

enum SignalLight {
    Red,
    Yellow,
    Green,
}

impl Duration for SignalLight {
    fn duration(&self) -> u64 {
        match self {
            SignalLight::Red => 30,
            SignalLight::Yellow => 5,
            SignalLight::Green => 45,
        }
    }
}
