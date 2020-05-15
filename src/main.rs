use asl::Address;
use parking_lot::{Mutex, MutexGuard};

// #[derive(ASLState)]
// #[Process = "hl2.exe"]
// pub struct EmptyState {}

// The current iteration of the autosplitter doesn't support multiple process names, so you'll have
// to change this if the name is different. The current name is for Source Unpack under Wine.
const process_name: &'static str = "hl2.exe";

#[no_mangle]
pub extern "C" fn configure() {
    asl::set_process_name(process_name);
    asl::set_tick_rate(100.0); // kinda arbitrary

    // workaround for livesplit-core not detecting disconnect unless there's a pointer path
    // we don't even read/use this path, but livesplit-core does so automatically before ticking.
    asl::push_pointer_path(process_name, &[0], asl::PointerKind::U32);
}

struct PortalState {
    sar_base: Option<Address>,
    current: SARInterface,
    old: SARInterface
}

impl PortalState {
    const fn new() -> Self {
        PortalState {
            sar_base: None,
            current: SARInterface {
                total: 0,
                ipt: 0.0,
                action: TimerState(0),
                //action_time: 0
            },
            old: SARInterface {
                total: 0,
                ipt: 0.0,
                action: TimerState(0),
                //action_time: 0
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum TimerAction {
    DoNothing = 0,
    Start = 1,
    Restart = 2,
    Split = 3,
    End = 4,
    Reset = 5,
    Pause = 6,
    Resume = 7,
}

#[repr(C)]
#[derive(Clone)]
pub struct TimerState(u32);

impl TimerState {
    fn action(&self) -> TimerAction {
       unsafe{ std::mem::transmute(self.0 & 7) }
    }

    fn paused(&self) -> bool {
        if (self.0 & (1<<31)) != 0 { true } else { false }
    }
}

#[repr(C)]
#[derive(Clone)]
struct SARInterface {
    total: i32,
    ipt: f32,
    action: TimerState,
    //action_time: i32,
}

static STATE: Mutex<PortalState> = Mutex::new(PortalState::new());

// TODO: detect SAR unloading.
fn state() -> MutexGuard<'static, PortalState> {
    let mut state = STATE.lock();
    if state.sar_base.is_none() {
        state.sar_base = find_interface();
    }
    state
}


fn find_interface() -> Option<Address> {
    asl::scan_signature(concat!(
        "53 41 52 5F 54 49 4D 45 52 5F 53 54 41 52 54 00", // char start[16]
        "?? ?? ?? ??", // int total
        "?? ?? ?? ??", // float ipt
        "?? ?? ?? ??", // TimerAction action
        //"?? ?? ?? ??", // int action_time
        "53 41 52 5F 54 49 4D 45 52 5F 45 4E 44 00" // char end[14]
    )).map(|a| Address(a.0 + 16))
}

#[no_mangle]
pub extern "C" fn disconnected() {
    *STATE.lock() = PortalState::new();
}

#[no_mangle]
pub extern "C" fn update() {
    let mut state = state();
    if let Some(base) = state.sar_base {
        if let Ok(val) = unsafe { asl::read_val(base) } {
            state.old = state.current.clone();
            state.current = val;
            //assert!(state.current.action as u32 <= 7, "Invalid Action: {}", state.current.action as u32);
        }
    }
}

#[no_mangle]
pub extern "C" fn is_loading() -> bool {
    true // TODO?
}

#[no_mangle]
pub extern "C" fn game_time() -> f64 {
    let state = state();
    state.current.total as f64 * state.current.ipt as f64
}

#[no_mangle]
pub extern "C" fn should_start() -> bool {
    let state = state();
    let test = match (state.current.action.action() != state.old.action.action() /*|| state.current.action_time != state.old.action_time*/, state.current.action.action()) {
        (false, _) => false,
        (true, TimerAction::Start) => true,
        (true, TimerAction::Restart) => true,
        (true, TimerAction::Resume) => true,
        (true, _) => false
    };

    test
}

#[no_mangle]
pub extern "C" fn should_reset() -> bool {
    let state = state();
    match (state.current.action.action() != state.old.action.action() /*|| state.current.action_time != state.old.action_time*/, state.current.action.action()) {
        (false, _) => false,
        (true, TimerAction::Reset) => true,
        (true, TimerAction::Restart) => true,
        (true, _) => false
    }
}

#[no_mangle]
pub extern "C" fn should_split() -> bool {
    let state = state();
    match (state.current.action.action() != state.old.action.action() /*|| state.current.action_time != state.old.action_time*/, state.current.action.action()) {
        (false, _) => false,
        (true, TimerAction::Split) => true,
        (true, TimerAction::End) => true,
        (true, _) => false
    }
}

pub fn main() {}
