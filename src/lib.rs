#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Session(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InstanceId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Sequence(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CreationId {
    pub instance_id: InstanceId,
    pub sequence: Sequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PaneId(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Pane {
    pub session: Session,
    pub pane_id: PaneId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Root(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Name(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Value(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Variable {
    pub name: Name,
    pub value: Value,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Workspace {
    pub session: Session,
    pub root: Root,
    pub variable_vector: std::vec::Vec<Variable>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AgentName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AgentKind {
    Pi,
    Claude,
    Codex,
    Gemini,
    Cursor,
    Devin,
    Agy,
    Cline,
    Omp,
    Mastracode,
    Opencode,
    Copilot,
    Kimi,
    Kiro,
    Droid,
    Amp,
    Grok,
    Hermes,
    Kilo,
    Qodercli,
    Qwen,
    Maki,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Program(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Executable {
    Configured,
    Explicit(Program),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Command {
    pub agent_name: AgentName,
    pub agent_kind: AgentKind,
    pub executable: Executable,
    pub string_vector: std::vec::Vec<String>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Text(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Keys(pub std::vec::Vec<String>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Label(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum State {
    Open,
    Busy,
    Idle,
    Dead,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Attempt {
    Settled,
    Uncertain,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Known {
    pub pane: Pane,
    pub label_option: std::option::Option<Label>,
    pub state: State,
    pub attempt: Attempt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Panes(pub std::vec::Vec<Known>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Changed {
    pub pane: Pane,
    pub state: State,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ErrorCode(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExitStatus(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Message(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SocketPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Refusal {
    pub error_code: ErrorCode,
    pub exit_status_option: std::option::Option<ExitStatus>,
    pub message: Message,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TransportRefusal {
    pub socket_path: SocketPath,
    pub message: Message,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryRefusal(pub Message);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ObservedEffect {
    Done,
    Created(Pane),
    Listed(std::vec::Vec<Known>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum StorePhase {
    BeforeEmission,
    Observed(ObservedEffect),
    Uncertain,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StoreRefusal {
    pub memory_refusal: MemoryRefusal,
    pub store_phase: StorePhase,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Outcome {
    Done,
    Created(Pane),
    Listed(std::vec::Vec<Known>),
    Refused(Refusal),
    Unknown(Pane),
    AttemptInProgress(Pane),
    Unavailable(TransportRefusal),
    Uncertain,
    StoreRefused(StoreRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Start_Data {
    pub pane: Pane,
    pub command: Command,
    pub label: Label,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Prompt_Data {
    pub pane: Pane,
    pub text: Text,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Press_Data {
    pub pane: Pane,
    pub keys: Keys,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Create(Workspace),
    Start(Start_Data),
    Prompt(Prompt_Data),
    Press(Press_Data),
    Close(Pane),
    List(Session),
    Subscribe(Session),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Created(Outcome),
    Started(Outcome),
    Prompted(Outcome),
    Pressed(Outcome),
    Closed(Outcome),
    Listed(Outcome),
    Subscribed(signal::Frame<Panes, Changed>),
    SubscribeRefused(Outcome),
}
#[rustfmt::skip]
#[allow(clippy::large_enum_variant)]
pub enum Answer {
    Once(Output),
    Subscribed(signal::Stream<Panes, Changed>),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn create(&self, workspace: &Workspace) -> std::result::Result<Outcome, Output>;
    fn start(&self, start: &Start_Data) -> std::result::Result<Outcome, Output>;
    fn prompt(&self, prompt: &Prompt_Data) -> std::result::Result<Outcome, Output>;
    fn press(&self, press: &Press_Data) -> std::result::Result<Outcome, Output>;
    fn close(&self, pane: &Pane) -> std::result::Result<Outcome, Output>;
    fn list(&self, session: &Session) -> std::result::Result<Outcome, Output>;
    fn subscribe(
        &self,
        session: &Session,
    ) -> std::result::Result<signal::Stream<Panes, Changed>, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    type Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Self::Answer;
}
#[rustfmt::skip]
impl Answerable for Input {
    type Answer = Answer;
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Answer {
        match self {
            Input::Create(carried) => {
                Answer::Once(
                    nexus.create(carried).map_or_else(|refusal| refusal, Output::Created),
                )
            }
            Input::Start(carried) => {
                Answer::Once(
                    nexus.start(carried).map_or_else(|refusal| refusal, Output::Started),
                )
            }
            Input::Prompt(carried) => {
                Answer::Once(
                    nexus
                        .prompt(carried)
                        .map_or_else(|refusal| refusal, Output::Prompted),
                )
            }
            Input::Press(carried) => {
                Answer::Once(
                    nexus.press(carried).map_or_else(|refusal| refusal, Output::Pressed),
                )
            }
            Input::Close(carried) => {
                Answer::Once(
                    nexus.close(carried).map_or_else(|refusal| refusal, Output::Closed),
                )
            }
            Input::List(carried) => {
                Answer::Once(
                    nexus.list(carried).map_or_else(|refusal| refusal, Output::Listed),
                )
            }
            Input::Subscribe(carried) => {
                nexus.subscribe(carried).map_or_else(Answer::Once, Answer::Subscribed)
            }
        }
    }
}
pub const ETHOS: &str = include_str!("../signal.ethos");

impl signal::Contracted for Input {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

/// A subscription's state follows its changes: a pane whose change is Dead
/// leaves the panes, so Dead is told once and no later state holds it; a
/// living state replaces the pane's own, and a pane not yet held is taken in
/// with that state, no label and its attempt settled.
impl signal::Following for Panes {
    type Change = Changed;

    fn after(panes: Self, changed: Changed) -> Self {
        let Panes(mut known) = panes;
        let held = known.iter().position(|pane| pane.pane == changed.pane);
        match (changed.state, held) {
            (State::Dead, Some(at)) => {
                known.remove(at);
            }
            (State::Dead, None) => {}
            (state, Some(at)) => known[at].state = state,
            (state, None) => known.push(Known {
                pane: changed.pane,
                label_option: None,
                state,
                attempt: Attempt::Settled,
            }),
        }
        Panes(known)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Attempt, Changed, ErrorCode, ExitStatus, Input, Known, Label, Message, Outcome, Output,
        Pane, PaneId, Panes, Refusal, Session, State,
    };
    use signal::{ByteViewable, Following, Frame, Restorable, Signal, Signalizable};

    fn pane(name: &str) -> Pane {
        Pane {
            session: Session("main".into()),
            pane_id: PaneId(name.into()),
        }
    }

    fn known(name: &str, state: State) -> Known {
        Known {
            pane: pane(name),
            label_option: Some(Label(name.into())),
            state,
            attempt: Attempt::Settled,
        }
    }

    fn changed(name: &str, state: State) -> Changed {
        Changed {
            pane: pane(name),
            state,
        }
    }

    #[test]
    fn public_query_and_response_restore_through_validated_signal() {
        let query = Input::Close(Pane {
            session: Session("main".into()),
            pane_id: PaneId("wQ:pT".into()),
        });
        let query_signal = query.signalize().expect("archive public query");
        let restored_query = Signal::<Input>::from(query_signal.bytes().to_vec())
            .restore()
            .expect("validate and restore public query");
        assert_eq!(restored_query, query);

        let response = Output::Closed(Outcome::Refused(Refusal {
            error_code: ErrorCode("pane_close_refused".into()),
            exit_status_option: Some(ExitStatus(17)),
            message: Message("pane stayed open".into()),
        }));
        let response_signal = response.signalize().expect("archive public response");
        let restored_response = Signal::<Output>::from(response_signal.bytes().to_vec())
            .restore()
            .expect("validate and restore public response");
        assert_eq!(restored_response, response);
    }

    #[test]
    fn a_living_change_replaces_the_pane_state_and_keeps_its_label() {
        let panes = Panes(vec![known("a", State::Open), known("b", State::Idle)]);
        let after = Panes::after(panes, changed("a", State::Busy));
        assert_eq!(
            after,
            Panes(vec![known("a", State::Busy), known("b", State::Idle)])
        );
    }

    #[test]
    fn a_dead_change_removes_the_pane_once() {
        let panes = Panes(vec![known("a", State::Open), known("b", State::Idle)]);
        let after = Panes::after(panes, changed("a", State::Dead));
        assert_eq!(after, Panes(vec![known("b", State::Idle)]));
        let again = Panes::after(after.clone(), changed("a", State::Dead));
        assert_eq!(again, after);
    }

    #[test]
    fn a_pane_not_yet_held_is_taken_in() {
        let after = Panes::after(Panes(Vec::new()), changed("c", State::Open));
        assert_eq!(
            after,
            Panes(vec![Known {
                pane: pane("c"),
                label_option: None,
                state: State::Open,
                attempt: Attempt::Settled,
            }])
        );
    }

    #[test]
    fn every_subscription_frame_restores_through_validated_signal() {
        for frame in [
            Frame::Opened(Panes(vec![known("a", State::Open)])),
            Frame::Changed(changed("a", State::Dead)),
            Frame::Lagged(Panes(Vec::new())),
        ] {
            let output = Output::Subscribed(frame);
            let archived = output.signalize().expect("archive a subscription frame");
            let restored = Signal::<Output>::from(archived.bytes().to_vec())
                .restore()
                .expect("validate and restore a subscription frame");
            assert_eq!(restored, output);
        }
        let refused = Output::SubscribeRefused(Outcome::Unknown(pane("a")));
        let archived = refused.signalize().expect("archive a subscription refusal");
        let restored = Signal::<Output>::from(archived.bytes().to_vec())
            .restore()
            .expect("validate and restore a subscription refusal");
        assert_eq!(restored, refused);
    }
}
