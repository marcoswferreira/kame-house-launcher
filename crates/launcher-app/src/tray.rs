use crate::LauncherService;
use launcher_core::models::Language;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tracing::info;

/// Action chosen by the user when closing the frontend window
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseDecision {
    MinimizeToTray,
    QuitApplication,
    ReopenWindow,
}

/// Shared state for window lifecycle, debug console visibility, and close confirmation
pub struct WindowLifecycleState {
    pub ui_url: RwLock<String>,
    pub debug_console_visible: AtomicBool,
    pub last_page_activity_ms: AtomicU64,
    pub last_explicit_action_ms: AtomicU64,
    pub shutdown_requested: AtomicBool,
}

impl Default for WindowLifecycleState {
    fn default() -> Self {
        Self::new("http://127.0.0.1:48080".to_string())
    }
}

impl WindowLifecycleState {
    pub fn new(ui_url: String) -> Self {
        Self {
            ui_url: RwLock::new(ui_url),
            debug_console_visible: AtomicBool::new(false),
            last_page_activity_ms: AtomicU64::new(current_time_ms()),
            last_explicit_action_ms: AtomicU64::new(0),
            shutdown_requested: AtomicBool::new(false),
        }
    }

    pub fn set_ui_url(&self, url: String) {
        *self.ui_url.write() = url;
    }

    pub fn get_ui_url(&self) -> String {
        self.ui_url.read().clone()
    }

    pub fn record_page_activity(&self) {
        self.last_page_activity_ms
            .store(current_time_ms(), Ordering::SeqCst);
    }

    pub fn record_explicit_action(&self) {
        self.last_explicit_action_ms
            .store(current_time_ms(), Ordering::SeqCst);
    }

    pub fn was_recently_active_or_explicit(&self, since_ms: u64) -> bool {
        let last_act = self.last_page_activity_ms.load(Ordering::SeqCst);
        let last_exp = self.last_explicit_action_ms.load(Ordering::SeqCst);
        last_act > since_ms || last_exp.saturating_add(3000) > since_ms
    }

    pub fn is_debug_console_visible(&self) -> bool {
        self.debug_console_visible.load(Ordering::SeqCst)
    }
}

pub fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Localized labels for System Tray menu and Close Confirmation Dialog
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayLabels {
    pub tooltip: &'static str,
    pub open_ui: &'static str,
    pub show_console: &'static str,
    pub hide_console: &'static str,
    pub quit_app: &'static str,
    pub minimized_title: &'static str,
    pub minimized_body: &'static str,
    pub close_prompt_title: &'static str,
    pub close_prompt_body: &'static str,
}

impl TrayLabels {
    pub fn for_language(lang: Language) -> Self {
        match lang {
            Language::PtBr => Self {
                tooltip: "Kame House Launcher (Online)",
                open_ui: "Abrir Kame House (Frontend)",
                show_console: "Abrir Terminal de Debug (Logs)",
                hide_console: "Ocultar Terminal de Debug",
                quit_app: "Encerrar Kame House",
                minimized_title: "Kame House Minimizado",
                minimized_body:
                    "O Kame House continua online na bandeja do sistema. Clique no ícone para restaurar ou abrir o terminal de debug.",
                close_prompt_title: "Fechar Kame House Launcher",
                close_prompt_body:
                    "Deseja minimizar o Kame House Launcher para a bandeja do sistema (System Tray) ou encerrar a aplicação?\n\n• Clique em [Sim] para MINIMIZAR e continuar online no System Tray.\n• Clique em [Não] para ENCERRAR completamente a aplicação.\n• Clique em [Cancelar] para reabrir a janela do frontend.",
            },
            Language::En => Self {
                tooltip: "Kame House Launcher (Online)",
                open_ui: "Open Kame House (Frontend)",
                show_console: "Open Debug Terminal (Logs)",
                hide_console: "Hide Debug Terminal",
                quit_app: "Quit Kame House",
                minimized_title: "Kame House Minimized",
                minimized_body:
                    "Kame House is still running in the system tray. Click the tray icon to restore or open the debug terminal.",
                close_prompt_title: "Close Kame House Launcher",
                close_prompt_body:
                    "Do you want to minimize Kame House Launcher to the System Tray or quit the application?\n\n• Click [Yes] to MINIMIZE and stay online in the System Tray.\n• Click [No] to QUIT the application completely.\n• Click [Cancel] to reopen the frontend window.",
            },
        }
    }
}

