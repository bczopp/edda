-- Fenrir: LED per GPIO steuern (Beispiel)
-- Verwendung: fenrir:gpio_write(pin, true/false), fenrir:gpio_read(pin)
local pin = 1
fenrir:gpio_write(pin, true)
local on = fenrir:gpio_read(pin)
return on and "LED on" or "LED off"
