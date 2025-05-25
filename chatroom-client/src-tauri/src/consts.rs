pub struct EventNames;

type EventName = &'static str;

impl EventNames {
    pub const MESSAGE_RECEIVED: EventName = "message";
    pub const KICKED: EventName = "kicked";
    pub const STATUS_UPD: EventName = "status";
    pub const SERVER_RESULT: EventName = "server_result";
}