/// Hides the console window on startup when running the GUI daemon on Windows
pub fn hide_console_on_startup_if_daemon(hide: bool) {
    let force_show = std::env::var("KAME_HOUSE_SHOW_CONSOLE").ok().as_deref() == Some("1");
    if hide && !force_show {
        #[cfg(target_os = "windows")]
        win32::set_console_visible(false);
    }
}

/// Toggles or sets the visibility of the Windows Debug Console window
pub fn set_debug_console_visible(lifecycle: &WindowLifecycleState, visible: bool) -> bool {
    lifecycle
        .debug_console_visible
        .store(visible, Ordering::SeqCst);
    info!("Debug terminal visibility set to: {}", visible);
    #[cfg(target_os = "windows")]
    {
        win32::set_console_visible(visible);
    }
    visible
}

pub fn toggle_debug_console(lifecycle: &WindowLifecycleState) -> bool {
    let next = !lifecycle.is_debug_console_visible();
    set_debug_console_visible(lifecycle, next)
}

/// Displays a balloon notification in the System Tray
pub fn notify_minimized_to_tray(lang: Language) {
    let labels = TrayLabels::for_language(lang);
    info!("{}: {}", labels.minimized_title, labels.minimized_body);
    #[cfg(target_os = "windows")]
    {
        win32::show_tray_balloon(labels.minimized_title, labels.minimized_body);
    }
}

/// Prompts the user with a native OS dialog when the window is closed via the native X button
pub fn prompt_native_close_decision(lang: Language) -> CloseDecision {
    let _labels = TrayLabels::for_language(lang);
    #[cfg(target_os = "windows")]
    {
        win32::show_close_confirmation_dialog(&_labels)
    }
    #[cfg(not(target_os = "windows"))]
    {
        CloseDecision::MinimizeToTray
    }
}

/// Starts the OS System Tray icon and context menu worker thread
pub fn start_system_tray(service: Arc<LauncherService>, lifecycle: Arc<WindowLifecycleState>) {
    #[cfg(target_os = "windows")]
    {
        win32::spawn_win32_tray_thread(service, lifecycle);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (service, lifecycle);
        info!("System tray ready (headless/Linux mode)");
    }
}

/// Cleanly removes the System Tray icon before process exit
pub fn cleanup_system_tray() {
    #[cfg(target_os = "windows")]
    {
        win32::remove_tray_icon();
    }
}

#[cfg(target_os = "windows")]
mod win32 {
    use super::{
        set_debug_console_visible, toggle_debug_console, CloseDecision, TrayLabels,
        WindowLifecycleState,
    };
    use crate::LauncherService;
    use std::mem::{size_of, zeroed};
    use std::ptr::null;
    use std::sync::atomic::{AtomicIsize, Ordering};
    use std::sync::{Arc, OnceLock};
    use tracing::info;

    const SW_HIDE: i32 = 0;
    const SW_RESTORE: i32 = 9;
    const SW_SHOW: i32 = 5;

    const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
    const CW_USEDEFAULT: i32 = -2147483648i32;

    const WM_DESTROY: u32 = 0x0002;
    const WM_COMMAND: u32 = 0x0111;
    const WM_USER: u32 = 0x0400;
    const WM_TRAYICON: u32 = WM_USER + 100;

    const WM_LBUTTONUP: u32 = 0x0202;
    const WM_LBUTTONDBLCLK: u32 = 0x0203;
    const WM_RBUTTONUP: u32 = 0x0205;
    const WM_CONTEXTMENU: u32 = 0x007B;

    const NIM_ADD: u32 = 0x00000000;
    const NIM_MODIFY: u32 = 0x00000001;
    const NIM_DELETE: u32 = 0x00000002;

    const NIF_MESSAGE: u32 = 0x00000001;
    const NIF_ICON: u32 = 0x00000002;
    const NIF_TIP: u32 = 0x00000004;
    const NIF_INFO: u32 = 0x00000010;
    const NIIF_INFO: u32 = 0x00000001;

