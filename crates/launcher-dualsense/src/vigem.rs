use crate::emulation::{EmulationMode, GyroSettings};
use crate::protocol::DualSenseInputState;
#[allow(unused_imports)]
use tracing::{debug, info, warn};

// ViGEmBus IOCTL Definitions
// CTL_CODE(FILE_DEVICE_BUS_EXTENDER(0x2A), Function, METHOD_BUFFERED(0), FILE_WRITE_DATA(2))
pub const IOCTL_VIGEM_PLUGIN_TARGET: u32 = 0x2AA004;
pub const IOCTL_VIGEM_UNPLUG_TARGET: u32 = 0x2AA008;
pub const IOCTL_XUSB_SUBMIT_REPORT: u32 = 0x2AA808;
pub const IOCTL_DS4_SUBMIT_REPORT: u32 = 0x2AA80C;

pub const VIGEM_TARGET_XBOX360_WIRED: u32 = 0;
pub const VIGEM_TARGET_DUALSHOCK4_WIRED: u32 = 2;
pub const DEFAULT_VIGEM_SERIAL: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViGEmPluginTarget {
    pub size: u32,
    pub serial_no: u32,
    pub target_type: u32,
    pub vendor_id: u16,
    pub product_id: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViGEmUnplugTarget {
    pub size: u32,
    pub serial_no: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct XusbReport {
    pub w_buttons: u16,
    pub b_left_trigger: u8,
    pub b_right_trigger: u8,
    pub s_thumb_lx: i16,
    pub s_thumb_ly: i16,
    pub s_thumb_rx: i16,
    pub s_thumb_ry: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct XusbSubmitReport {
    pub size: u32,
    pub serial_no: u32,
    pub report: XusbReport,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ds4Report {
    pub b_thumb_lx: u8,
    pub b_thumb_ly: u8,
    pub b_thumb_rx: u8,
    pub b_thumb_ry: u8,
    pub w_buttons: u16,
    pub b_special: u8,
    pub b_trigger_l: u8,
    pub b_trigger_r: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ds4SubmitReport {
    pub size: u32,
    pub serial_no: u32,
    pub report: Ds4Report,
}

pub struct VirtualGamepadBus {
    #[cfg(target_os = "windows")]
    handle: isize,
    active_mode: EmulationMode,
    is_plugged: bool,
}

impl Default for VirtualGamepadBus {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualGamepadBus {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            handle: -1,
            active_mode: EmulationMode::Native,
            is_plugged: false,
        }
    }

    /// Checks if ViGEmBus kernel driver is installed on this system
    pub fn is_vigem_driver_installed() -> bool {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let h = open_vigem_handle();
                if h > 0 {
                    close_handle(h);
                    true
                } else {
                    false
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Sets controller emulation mode (Native, Xbox 360, or DualShock 4)
    pub fn set_mode(&mut self, mode: EmulationMode) -> bool {
        if self.active_mode == mode && self.is_plugged {
            return true;
        }

        self.unplug();
        self.active_mode = mode;

        if mode == EmulationMode::Native {
            return true;
        }

        #[cfg(target_os = "windows")]
        {
            unsafe {
                if self.handle <= 0 {
                    let h = open_vigem_handle();
                    if h <= 0 {
                        warn!(
                            "ViGEmBus driver not found. Virtual {:?} emulation cannot attach.",
                            mode
                        );
                        return false;
                    }
                    self.handle = h;
                }

                let target_type = match mode {
                    EmulationMode::Xbox360 => VIGEM_TARGET_XBOX360_WIRED,
                    EmulationMode::DualShock4 => VIGEM_TARGET_DUALSHOCK4_WIRED,
                    EmulationMode::Native => return true,
                };

                let mut plugin = ViGEmPluginTarget {
                    size: std::mem::size_of::<ViGEmPluginTarget>() as u32,
                    serial_no: DEFAULT_VIGEM_SERIAL,
                    target_type,
                    vendor_id: 0,
                    product_id: 0,
                };

                let mut bytes_ret = 0u32;
                let ok = device_io_control(
                    self.handle,
                    IOCTL_VIGEM_PLUGIN_TARGET,
                    &mut plugin as *mut _ as *mut _,
                    std::mem::size_of::<ViGEmPluginTarget>() as u32,
                    std::ptr::null_mut(),
                    0,
                    &mut bytes_ret,
                );

                if ok != 0 {
                    self.is_plugged = true;
                    info!(
                        "ViGEmBus: Successfully plugged virtual {:?} controller (serial={})",
                        mode, DEFAULT_VIGEM_SERIAL
                    );
                    true
                } else {
                    warn!("ViGEmBus: Failed to plug virtual {:?} controller", mode);
                    false
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            debug!(
                "ViGEmBus is only supported on Windows. Emulation state recorded: {:?}",
                mode
            );
            true
        }
    }

    /// Dispatches physical input report into the emulated virtual gamepad
    pub fn submit_input(&mut self, _input: &DualSenseInputState, _gyro: &GyroSettings) {
        #[cfg(target_os = "windows")]
        {
            if !self.is_plugged || self.active_mode == EmulationMode::Native || self.handle <= 0 {
                return;
            }

            unsafe {
                match self.active_mode {
                    EmulationMode::Xbox360 => {
                        let report = map_input_to_xusb(_input, _gyro);
                        let mut submit = XusbSubmitReport {
                            size: std::mem::size_of::<XusbSubmitReport>() as u32,
                            serial_no: DEFAULT_VIGEM_SERIAL,
                            report,
                        };
                        let mut bytes_ret = 0u32;
                        let _ = device_io_control(
                            self.handle,
                            IOCTL_XUSB_SUBMIT_REPORT,
                            &mut submit as *mut _ as *mut _,
                            std::mem::size_of::<XusbSubmitReport>() as u32,
                            std::ptr::null_mut(),
                            0,
                            &mut bytes_ret,
                        );
                    }
                    EmulationMode::DualShock4 => {
                        let report = map_input_to_ds4(_input);
                        let mut submit = Ds4SubmitReport {
                            size: std::mem::size_of::<Ds4SubmitReport>() as u32,
                            serial_no: DEFAULT_VIGEM_SERIAL,
                            report,
                        };
                        let mut bytes_ret = 0u32;
                        let _ = device_io_control(
                            self.handle,
                            IOCTL_DS4_SUBMIT_REPORT,
                            &mut submit as *mut _ as *mut _,
                            std::mem::size_of::<Ds4SubmitReport>() as u32,
                            std::ptr::null_mut(),
                            0,
                            &mut bytes_ret,
                        );
                    }
                    EmulationMode::Native => {}
                }
            }
        }
    }

    /// Unplugs the virtual controller and closes the driver handle
    pub fn unplug(&mut self) {
        if self.is_plugged {
            #[cfg(target_os = "windows")]
            unsafe {
                if self.handle > 0 {
                    let mut unplug = ViGEmUnplugTarget {
                        size: std::mem::size_of::<ViGEmUnplugTarget>() as u32,
                        serial_no: DEFAULT_VIGEM_SERIAL,
                    };
                    let mut bytes_ret = 0u32;
                    let _ = device_io_control(
                        self.handle,
                        IOCTL_VIGEM_UNPLUG_TARGET,
                        &mut unplug as *mut _ as *mut _,
                        std::mem::size_of::<ViGEmUnplugTarget>() as u32,
                        std::ptr::null_mut(),
                        0,
                        &mut bytes_ret,
                    );
                    info!(
                        "ViGEmBus: Virtual controller unplugged (serial={})",
                        DEFAULT_VIGEM_SERIAL
                    );
                }
            }
            self.is_plugged = false;
        }

        #[cfg(target_os = "windows")]
        unsafe {
            if self.handle > 0 {
                close_handle(self.handle);
                self.handle = -1;
            }
        }
    }
}

impl Drop for VirtualGamepadBus {
    fn drop(&mut self) {
        self.unplug();
    }
}

/// Translates DualSense physical input to XInput Xbox 360 structure
pub fn map_input_to_xusb(input: &DualSenseInputState, gyro: &GyroSettings) -> XusbReport {
    let mut w_buttons: u16 = 0;

    // D-Pad mapping
    if input.dpad_up {
        w_buttons |= 0x0001; // XINPUT_GAMEPAD_DPAD_UP
    }
    if input.dpad_down {
        w_buttons |= 0x0002; // XINPUT_GAMEPAD_DPAD_DOWN
    }
    if input.dpad_left {
        w_buttons |= 0x0004; // XINPUT_GAMEPAD_DPAD_LEFT
    }
    if input.dpad_right {
        w_buttons |= 0x0008; // XINPUT_GAMEPAD_DPAD_RIGHT
    }

    // Menu / Navigation
    if input.options {
        w_buttons |= 0x0010; // XINPUT_GAMEPAD_START
    }
    if input.create {
        w_buttons |= 0x0020; // XINPUT_GAMEPAD_BACK
    }
    if input.l3 {
        w_buttons |= 0x0040; // XINPUT_GAMEPAD_LEFT_THUMB
    }
    if input.r3 {
        w_buttons |= 0x0080; // XINPUT_GAMEPAD_RIGHT_THUMB
    }
    if input.l1 {
        w_buttons |= 0x0100; // XINPUT_GAMEPAD_LEFT_SHOULDER
    }
    if input.r1 {
        w_buttons |= 0x0200; // XINPUT_GAMEPAD_RIGHT_SHOULDER
    }

    // Face buttons: Cross -> A, Circle -> B, Square -> X, Triangle -> Y
    if input.cross {
        w_buttons |= 0x1000; // XINPUT_GAMEPAD_A
    }
    if input.circle {
        w_buttons |= 0x2000; // XINPUT_GAMEPAD_B
    }
    if input.square {
        w_buttons |= 0x4000; // XINPUT_GAMEPAD_X
    }
    if input.triangle {
        w_buttons |= 0x8000; // XINPUT_GAMEPAD_Y
    }

    // Joysticks: DualSense 0..255 (128 center) -> XInput -32768..32767
    let lx = ((input.left_stick_x as i32 - 128) * 256).clamp(-32768, 32767) as i16;
    let ly = (-(input.left_stick_y as i32 - 128) * 256).clamp(-32768, 32767) as i16;

    let (gyro_dx, gyro_dy) = gyro.map_gyro_to_stick_delta(input.motion.gyro_z, input.motion.gyro_y);
    let rx =
        ((input.right_stick_x as i32 - 128) * 256 + (gyro_dx as i32)).clamp(-32768, 32767) as i16;
    let ry =
        (-(input.right_stick_y as i32 - 128) * 256 + (gyro_dy as i32)).clamp(-32768, 32767) as i16;

    XusbReport {
        w_buttons,
        b_left_trigger: input.l2_trigger,
        b_right_trigger: input.r2_trigger,
        s_thumb_lx: lx,
        s_thumb_ly: ly,
        s_thumb_rx: rx,
        s_thumb_ry: ry,
    }
}

/// Translates DualSense physical input to DualShock 4 structure
pub fn map_input_to_ds4(input: &DualSenseInputState) -> Ds4Report {
    let mut w_buttons: u16 = 0;

    // D-Pad HAT encoding
    let dpad = match (
        input.dpad_up,
        input.dpad_right,
        input.dpad_down,
        input.dpad_left,
    ) {
        (true, false, false, false) => 0,
        (true, true, false, false) => 1,
        (false, true, false, false) => 2,
        (false, true, true, false) => 3,
        (false, false, true, false) => 4,
        (false, false, true, true) => 5,
        (false, false, false, true) => 6,
        (true, false, false, true) => 7,
        _ => 8, // Center / Release
    };
    w_buttons |= dpad & 0x0F;

    if input.square {
        w_buttons |= 0x0010;
    }
    if input.cross {
        w_buttons |= 0x0020;
    }
    if input.circle {
        w_buttons |= 0x0040;
    }
    if input.triangle {
        w_buttons |= 0x0080;
    }
    if input.l1 {
        w_buttons |= 0x0100;
    }
    if input.r1 {
        w_buttons |= 0x0200;
    }
    if input.l2_click {
        w_buttons |= 0x0400;
    }
    if input.r2_click {
        w_buttons |= 0x0800;
    }
    if input.create {
        w_buttons |= 0x1000; // Share
    }
    if input.options {
        w_buttons |= 0x2000; // Options
    }
    if input.l3 {
        w_buttons |= 0x4000;
    }
    if input.r3 {
        w_buttons |= 0x8000;
    }

    let mut b_special: u8 = 0;
    if input.ps_button {
        b_special |= 0x01;
    }
    if input.touchpad_click {
        b_special |= 0x02;
    }

    Ds4Report {
        b_thumb_lx: input.left_stick_x,
        b_thumb_ly: input.left_stick_y,
        b_thumb_rx: input.right_stick_x,
        b_thumb_ry: input.right_stick_y,
        w_buttons,
        b_special,
        b_trigger_l: input.l2_trigger,
        b_trigger_r: input.r2_trigger,
    }
}

#[cfg(target_os = "windows")]
#[repr(C)]
#[derive(Copy, Clone)]
struct WinGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

// GUID_DEVINTERFACE_VIGEMBUS: {96E42B22-F5E9-42F8-B043-ED0F932F014F}
#[cfg(target_os = "windows")]
const VIGEM_GUID: WinGuid = WinGuid {
    data1: 0x96E42B22,
    data2: 0xF5E9,
    data3: 0x42F8,
    data4: [0xB0, 0x43, 0xED, 0x0F, 0x93, 0x2F, 0x01, 0x4F],
};

#[cfg(target_os = "windows")]
#[repr(C)]
struct SpDeviceInterfaceData {
    cb_size: u32,
    interface_class_guid: WinGuid,
    flags: u32,
    reserved: usize,
}

#[cfg(target_os = "windows")]
#[link(name = "setupapi")]
extern "system" {
    fn SetupDiGetClassDevsW(
        class_guid: *const WinGuid,
        enumerator: *const u16,
        hwnd_parent: isize,
        flags: u32,
    ) -> isize;

    fn SetupDiEnumDeviceInterfaces(
        device_info_set: isize,
        device_info_data: *const std::ffi::c_void,
        interface_class_guid: *const WinGuid,
        member_index: u32,
        device_interface_data: *mut SpDeviceInterfaceData,
    ) -> i32;

    fn SetupDiGetDeviceInterfaceDetailW(
        device_info_set: isize,
        device_info_data: *mut SpDeviceInterfaceData,
        device_interface_detail_data: *mut u8,
        device_interface_detail_data_size: u32,
        required_size: *mut u32,
        device_info_data_out: *mut std::ffi::c_void,
    ) -> i32;

    fn SetupDiDestroyDeviceInfoList(device_info_set: isize) -> i32;
}

#[cfg(target_os = "windows")]
extern "system" {
    fn GetLastError() -> u32;
}

#[cfg(target_os = "windows")]
unsafe fn find_vigem_device_paths() -> Vec<Vec<u16>> {
    const DIGCF_PRESENT: u32 = 0x00000002;
    const DIGCF_DEVICEINTERFACE: u32 = 0x00000010;
    const INVALID_HANDLE_VALUE: isize = -1;

    let mut found_paths = Vec::new();

    let h_dev_info = SetupDiGetClassDevsW(
        &VIGEM_GUID,
        std::ptr::null(),
        0,
        DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
    );

    if h_dev_info == INVALID_HANDLE_VALUE || h_dev_info == 0 {
        let err = GetLastError();
        debug!("SetupDiGetClassDevsW returned invalid handle (err={})", err);
        return found_paths;
    }

    let mut if_data: SpDeviceInterfaceData = std::mem::zeroed();
    if_data.cb_size = std::mem::size_of::<SpDeviceInterfaceData>() as u32;

    for index in 0..16 {
        if SetupDiEnumDeviceInterfaces(
            h_dev_info,
            std::ptr::null(),
            &VIGEM_GUID,
            index,
            &mut if_data,
        ) == 0
        {
            break;
        }

        let mut required_size = 0u32;
        let _ = SetupDiGetDeviceInterfaceDetailW(
            h_dev_info,
            &mut if_data,
            std::ptr::null_mut(),
            0,
            &mut required_size,
            std::ptr::null_mut(),
        );

        if required_size == 0 {
            continue;
        }

        let mut buf = vec![0u8; required_size as usize];

        // Probe expected cbSize header values for x86_64 and x86
        let cb_candidates: [u32; 5] = if cfg!(target_pointer_width = "64") {
            [8, 4, 6, 5, 12]
        } else {
            [6, 5, 4, 8, 12]
        };

        let mut success = false;
        for test_cb in cb_candidates {
            buf[0..4].copy_from_slice(&test_cb.to_ne_bytes());
            if SetupDiGetDeviceInterfaceDetailW(
                h_dev_info,
                &mut if_data,
                buf.as_mut_ptr(),
                required_size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) != 0
            {
                success = true;
                break;
            }
        }

        if success {
            // Locate string start: check offset 4 or offset 8 for '\\' (0x005C)
            let offset = if buf.len() > 6 && buf[4] == b'\\' && buf[5] == 0 {
                4
            } else if buf.len() > 10 && buf[8] == b'\\' && buf[9] == 0 {
                8
            } else {
                4
            };

            let u16_slice = std::slice::from_raw_parts(
                buf.as_ptr().add(offset) as *const u16,
                (required_size as usize - offset) / 2,
            );

            if let Some(pos) = u16_slice.iter().position(|&c| c == 0) {
                let mut path = u16_slice[..pos].to_vec();
                path.push(0);
                let path_str = String::from_utf16_lossy(&path[..pos]);
                info!("ViGEmBus: Discovered device interface path: {}", path_str);
                found_paths.push(path);
            }
        }
    }

    SetupDiDestroyDeviceInfoList(h_dev_info);
    found_paths
}

#[cfg(target_os = "windows")]
unsafe fn open_vigem_handle() -> isize {
    const GENERIC_READ: u32 = 0x80000000;
    const GENERIC_WRITE: u32 = 0x40000000;
    const FILE_SHARE_READ: u32 = 0x00000001;
    const FILE_SHARE_WRITE: u32 = 0x00000002;
    const OPEN_EXISTING: u32 = 3;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x00000080;
    const INVALID_HANDLE_VALUE: isize = -1;

    // 1. Try SetupAPI device interface paths (standard ViGEmBus driver registration)
    let paths = find_vigem_device_paths();
    for p in paths {
        let h = CreateFileW(
            p.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0,
        );
        if h != INVALID_HANDLE_VALUE && h != 0 {
            info!("ViGEmBus: Successfully opened communication handle");
            return h;
        } else {
            let err = GetLastError();
            warn!(
                "ViGEmBus: CreateFileW on device path failed with error {}",
                err
            );
        }
    }

    // 2. Fallback to symbolic names
    let fallbacks = [r"\\.\ViGEmBus", r"\\.\NefariusViGEmBus", r"\\.\ViGEmBus\0"];
    for fb in fallbacks {
        let wide: Vec<u16> = fb.encode_utf16().chain(std::iter::once(0)).collect();
        let h = CreateFileW(
            wide.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            0,
        );
        if h != INVALID_HANDLE_VALUE && h != 0 {
            info!("ViGEmBus: Opened handle via symbolic fallback: {}", fb);
            return h;
        }
    }

    INVALID_HANDLE_VALUE
}

#[cfg(target_os = "windows")]
unsafe fn close_handle(h: isize) {
    if h > 0 {
        CloseHandle(h);
    }
}

#[cfg(target_os = "windows")]
extern "system" {
    fn CreateFileW(
        lpFileName: *const u16,
        dwDesiredAccess: u32,
        dwShareMode: u32,
        lpSecurityAttributes: *const std::ffi::c_void,
        dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32,
        hTemplateFile: isize,
    ) -> isize;

    fn CloseHandle(hObject: isize) -> i32;

    fn DeviceIoControl(
        hDevice: isize,
        dwIoControlCode: u32,
        lpInBuffer: *const std::ffi::c_void,
        nInBufferSize: u32,
        lpOutBuffer: *mut std::ffi::c_void,
        nOutBufferSize: u32,
        lpBytesReturned: *mut u32,
        lpOverlapped: *mut std::ffi::c_void,
    ) -> i32;
}

#[cfg(target_os = "windows")]
unsafe fn device_io_control(
    h_device: isize,
    dw_io_control_code: u32,
    lp_in_buffer: *const std::ffi::c_void,
    n_in_buffer_size: u32,
    lp_out_buffer: *mut std::ffi::c_void,
    n_out_buffer_size: u32,
    lp_bytes_returned: *mut u32,
) -> i32 {
    DeviceIoControl(
        h_device,
        dw_io_control_code,
        lp_in_buffer,
        n_in_buffer_size,
        lp_out_buffer,
        n_out_buffer_size,
        lp_bytes_returned,
        std::ptr::null_mut(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xusb_mapping() {
        let input = DualSenseInputState {
            cross: true,
            circle: true,
            l2_trigger: 200,
            r2_trigger: 255,
            left_stick_x: 255, // full right
            left_stick_y: 0,   // full up
            ..Default::default()
        };

        let gyro = GyroSettings::default();
        let xusb = map_input_to_xusb(&input, &gyro);

        assert_eq!(xusb.w_buttons & 0x1000, 0x1000); // A
        assert_eq!(xusb.w_buttons & 0x2000, 0x2000); // B
        assert_eq!(xusb.b_left_trigger, 200);
        assert_eq!(xusb.b_right_trigger, 255);
        assert!(xusb.s_thumb_lx > 30000);
        assert!(xusb.s_thumb_ly > 30000); // Inverted Y is positive
    }

    #[test]
    fn test_ds4_mapping() {
        let input = DualSenseInputState {
            square: true,
            triangle: true,
            ps_button: true,
            dpad_up: true,
            ..Default::default()
        };

        let ds4 = map_input_to_ds4(&input);
        assert_eq!(ds4.w_buttons & 0x0F, 0); // Dpad Up is 0
        assert_eq!(ds4.w_buttons & 0x0010, 0x0010); // Square
        assert_eq!(ds4.w_buttons & 0x0080, 0x0080); // Triangle
        assert_eq!(ds4.b_special & 0x01, 0x01); // PS button
    }
}
