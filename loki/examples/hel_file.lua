-- Hel: Datei schreiben und lesen
-- Verwendung: hel:fs_write(path, contents), hel:fs_read(path)
hel:fs_write("example.txt", "Hello from Loki script")
local content = hel:fs_read("example.txt")
return content or ""