    const MF_STRING: u32 = 0x00000000;
    const MF_SEPARATOR: u32 = 0x00000800;
    const MF_BYCOMMAND: u32 = 0x00000000;
    const TPM_BOTTOMALIGN: u32 = 0x0020;
    const TPM_LEFTALIGN: u32 = 0x0000;
    const TPM_RIGHTBUTTON: u32 = 0x0002;

    const SC_CLOSE: u32 = 0xF060;

    const ID_TRAY_OPEN_UI: usize = 1001;
    const ID_TRAY_TOGGLE_CONSOLE: usize = 1002;
    const ID_TRAY_QUIT: usize = 1003;

    const MB_YESNOCANCEL: u32 = 0x00000003;
    const MB_ICONQUESTION: u32 = 0x00000020;
    const MB_SETFOREGROUND: u32 = 0x00010000;
    const MB_TOPMOST: u32 = 0x00040000;

    const IDYES: i32 = 6;
    const IDNO: i32 = 7;

    static TRAY_HWND: AtomicIsize = AtomicIsize::new(0);
    static GLOBAL_TRAY_CTX: OnceLock<(Arc<LauncherService>, Arc<WindowLifecycleState>)> =
        OnceLock::new();

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct POINT {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct GUID {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct NOTIFYICONDATAW {
        cb_size: u32,
        hwnd: isize,
        u_id: u32,
        u_flags: u32,
        u_callback_message: u32,
        h_icon: isize,
        sz_tip: [u16; 128],
        dw_state: u32,
        dw_state_mask: u32,
        sz_info: [u16; 256],
        u_timeout_or_version: u32,
        sz_info_title: [u16; 64],
        dw_info_flags: u32,
        guid_item: GUID,
        h_balloon_icon: isize,
    }

    #[repr(C)]
    struct WNDCLASSW {
        style: u32,
        lpfn_wnd_proc: unsafe extern "system" fn(isize, u32, usize, isize) -> isize,
        cb_cls_extra: i32,
        cb_wnd_extra: i32,
        h_instance: isize,
        h_icon: isize,
        h_cursor: isize,
        hbr_background: isize,
        lpsz_menu_name: *const u16,
        lpsz_class_name: *const u16,
    }

    #[repr(C)]
    struct MSG {
        hwnd: isize,
        message: u32,
        w_param: usize,
        l_param: isize,
        time: u32,
        pt: POINT,
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleWindow() -> isize;
        fn AllocConsole() -> i32;
        fn SetConsoleTitleW(lp_console_title: *const u16) -> i32;
        fn GetModuleHandleW(lp_module_name: *const u16) -> isize;
    }

    #[link(name = "user32")]
    extern "system" {
        fn ShowWindow(hwnd: isize, n_cmd_show: i32) -> i32;
        fn SetForegroundWindow(hwnd: isize) -> i32;
        fn GetSystemMenu(hwnd: isize, b_revert: i32) -> isize;
        fn DeleteMenu(h_menu: isize, u_position: u32, u_flags: u32) -> i32;
        fn RegisterClassW(lp_wnd_class: *const WNDCLASSW) -> u16;
        fn CreateWindowExW(
            dw_ex_style: u32,
            lp_class_name: *const u16,
            lp_window_name: *const u16,
            dw_style: u32,
            x: i32,
            y: i32,
            n_width: i32,
            n_height: i32,
            h_wnd_parent: isize,
            h_menu: isize,
            h_instance: isize,
            lp_param: *const core::ffi::c_void,
        ) -> isize;
        fn DefWindowProcW(hwnd: isize, msg: u32, w_param: usize, l_param: isize) -> isize;
        fn GetMessageW(
            lp_msg: *mut MSG,
            hwnd: isize,
            w_msg_filter_min: u32,
            w_msg_filter_max: u32,
        ) -> i32;
        fn TranslateMessage(lp_msg: *const MSG) -> i32;
        fn DispatchMessageW(lp_msg: *const MSG) -> isize;
        fn PostQuitMessage(n_exit_code: i32);
        fn CreatePopupMenu() -> isize;
        fn AppendMenuW(
            h_menu: isize,
            u_flags: u32,
            u_id_new_item: usize,
            lp_new_item: *const u16,
        ) -> i32;
        fn TrackPopupMenu(
            h_menu: isize,
            u_flags: u32,
            x: i32,
            y: i32,
            n_reserved: i32,
            hwnd: isize,
            prc_rect: *const core::ffi::c_void,
        ) -> i32;
        fn DestroyMenu(h_menu: isize) -> i32;
        fn GetCursorPos(lp_point: *mut POINT) -> i32;
        fn CreateIcon(
            h_instance: isize,
            n_width: i32,
            n_height: i32,
            c_planes: u8,
            c_bits_pixel: u8,
            lpb_and_bits: *const u8,
            lpb_xor_bits: *const u8,
        ) -> isize;
        fn LoadIconW(h_instance: isize, lp_icon_name: *const u16) -> isize;
        fn MessageBoxW(
            hwnd: isize,
            lp_text: *const u16,
            lp_caption: *const u16,
            u_type: u32,
        ) -> i32;
    }

    #[link(name = "shell32")]
    extern "system" {
        fn Shell_NotifyIconW(dw_message: u32, lp_data: *mut NOTIFYICONDATAW) -> i32;
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn copy_wide_fixed<const N: usize>(dest: &mut [u16; N], text: &str) {
        let encoded: Vec<u16> = text.encode_utf16().collect();
        let len = encoded.len().min(N.saturating_sub(1));
        dest[..len].copy_from_slice(&encoded[..len]);
        dest[len] = 0;
    }

    pub fn set_console_visible(visible: bool) {
        unsafe {
            let mut hwnd = GetConsoleWindow();
            if hwnd == 0 && visible {
                let _ = AllocConsole();
                hwnd = GetConsoleWindow();
            }
            if hwnd != 0 {
                let title = to_wide("Kame House Launcher — Debug Terminal");
                let _ = SetConsoleTitleW(title.as_ptr());
                // Prevent accidental termination via console X button; users hide it via Tray or UI
                let sys_menu = GetSystemMenu(hwnd, 0);
                if sys_menu != 0 {
                    let _ = DeleteMenu(sys_menu, SC_CLOSE, MF_BYCOMMAND);
                }
                if visible {
                    let _ = ShowWindow(hwnd, SW_RESTORE);
                    let _ = ShowWindow(hwnd, SW_SHOW);
                    let _ = SetForegroundWindow(hwnd);
                } else {
                    let _ = ShowWindow(hwnd, SW_HIDE);
                }
            }
        }
    }

    /// Creates a 16x16 32-bit BGRA Dragon Ball / Kame House icon in memory
    unsafe fn create_kame_tray_icon() -> isize {
        let width = 16i32;
        let height = 16i32;
        let and_mask = [0u8; 32]; // 16x16 / 8 = 32 bytes
        let mut xor_bgra = [0u8; 16 * 16 * 4];

        for y in 0..16 {
            for x in 0..16 {
                let dx = (x as f32) - 7.5;
                let dy = (y as f32) - 7.5;
                let dist = (dx * dx + dy * dy).sqrt();
                let idx = (y * 16 + x) * 4;

                if dist <= 7.2 {
                    // Center star highlight
                    let is_star = (dx.abs() <= 1.6 && dy.abs() <= 1.6)
                        || (dx.abs() <= 0.8 && dy.abs() <= 2.8)
                        || (dy.abs() <= 0.8 && dx.abs() <= 2.8);
                    if is_star {
                        // Crimson star (#E63946)
                        xor_bgra[idx] = 0x46; // B
                        xor_bgra[idx + 1] = 0x39; // G
                        xor_bgra[idx + 2] = 0xE6; // R
                        xor_bgra[idx + 3] = 0xFF; // A
                    } else {
                        // Kame House Dragon Ball Amber-Orange (#FF8C00 -> #FFA500)
                        xor_bgra[idx] = 0x00; // B
                        xor_bgra[idx + 1] = 0x95; // G
                        xor_bgra[idx + 2] = 0xFF; // R
                        xor_bgra[idx + 3] = 0xFF; // A
                    }
                }
            }
        }

        let icon = CreateIcon(
            0,
            width,
            height,
            1,
            32,
            and_mask.as_ptr(),
            xor_bgra.as_ptr(),
        );
        if icon != 0 {
            icon
        } else {
            // Fallback to IDI_APPLICATION (32512)
            LoadIconW(0, 32512 as *const u16)
        }
    }

    pub fn show_tray_balloon(title: &str, body: &str) {
        let hwnd = TRAY_HWND.load(Ordering::SeqCst);
        if hwnd == 0 {
            return;
        }
        unsafe {
            let mut nid: NOTIFYICONDATAW = zeroed();
            nid.cb_size = size_of::<NOTIFYICONDATAW>() as u32;
            nid.hwnd = hwnd;
            nid.u_id = 1;
            nid.u_flags = NIF_INFO;
            nid.dw_info_flags = NIIF_INFO;
            copy_wide_fixed(&mut nid.sz_info_title, title);
            copy_wide_fixed(&mut nid.sz_info, body);
            let _ = Shell_NotifyIconW(NIM_MODIFY, &mut nid);
        }
    }

    pub fn remove_tray_icon() {
        let hwnd = TRAY_HWND.swap(0, Ordering::SeqCst);
        if hwnd != 0 {
            unsafe {
                let mut nid: NOTIFYICONDATAW = zeroed();
                nid.cb_size = size_of::<NOTIFYICONDATAW>() as u32;
                nid.hwnd = hwnd;
                nid.u_id = 1;
                let _ = Shell_NotifyIconW(NIM_DELETE, &mut nid);
            }
        }
    }

    pub fn show_close_confirmation_dialog(labels: &TrayLabels) -> CloseDecision {
        unsafe {
            let title = to_wide(labels.close_prompt_title);
            let body = to_wide(labels.close_prompt_body);
            let result = MessageBoxW(
                0,
                body.as_ptr(),
                title.as_ptr(),
                MB_YESNOCANCEL | MB_ICONQUESTION | MB_SETFOREGROUND | MB_TOPMOST,
            );
            match result {
                IDYES => CloseDecision::MinimizeToTray,
                IDNO => CloseDecision::QuitApplication,
                _ => CloseDecision::ReopenWindow,
            }
        }
    }

    unsafe extern "system" fn tray_wnd_proc(
        hwnd: isize,
        msg: u32,
        w_param: usize,
        l_param: isize,
    ) -> isize {
        match msg {
            WM_TRAYICON => {
                let event = (l_param as u32) & 0xFFFF;
                if event == WM_LBUTTONDBLCLK {
                    if let Some((_, lifecycle)) = GLOBAL_TRAY_CTX.get() {
                        let url = lifecycle.get_ui_url();
                        crate::server::open_fullscreen_ui(&url);
                    }
                    return 0;
                } else if event == WM_RBUTTONUP || event == WM_LBUTTONUP || event == WM_CONTEXTMENU
                {
                    if let Some((service, lifecycle)) = GLOBAL_TRAY_CTX.get() {
                        let lang = service.language();
                        let labels = TrayLabels::for_language(lang);
                        let h_menu = CreatePopupMenu();
                        if h_menu != 0 {
                            let open_txt = to_wide(labels.open_ui);
                            let console_txt = if lifecycle.is_debug_console_visible() {
                                to_wide(labels.hide_console)
                            } else {
                                to_wide(labels.show_console)
                            };
                            let quit_txt = to_wide(labels.quit_app);

                            let _ =
                                AppendMenuW(h_menu, MF_STRING, ID_TRAY_OPEN_UI, open_txt.as_ptr());
                            let _ = AppendMenuW(
                                h_menu,
                                MF_STRING,
                                ID_TRAY_TOGGLE_CONSOLE,
                                console_txt.as_ptr(),
                            );
                            let _ = AppendMenuW(h_menu, MF_SEPARATOR, 0, null());
                            let _ = AppendMenuW(h_menu, MF_STRING, ID_TRAY_QUIT, quit_txt.as_ptr());

                            let mut pt = POINT { x: 0, y: 0 };
                            let _ = GetCursorPos(&mut pt);
                            let _ = SetForegroundWindow(hwnd);
                            let _ = TrackPopupMenu(
                                h_menu,
                                TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON,
                                pt.x,
                                pt.y,
                                0,
                                hwnd,
                                null(),
                            );
                            let _ = DestroyMenu(h_menu);
                        }
                    }
                    return 0;
                }
            }
            WM_COMMAND => {
                let cmd_id = w_param & 0xFFFF;
                if let Some((_service, lifecycle)) = GLOBAL_TRAY_CTX.get() {
                    match cmd_id {
                        ID_TRAY_OPEN_UI => {
                            let url = lifecycle.get_ui_url();
                            info!("System Tray action: Opening frontend window ({})", url);
                            crate::server::open_fullscreen_ui(&url);
                        }
                        ID_TRAY_TOGGLE_CONSOLE => {
                            let now_visible = toggle_debug_console(lifecycle);
                            info!(
                                "System Tray action: Toggled debug console (visible={})",
                                now_visible
                            );
                        }
                        ID_TRAY_QUIT => {
                            info!("System Tray action: Quit requested by user");
                            set_debug_console_visible(lifecycle, false);
                            remove_tray_icon();
                            lifecycle.shutdown_requested.store(true, Ordering::SeqCst);
                            PostQuitMessage(0);
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                return 0;
            }
            WM_DESTROY => {
                remove_tray_icon();
                PostQuitMessage(0);
                return 0;
            }
            _ => {}
        }
        DefWindowProcW(hwnd, msg, w_param, l_param)
    }

    pub fn spawn_win32_tray_thread(
        service: Arc<LauncherService>,
        lifecycle: Arc<WindowLifecycleState>,
    ) {
        std::thread::Builder::new()
            .name("kame-house-tray".to_string())
            .spawn(move || unsafe {
                let lang = service.language();
                let labels = TrayLabels::for_language(lang);
                let _ = GLOBAL_TRAY_CTX.set((service, lifecycle));

                let class_name = to_wide("KameHouseTrayWindowClass");
                let h_instance = GetModuleHandleW(null());

                let wc = WNDCLASSW {
                    style: 0,
                    lpfn_wnd_proc: tray_wnd_proc,
                    cb_cls_extra: 0,
                    cb_wnd_extra: 0,
                    h_instance,
                    h_icon: 0,
                    h_cursor: 0,
                    hbr_background: 0,
                    lpsz_menu_name: null(),
                    lpsz_class_name: class_name.as_ptr(),
                };

                let _ = RegisterClassW(&wc);
                let wnd_title = to_wide("KameHouseTrayHiddenWindow");
                let hwnd = CreateWindowExW(
                    0,
                    class_name.as_ptr(),
                    wnd_title.as_ptr(),
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    0,
                    0,
                    h_instance,
                    null(),
                );

                if hwnd == 0 {
                    tracing::warn!("Failed to create Win32 System Tray message window");
                    return;
                }

                TRAY_HWND.store(hwnd, Ordering::SeqCst);

                let mut nid: NOTIFYICONDATAW = zeroed();
                nid.cb_size = size_of::<NOTIFYICONDATAW>() as u32;
                nid.hwnd = hwnd;
                nid.u_id = 1;
                nid.u_flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
                nid.u_callback_message = WM_TRAYICON;
                nid.h_icon = create_kame_tray_icon();
                copy_wide_fixed(&mut nid.sz_tip, labels.tooltip);

                let _ = Shell_NotifyIconW(NIM_ADD, &mut nid);
                info!("Windows System Tray icon initialized successfully");

                let mut msg: MSG = zeroed();
                while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                }
            })
            .ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_lifecycle_state_and_labels() {
        let state = WindowLifecycleState::new("http://127.0.0.1:48080".to_string());
        assert_eq!(state.get_ui_url(), "http://127.0.0.1:48080");
        assert!(!state.is_debug_console_visible());

        let toggled = toggle_debug_console(&state);
        assert!(toggled);
        assert!(state.is_debug_console_visible());

        let toggled_back = toggle_debug_console(&state);
        assert!(!toggled_back);
        assert!(!state.is_debug_console_visible());

        state.set_ui_url("http://127.0.0.1:48081".to_string());
        assert_eq!(state.get_ui_url(), "http://127.0.0.1:48081");

        let before = current_time_ms().saturating_sub(10);
        state.record_page_activity();
        assert!(state.was_recently_active_or_explicit(before));

        let pt_labels = TrayLabels::for_language(Language::PtBr);
        assert!(pt_labels.show_console.contains("Terminal de Debug"));
        assert!(pt_labels.close_prompt_body.contains("System Tray"));

        let en_labels = TrayLabels::for_language(Language::En);
        assert!(en_labels.show_console.contains("Debug Terminal"));
        assert!(en_labels.close_prompt_body.contains("System Tray"));
    }
}
