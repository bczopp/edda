use crate::actions::{ActionExecutor, ActionContext, ActionError};
use crate::ui_automation::platform::{OperatingSystemDetector, OperatingSystem};
use serde_json::Value;
use async_trait::async_trait;

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
    use std::mem::size_of;

    fn position_from_element(element: &Element) -> Result<(i32, i32), ActionError> {
        let x = element.value.get("x").and_then(Value::as_i64).ok_or_else(|| {
            ActionError::InvalidAction("Element by_position missing 'x'".to_string())
        })? as i32;
        let y = element.value.get("y").and_then(Value::as_i64).ok_or_else(|| {
            ActionError::InvalidAction("Element by_position missing 'y'".to_string())
        })? as i32;
        Ok((x, y))
    }

    pub async fn click_element(element: &Element) -> Result<(), ActionError> {
        let (x, y) = match element.element_type.as_str() {
            "by_position" => position_from_element(element)?,
            _ => {
                return Err(ActionError::ExecutionFailed(
                    "Windows UI Automation: by_name/by_id not yet implemented, use by_position".to_string(),
                ));
            }
        };
        unsafe {
            SetCursorPos(x, y).map_err(|e| {
                ActionError::ExecutionFailed(format!("SetCursorPos failed: {}", e))
            })?;
            // Send left button down then up at current (x,y)
            let down = INPUT {
                r#type: INPUT_TYPE::INPUT_MOUSE,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSE_EVENT_FLAGS::MOUSEEVENTF_LEFTDOWN,
                        dwExtraInfo: 0,
                        time: 0,
                    },
                },
            };
            let up = INPUT {
                r#type: INPUT_TYPE::INPUT_MOUSE,
                Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSE_EVENT_FLAGS::MOUSEEVENTF_LEFTUP,
                        dwExtraInfo: 0,
                        time: 0,
                    },
                },
            };
            let inputs = [down, up];
            let n = SendInput(&inputs, size_of::<INPUT>() as i32);
            if n != 2 {
                return Err(ActionError::ExecutionFailed(
                    "SendInput (mouse click) failed".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub async fn type_text(_element: &Element, text: &str) -> Result<(), ActionError> {
        unsafe {
            for ch in text.chars() {
                // BMP only; surrogates (char > U+FFFF) would need encode_utf16
                let scan = match ch as u32 {
                    n if n <= 0xFFFF => n as u16,
                    _ => continue,
                };
                let inputs = [
                    INPUT {
                        r#type: INPUT_TYPE::INPUT_KEYBOARD,
                        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: 0,
                                wScan: scan,
                                dwFlags: KEYBD_EVENT_FLAGS::KEYEVENTF_UNICODE,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                    INPUT {
                        r#type: INPUT_TYPE::INPUT_KEYBOARD,
                        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                            ki: KEYBDINPUT {
                                wVk: 0,
                                wScan: scan,
                                dwFlags: KEYBD_EVENT_FLAGS::KEYEVENTF_UNICODE
                                    | KEYBD_EVENT_FLAGS::KEYEVENTF_KEYUP,
                                time: 0,
                                dwExtraInfo: 0,
                            },
                        },
                    },
                ];
                let n = SendInput(&inputs, size_of::<INPUT>() as i32);
                if n != 2 {
                    return Err(ActionError::ExecutionFailed(
                        "SendInput (keyboard) failed".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub async fn move_cursor(x: i32, y: i32) -> Result<(), ActionError> {
        unsafe {
            SetCursorPos(x, y).map_err(|e| {
                ActionError::ExecutionFailed(format!("SetCursorPos failed: {}", e))
            })?;
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod macos_impl {
    use super::*;
    
    pub async fn click_element(_element: &Element) -> Result<(), ActionError> {
        // macOS Accessibility API implementation
        // Would use cocoa crate for actual implementation
        Err(ActionError::ExecutionFailed("macOS UI Automation not yet fully implemented".to_string()))
    }
    
    pub async fn type_text(_element: &Element, _text: &str) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("macOS UI Automation not yet fully implemented".to_string()))
    }
    
    pub async fn move_cursor(_x: i32, _y: i32) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("macOS UI Automation not yet fully implemented".to_string()))
    }
}

#[cfg(target_os = "linux")]
mod linux_impl {
    use super::*;
    use atspi::AccessibilityConnection;
    use atspi::proxy::action::ActionProxy;
    use atspi::proxy::component::ComponentProxy;
    use atspi::CoordType;
    use atspi::ObjectRef;
    use std::sync::Arc;
    use tokio::sync::OnceCell;
    use zbus::Proxy;

    static ATSPI_CONNECTION: OnceCell<Result<Arc<AccessibilityConnection>, String>> = OnceCell::const_new();

    async fn get_connection() -> Result<Arc<AccessibilityConnection>, ActionError> {
        ATSPI_CONNECTION
            .get_or_init(|| async {
                match AccessibilityConnection::new().await {
                    Ok(conn) => Ok(Arc::new(conn)),
                    Err(e) => Err(format!("Linux UI Automation AT-SPI connection failed: {e}")),
                }
            })
            .await
            .clone()
            .map_err(ActionError::ExecutionFailed)
    }

    fn position_from_element(element: &Element) -> Result<(i32, i32), ActionError> {
        let x = element.value.get("x").and_then(Value::as_i64).ok_or_else(|| {
            ActionError::InvalidAction("Element by_position missing 'x'".to_string())
        })? as i32;
        let y = element.value.get("y").and_then(Value::as_i64).ok_or_else(|| {
            ActionError::InvalidAction("Element by_position missing 'y'".to_string())
        })? as i32;
        Ok((x, y))
    }

    pub async fn click_element(element: &Element) -> Result<(), ActionError> {
        let (x, y) = match element.element_type.as_str() {
            "by_position" => position_from_element(element)?,
            _ => {
                return Err(ActionError::ExecutionFailed(
                    "Linux UI Automation: by_name/by_id not yet implemented, use by_position".to_string(),
                ));
            }
        };
        let conn = get_connection().await?;
        let root = conn
            .root_accessible_on_registry()
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI root: {e}")))?;
        let path = root.inner().path().clone();
        let destination = root
            .inner()
            .destination()
            .cloned()
            .ok_or_else(|| ActionError::ExecutionFailed("AT-SPI root has no destination".to_string()))?;
        let component = ComponentProxy::builder(conn.connection())
            .path(path)
            .destination(destination)
            .build()
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI Component proxy: {e}")))?;
        let obj_ref = component
            .get_accessible_at_point(x, y, CoordType::Screen)
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI get_accessible_at_point: {e}")))?;
        if matches!(obj_ref.as_ref(), ObjectRef::Null) {
            return Err(ActionError::ExecutionFailed(format!(
                "No accessible element at position ({}, {})",
                x, y
            )));
        }
        let accessible = conn
            .object_as_accessible(&obj_ref)
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI object_as_accessible: {e}")))?;
        let path = accessible.inner().path().clone();
        let destination = accessible
            .inner()
            .destination()
            .cloned()
            .ok_or_else(|| ActionError::ExecutionFailed("AT-SPI accessible has no destination".to_string()))?;
        let action_proxy = ActionProxy::builder(conn.connection())
            .path(path)
            .destination(destination)
            .build()
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI Action proxy: {e}")))?;
        let n = action_proxy
            .nactions()
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI nactions: {e}")))?;
        if n <= 0 {
            return Err(ActionError::ExecutionFailed(
                "Element at position does not support actions".to_string(),
            ));
        }
        let ok = action_proxy
            .do_action(0)
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("AT-SPI do_action: {e}")))?;
        if !ok {
            return Err(ActionError::ExecutionFailed("AT-SPI do_action returned false".to_string()));
        }
        Ok(())
    }
    
    pub async fn type_text(_element: &Element, _text: &str) -> Result<(), ActionError> {
        let _conn = get_connection().await?;
        Err(ActionError::ExecutionFailed(
            "Linux UI Automation (AT-SPI type) not yet fully implemented".to_string(),
        ))
    }
    
    pub async fn move_cursor(_x: i32, _y: i32) -> Result<(), ActionError> {
        let _conn = get_connection().await?;
        Err(ActionError::ExecutionFailed(
            "Linux UI Automation (AT-SPI move) not yet fully implemented".to_string(),
        ))
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
mod fallback_impl {
    use super::*;
    
    pub async fn click_element(_element: &Element) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("UI Automation not supported on this operating system".to_string()))
    }
    
    pub async fn type_text(_element: &Element, _text: &str) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("UI Automation not supported on this operating system".to_string()))
    }
    
    pub async fn move_cursor(_x: i32, _y: i32) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("UI Automation not supported on this operating system".to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub element_type: String, // "by_name", "by_position", "by_id"
    pub value: Value,
}

/// UI Automation action handler
pub struct UIAutomationHandler {
    os_detector: OperatingSystemDetector,
}

impl UIAutomationHandler {
    pub fn new() -> Self {
        Self {
            os_detector: OperatingSystemDetector::new(),
        }
    }

    fn parse_params(&self, action_data: &[u8]) -> Result<UIActionParams, ActionError> {
        let value: Value = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Failed to parse action data: {}", e)))?;
        
        let action = value["action"]
            .as_str()
            .ok_or_else(|| ActionError::InvalidAction("Missing 'action' field".to_string()))?
            .to_string();
        
        let element = if value["element"].is_object() {
            Some(Element {
                element_type: value["element"]["type"]
                    .as_str()
                    .unwrap_or("by_name")
                    .to_string(),
                value: value["element"].clone(),
            })
        } else {
            None
        };
        
        Ok(UIActionParams {
            action,
            element,
            text: value["text"].as_str().map(|s| s.to_string()),
            x: value["x"].as_i64().map(|v| v as i32),
            y: value["y"].as_i64().map(|v| v as i32),
            confirmed: value["confirmed"].as_bool().unwrap_or(false),
        })
    }

    async fn execute_os_action(&self, params: &UIActionParams) -> Result<(), ActionError> {
        let os = self.os_detector.detect();

        // Kritische UI-Aktionen wie "click" dürfen nur mit expliziter
        // User-Bestätigung ausgeführt werden. Die Bestätigung muss vom
        // aufrufenden System (z. B. Odin/Plattform) gesetzt werden.
        if params.action == "click" && !params.confirmed {
            return Err(ActionError::InvalidAction(
                "User confirmation required for click action".to_string(),
            ));
        }

        match os {
            #[cfg(target_os = "windows")]
            OperatingSystem::Windows => {
                match params.action.as_str() {
                    "click" => {
                        if let Some(ref element) = params.element {
                            windows_impl::click_element(element).await
                        } else {
                            Err(ActionError::InvalidAction("Missing element for click action".to_string()))
                        }
                    }
                    "type" => {
                        if let Some(ref element) = params.element {
                            if let Some(ref text) = params.text {
                                windows_impl::type_text(element, text).await
                            } else {
                                Err(ActionError::InvalidAction("Missing text for type action".to_string()))
                            }
                        } else {
                            Err(ActionError::InvalidAction("Missing element for type action".to_string()))
                        }
                    }
                    "move" => {
                        if let (Some(x), Some(y)) = (params.x, params.y) {
                            windows_impl::move_cursor(x, y).await
                        } else {
                            Err(ActionError::InvalidAction("Missing coordinates for move action".to_string()))
                        }
                    }
                    _ => Err(ActionError::InvalidAction(format!("Unknown UI action: {}", params.action))),
                }
            }
            #[cfg(target_os = "macos")]
            OperatingSystem::MacOS => {
                match params.action.as_str() {
                    "click" => {
                        if let Some(ref element) = params.element {
                            macos_impl::click_element(element).await
                        } else {
                            Err(ActionError::InvalidAction("Missing element for click action".to_string()))
                        }
                    }
                    "type" => {
                        if let Some(ref element) = params.element {
                            if let Some(ref text) = params.text {
                                macos_impl::type_text(element, text).await
                            } else {
                                Err(ActionError::InvalidAction("Missing text for type action".to_string()))
                            }
                        } else {
                            Err(ActionError::InvalidAction("Missing element for type action".to_string()))
                        }
                    }
                    "move" => {
                        if let (Some(x), Some(y)) = (params.x, params.y) {
                            macos_impl::move_cursor(x, y).await
                        } else {
                            Err(ActionError::InvalidAction("Missing coordinates for move action".to_string()))
                        }
                    }
                    _ => Err(ActionError::InvalidAction(format!("Unknown UI action: {}", params.action))),
                }
            }
            #[cfg(target_os = "linux")]
            OperatingSystem::Linux => {
                match params.action.as_str() {
                    "click" => {
                        if let Some(ref element) = params.element {
                            linux_impl::click_element(element).await
                        } else {
                            Err(ActionError::InvalidAction("Missing element for click action".to_string()))
                        }
                    }
                    "type" => {
                        if let Some(ref element) = params.element {
                            if let Some(ref text) = params.text {
                                linux_impl::type_text(element, text).await
                            } else {
                                Err(ActionError::InvalidAction("Missing text for type action".to_string()))
                            }
                        } else {
                            Err(ActionError::InvalidAction("Missing element for type action".to_string()))
                        }
                    }
                    "move" => {
                        if let (Some(x), Some(y)) = (params.x, params.y) {
                            linux_impl::move_cursor(x, y).await
                        } else {
                            Err(ActionError::InvalidAction("Missing coordinates for move action".to_string()))
                        }
                    }
                    _ => Err(ActionError::InvalidAction(format!("Unknown UI action: {}", params.action))),
                }
            }
            _ => {
                #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
                {
                    match params.action.as_str() {
                        "click" => {
                            if let Some(ref element) = params.element {
                                fallback_impl::click_element(element).await
                            } else {
                                Err(ActionError::InvalidAction("Missing element for click action".to_string()))
                            }
                        }
                        "type" => {
                            if let Some(ref element) = params.element {
                                if let Some(ref text) = params.text {
                                    fallback_impl::type_text(element, text).await
                                } else {
                                    Err(ActionError::InvalidAction("Missing text for type action".to_string()))
                                }
                            } else {
                                Err(ActionError::InvalidAction("Missing element for type action".to_string()))
                            }
                        }
                        "move" => {
                            if let (Some(x), Some(y)) = (params.x, params.y) {
                                fallback_impl::move_cursor(x, y).await
                            } else {
                                Err(ActionError::InvalidAction("Missing coordinates for move action".to_string()))
                            }
                        }
                        _ => Err(ActionError::InvalidAction(format!("Unknown UI action: {}", params.action))),
                    }
                }
                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                {
                    Err(ActionError::ExecutionFailed("Unsupported operating system".to_string()))
                }
            }
        }
    }
}

#[derive(Debug)]
struct UIActionParams {
    action: String,
    element: Option<Element>,
    text: Option<String>,
    x: Option<i32>,
    y: Option<i32>,
    confirmed: bool,
}

#[async_trait]
impl ActionExecutor for UIAutomationHandler {
    fn action_type(&self) -> &str {
        "UI_AUTOMATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params = self.parse_params(action_data)?;
        
        // Execute operating-system-specific action
        self.execute_os_action(&params).await?;
        
        // Return success result
        let result = serde_json::json!({
            "success": true,
            "action": params.action,
            "operating_system": format!("{:?}", self.os_detector.detect()),
        });
        
        serde_json::to_vec(&result)
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to serialize result: {}", e)))
    }
}

impl Default for UIAutomationHandler {
    fn default() -> Self {
        Self::new()
    }
}
