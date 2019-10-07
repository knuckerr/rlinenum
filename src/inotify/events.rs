use nix::sys::inotify::AddWatchFlags;
use std::collections::HashMap;

lazy_static! {
    pub static ref EVENTS: HashMap<AddWatchFlags, &'static str> = {
        let mut map = HashMap::new();
        map.insert(AddWatchFlags::IN_ACCESS, "ACCESS");
        map.insert(AddWatchFlags::IN_ATTRIB, "ATTRIB");
        map.insert(AddWatchFlags::IN_CLOSE_NOWRITE, "CLOSE_NOWRITE");
        map.insert(AddWatchFlags::IN_CLOSE_WRITE, "CLOSE_WRITE");
        map.insert(AddWatchFlags::IN_CREATE, "CREATE");
        map.insert(AddWatchFlags::IN_DELETE, "DELETE");
        map.insert(AddWatchFlags::IN_DELETE_SELF, "DELETE_SELF");
        map.insert(AddWatchFlags::IN_MODIFY, "MODIFY");
        map.insert(AddWatchFlags::IN_MOVED_FROM, "MOVED_FROM");
        map.insert(AddWatchFlags::IN_MOVED_TO, "MOVED_TO");
        map.insert(AddWatchFlags::IN_MOVE_SELF, "MOVE_SELF");
        map.insert(AddWatchFlags::IN_OPEN, "OPEN");
        map.insert(
            AddWatchFlags::IN_ACCESS | AddWatchFlags::IN_ISDIR,
            "ACCESS DIR",
        );
        map.insert(
            AddWatchFlags::IN_ATTRIB | AddWatchFlags::IN_ISDIR,
            "ATTRIB DIR",
        );
        map.insert(
            AddWatchFlags::IN_CLOSE_NOWRITE | AddWatchFlags::IN_ISDIR,
            "CLOSE_NOWRITE DIR",
        );
        map.insert(
            AddWatchFlags::IN_CLOSE_WRITE | AddWatchFlags::IN_ISDIR,
            "CLOSE_WRITE DIR",
        );
        map.insert(
            AddWatchFlags::IN_CREATE | AddWatchFlags::IN_ISDIR,
            "CREATE DIR",
        );
        map.insert(
            AddWatchFlags::IN_DELETE | AddWatchFlags::IN_ISDIR,
            "DELETE DIR",
        );
        map.insert(
            AddWatchFlags::IN_DELETE_SELF | AddWatchFlags::IN_ISDIR,
            "DELETE_SELF DIR",
        );
        map.insert(
            AddWatchFlags::IN_MODIFY | AddWatchFlags::IN_ISDIR,
            "MODIFY DIR",
        );
        map.insert(
            AddWatchFlags::IN_MOVED_FROM | AddWatchFlags::IN_ISDIR,
            "MOVED_FROM DIR",
        );
        map.insert(
            AddWatchFlags::IN_MOVE_SELF | AddWatchFlags::IN_ISDIR,
            "MODE_SELF DIR",
        );
        map.insert(AddWatchFlags::IN_OPEN | AddWatchFlags::IN_ISDIR, "OPEN DIR");
        map
    };
}
