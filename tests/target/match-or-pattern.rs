enum LongerName {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

enum Wrap {
    A(LongerName),
    B(
        LongerName,
        LongerName,
        LongerName,
        LongerName,
        LongerName,
        LongerName,
        LongerName,
        LongerName,
        LongerName,
    ),
}

fn tc1(wrap: Wrap) {
    match wrap {
        Wrap::A(
            LongerName::One
            | LongerName::Two
            | LongerName::Three
            | LongerName::Four
            | LongerName::Five
            | LongerName::Six
            | LongerName::Seven
            | LongerName::Eight
            | LongerName::Nine
        ) => {}
    }
}

fn tc2(wrap: Wrap) {
    match wrap {
        Wrap::A(LongerName::One)
        | Wrap::A(LongerName::Two)
        | Wrap::A(LongerName::Three)
        | Wrap::A(LongerName::Four)
        | Wrap::A(LongerName::Five)
        | Wrap::A(LongerName::Six)
        | Wrap::A(LongerName::Seven)
        | Wrap::A(LongerName::Eight)
        | Wrap::A(LongerName::Nine) => {}
    }
}

fn tc3(wrap: Wrap) {
    match wrap {
        Wrap::B(
            LongerName::One,
            LongerName::Two,
            LongerName::Three,
            LongerName::Four,
            LongerName::Five,
            LongerName::Six,
            LongerName::Seven,
            LongerName::Eight,
            LongerName::Nine,
        ) => {}
        _ => {}
    }
}

fn tc4(wrap: Wrap) -> bool {
    matches!(
        wrap,
        Wrap::A(
            LongerName::One
                | LongerName::Two
                | LongerName::Three
                | LongerName::Four
                | LongerName::Five
                | LongerName::Six
                | LongerName::Seven
                | LongerName::Eight
                | LongerName::Nine
        ),
    )
}
