-- Jörmungandr: HTTP GET (Beispiel)
-- Verwendung: jormungandr:http_get(url) -> body string
local url = "https://httpbin.org/get"
local body = jormungandr:http_get(url)
return body or "empty"
