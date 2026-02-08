-- Hel: Cache setzen, lesen, invalidieren
-- Verwendung: hel:cache_set(key, value), hel:cache_get(key), hel:cache_invalidate(key)
hel:cache_set("greeting", "Hello, World!")
local value = hel:cache_get("greeting")
hel:cache_invalidate("greeting")
return value or ""
