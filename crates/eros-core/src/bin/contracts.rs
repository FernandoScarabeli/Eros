use eros_core::*;
use std::path::Path;
use ts_rs::TS;

fn main() {
    let mut text =
        String::from("// Generated from eros-core. Run cargo run -p eros-core --bin contracts.\n");
    text.push_str(&format!("export const MAX_PANELS = {};\n", MAX_PANELS));
    macro_rules! add { ($($ty:ty),*) => { $(text.push_str("export "); text.push_str(&<$ty>::decl()); text.push('\n');)* }; }
    add!(
        Project,
        SessionKind,
        SessionState,
        ChatStatus,
        ChatActivityEvent,
        Conversation,
        Panel,
        Axis,
        Layout,
        Workspace,
        AuthenticatedProvider,
        SidebarOrders,
        SidebarGroup,
        Snapshot,
        TerminalEvent
    );
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../apps/desktop/src/lib/contracts.ts");
    if std::env::args().any(|arg| arg == "--check") {
        assert_eq!(
            std::fs::read_to_string(root).expect("generate contracts first"),
            text,
            "generated contracts differ"
        );
    } else {
        std::fs::create_dir_all(root.parent().unwrap()).unwrap();
        std::fs::write(root, text).unwrap();
    }
}
