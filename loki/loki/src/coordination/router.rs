//! Script router – determines which sub-service APIs a script needs (Phase 9.2.1).

/// Which sub-service APIs a script uses (detected by string scan).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptRoute {
    pub fenrir: bool,
    pub jormungandr: bool,
    pub hel: bool,
}

impl ScriptRoute {
    pub fn needs_fenrir(&self) -> bool {
        self.fenrir
    }
    pub fn needs_jormungandr(&self) -> bool {
        self.jormungandr
    }
    pub fn needs_hel(&self) -> bool {
        self.hel
    }
}

/// Route script source to required sub-services by scanning for API usage.
pub fn route(script_source: &str) -> ScriptRoute {
    let s = script_source;
    ScriptRoute {
        fenrir: s.contains("fenrir:") || s.contains("fenrir."),
        jormungandr: s.contains("jormungandr:") || s.contains("jormungandr."),
        hel: s.contains("hel:") || s.contains("hel."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_detects_fenrir() {
        let r = route("fenrir:gpio_read(1)");
        assert!(r.needs_fenrir());
        assert!(!r.needs_jormungandr());
        assert!(!r.needs_hel());
    }

    #[test]
    fn route_detects_jormungandr() {
        let r = route("jormungandr:http_get(\"http://x\")");
        assert!(!r.needs_fenrir());
        assert!(r.needs_jormungandr());
        assert!(!r.needs_hel());
    }

    #[test]
    fn route_detects_hel() {
        let r = route("hel:storage_get(\"k\")");
        assert!(!r.needs_fenrir());
        assert!(!r.needs_jormungandr());
        assert!(r.needs_hel());
    }

    #[test]
    fn route_detects_multiple() {
        let r = route("fenrir:gpio_write(1, true); hel:cache_set(\"x\", \"y\")");
        assert!(r.needs_fenrir());
        assert!(!r.needs_jormungandr());
        assert!(r.needs_hel());
    }

    #[test]
    fn route_no_apis() {
        let r = route("return 42");
        assert!(!r.needs_fenrir());
        assert!(!r.needs_jormungandr());
        assert!(!r.needs_hel());
    }
}
