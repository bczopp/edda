-- Fenrir: Sensor auslesen (z. B. Temperatur)
-- Verwendung: fenrir:sensor_read(sensor_id) -> number
local temp = fenrir:sensor_read("temp1")
return "Temperature: " .. tostring(temp) .. " C"
