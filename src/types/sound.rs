use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Sound {
    Pushover,
    Bike,
    Bugle,
    CashRegister,
    Classical,
    Cosmic,
    Falling,
    Gamelan,
    Incoming,
    Intermission,
    Magic,
    Mechanical,
    PianoBar,
    Siren,
    SpaceAlarm,
    TugBoat,
    Alien,
    Climb,
    Persistent,
    Echo,
    UpDown,
    None,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Sound::Pushover => "pushover",
            Sound::Bike => "bike",
            Sound::Bugle => "bugle",
            Sound::CashRegister => "cashregister",
            Sound::Classical => "classical",
            Sound::Cosmic => "cosmic",
            Sound::Falling => "falling",
            Sound::Gamelan => "gamelan",
            Sound::Incoming => "incoming",
            Sound::Intermission => "intermission",
            Sound::Magic => "magic",
            Sound::Mechanical => "mechanical",
            Sound::PianoBar => "pianobar",
            Sound::Siren => "siren",
            Sound::SpaceAlarm => "spacealarm",
            Sound::TugBoat => "tugboat",
            Sound::Alien => "alien",
            Sound::Climb => "climb",
            Sound::Persistent => "persistent",
            Sound::Echo => "echo",
            Sound::UpDown => "updown",
            Sound::None => "none",
        };

        write!(f, "{}", printable)
    }
}