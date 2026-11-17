//! The ordinary capsule-nexus contract reads and prints datom text, and refuses wrong shapes.

#![cfg(feature = "datom")]

use datom_codec::{Actualizing, Budget, Composing, Datomizable, Path, Potential};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal::Frame;
use signal_capsule_nexus::{
    Attempt, Input, Known, Label, Output, Pane, PaneId, Panes, Session, State,
};

fn budget() -> Budget {
    Budget {
        remaining: 4096,
        reader: ReaderBudget::try_from(4096).expect("the reader budget is within range"),
        depth: 0,
        maximum_depth: 4096,
    }
}

fn read<T: Composing>(source: &str) -> Result<T, datom_codec::Error> {
    Potential::<T>::from(source).actualize(&mut budget())
}

fn print<T: Datomizable>(value: &T) -> String {
    value.datomize(Path::new()).protosize().compact()
}

fn round_trip<T: Composing + Datomizable + std::fmt::Debug + PartialEq>(source: &str) {
    let value = read::<T>(source).unwrap_or_else(|error| panic!("{source}: {error:?}"));
    assert_eq!(
        read::<T>(&print(&value)).expect("the printing reads back"),
        value,
        "{source}"
    );
}

#[test]
fn every_query_shape_round_trips() {
    round_trip::<Input>("Close.{ main wQ:pT }");
    round_trip::<Input>("List.main");
    round_trip::<Input>("Subscribe.main");
    round_trip::<Input>("Press.{ { main wQ:pT } [ Enter ] }");
    round_trip::<Input>("Prompt.{ { main wQ:pT } «hello world» }");
}

#[test]
fn a_response_round_trips() {
    round_trip::<Output>("Listed.AttemptInProgress.{ main wQ:pT }");
    round_trip::<Output>("SubscribeRefused.Unknown.{ main wQ:pT }");
}

#[test]
fn a_subscription_reads_as_its_frames() {
    round_trip::<Output>("Subscribed.Opened.[]");
    round_trip::<Output>("Subscribed.Changed.{ { main wQ:pT } Idle }");
    round_trip::<Output>("Subscribed.Lagged.[]");
    let opened = Output::Subscribed(Frame::Opened(Panes(vec![Known {
        pane: Pane {
            session: Session("main".into()),
            pane_id: PaneId("wQ:pT".into()),
        },
        label_option: Some(Label("designer".into())),
        state: State::Busy,
        attempt: Attempt::Settled,
    }])));
    let printed = print(&opened);
    assert_eq!(
        read::<Output>(&printed).expect("the printing reads back"),
        opened
    );
}

#[test]
fn wrong_shapes_and_unknown_variants_are_refused() {
    assert!(read::<Input>("Close.main").is_err());
    assert!(read::<Input>("Frobnicate.main").is_err());
    assert!(read::<Input>("Press.{ main }").is_err());
    assert!(read::<Output>("Subscribed.Changed.{ { main wQ:pT } Sleeping }").is_err());
    assert!(read::<Output>("Changed.{ { main wQ:pT } Idle }").is_err());
}
