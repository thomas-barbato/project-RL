//! Windows pointer images prepared once, drawn last when requested by Jouer.cmd.
//! Native cursor handling remains in use when preparation fails or is disabled.
use macroquad::{miniquad::CursorIcon, prelude::*};
use std::cell::RefCell;

struct Pointer {
    texture: Texture2D,
    hotspot: Vec2,
}
thread_local! {
    static POINTERS: RefCell<Option<[Pointer; 3]>> = const { RefCell::new(None) };
}

pub fn active() -> bool {
    POINTERS.with(|p| p.borrow().is_some())
}

pub fn prepare() {
    #[cfg(target_os = "windows")]
    if std::env::var("PROJECT_RL_SOFTWARE_CURSOR").as_deref() == Ok("1") {
        let images = [
            CursorIcon::Default,
            CursorIcon::Pointer,
            CursorIcon::Crosshair,
        ]
        .map(windows::image);
        match images.into_iter().collect::<Result<Vec<_>, _>>() {
            Ok(images) => {
                let pointers = images
                    .into_iter()
                    .map(|(image, hotspot)| {
                        let texture = crate::asset_loading::from_image(&image);
                        texture.set_filter(FilterMode::Nearest);
                        Pointer { texture, hotspot }
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .ok()
                    .unwrap();
                POINTERS.with(|p| *p.borrow_mut() = Some(pointers));
                miniquad::window::show_mouse(false);
            }
            Err(error) => eprintln!("[CURSOR] Native pointer retained: {error}"),
        }
    }
}

pub fn draw(icon: CursorIcon) {
    if !active() {
        return;
    }
    #[cfg(target_os = "windows")]
    let Some(position) = windows::position() else {
        return;
    };
    #[cfg(not(target_os = "windows"))]
    let position = Vec2::from(mouse_position());
    POINTERS.with(|p| {
        let pointers = p.borrow();
        let Some(pointers) = pointers.as_ref() else {
            return;
        };
        let (x, y) = (position.x, position.y);
        if !(0.0..screen_width()).contains(&x) || !(0.0..screen_height()).contains(&y) {
            return;
        }
        let pointer = &pointers[match icon {
            CursorIcon::Pointer => 1,
            CursorIcon::Crosshair => 2,
            _ => 0,
        }];
        push_camera_state();
        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
        gl_use_default_material();
        let scale = screen_dpi_scale().recip();
        draw_texture_ex(
            &pointer.texture,
            x - pointer.hotspot.x * scale,
            y - pointer.hotspot.y * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(pointer.texture.size() * scale),
                ..Default::default()
            },
        );
        pop_camera_state();
    });
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use std::{mem, ptr};
    use winapi::{
        ctypes::c_void,
        shared::windef::{HBITMAP, HDC, HGDIOBJ},
        um::{wingdi::*, winuser::*},
    };
    // DI_IMAGE | DI_MASK, omitted by winapi 0.3.9's winuser bindings.
    const DI_NORMAL: u32 = 0x0003;

    struct Bitmaps(ICONINFO);
    impl Drop for Bitmaps {
        fn drop(&mut self) {
            unsafe {
                if !self.0.hbmColor.is_null() {
                    DeleteObject(self.0.hbmColor as HGDIOBJ);
                }
                if !self.0.hbmMask.is_null() {
                    DeleteObject(self.0.hbmMask as HGDIOBJ);
                }
            }
        }
    }
    struct Surface {
        dc: HDC,
        bitmap: HBITMAP,
        previous: HGDIOBJ,
    }
    impl Drop for Surface {
        fn drop(&mut self) {
            unsafe {
                SelectObject(self.dc, self.previous);
                DeleteObject(self.bitmap as HGDIOBJ);
                DeleteDC(self.dc);
            }
        }
    }

    pub fn position() -> Option<Vec2> {
        unsafe {
            let window = GetForegroundWindow();
            let mut process = 0;
            GetWindowThreadProcessId(window, &mut process);
            if process != std::process::id() {
                return None;
            }
            let mut info: CURSORINFO = mem::zeroed();
            info.cbSize = mem::size_of::<CURSORINFO>() as u32;
            let mut client: winapi::shared::windef::RECT = mem::zeroed();
            if GetCursorInfo(&mut info) == 0
                || info.flags & CURSOR_SUPPRESSED != 0
                || ScreenToClient(window, &mut info.ptScreenPos) == 0
                || GetClientRect(window, &mut client) == 0
                || client.right <= 0
                || client.bottom <= 0
            {
                return None;
            }
            Some(vec2(
                info.ptScreenPos.x as f32 * screen_width() / client.right as f32,
                info.ptScreenPos.y as f32 * screen_height() / client.bottom as f32,
            ))
        }
    }

    pub fn image(icon: CursorIcon) -> Result<(Image, Vec2), String> {
        unsafe {
            let cursor = LoadCursorW(
                ptr::null_mut(),
                match icon {
                    CursorIcon::Pointer => IDC_HAND,
                    CursorIcon::Crosshair => IDC_CROSS,
                    _ => IDC_ARROW,
                },
            );
            let mut info: ICONINFO = mem::zeroed();
            if cursor.is_null() || GetIconInfo(cursor, &mut info) == 0 {
                return Err("System cursor unavailable".into());
            }
            let source = Bitmaps(info);
            let mut bitmap: BITMAP = mem::zeroed();
            let color = !source.0.hbmColor.is_null();
            if GetObjectW(
                if color {
                    source.0.hbmColor
                } else {
                    source.0.hbmMask
                } as _,
                mem::size_of::<BITMAP>() as i32,
                &mut bitmap as *mut _ as _,
            ) == 0
            {
                return Err("System cursor dimensions unavailable".into());
            }
            let w = bitmap.bmWidth;
            let h = bitmap.bmHeight / if color { 1 } else { 2 };
            if !(1..=256).contains(&w) || !(1..=256).contains(&h) {
                return Err("Unsupported system cursor dimensions".into());
            }
            let mut dib: BITMAPINFO = mem::zeroed();
            dib.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
            dib.bmiHeader.biWidth = w;
            dib.bmiHeader.biHeight = -h;
            dib.bmiHeader.biPlanes = 1;
            dib.bmiHeader.biBitCount = 32;
            dib.bmiHeader.biCompression = BI_RGB;
            let dc = CreateCompatibleDC(ptr::null_mut());
            if dc.is_null() {
                return Err("Cursor surface unavailable".into());
            }
            let mut bits: *mut c_void = ptr::null_mut();
            let output = CreateDIBSection(dc, &dib, DIB_RGB_COLORS, &mut bits, ptr::null_mut(), 0);
            if output.is_null() || bits.is_null() {
                if !output.is_null() {
                    DeleteObject(output as HGDIOBJ);
                }
                DeleteDC(dc);
                return Err("Cursor bitmap unavailable".into());
            }
            let _surface = Surface {
                dc,
                bitmap: output,
                previous: SelectObject(dc, output as _),
            };
            let pixels = std::slice::from_raw_parts_mut(bits as *mut u8, (w * h * 4) as usize);
            pixels.fill(0);
            if DrawIconEx(dc, 0, 0, cursor, w, h, 0, ptr::null_mut(), DI_NORMAL) == 0 {
                return Err("Cursor drawing failed".into());
            }
            GdiFlush();
            let black = pixels.to_vec();
            pixels.fill(255);
            if DrawIconEx(dc, 0, 0, cursor, w, h, 0, ptr::null_mut(), DI_NORMAL) == 0 {
                return Err("Cursor drawing failed".into());
            }
            GdiFlush();
            let mut rgba = vec![0; black.len()];
            for ((black, white), out) in black
                .chunks_exact(4)
                .zip(pixels.chunks_exact(4))
                .zip(rgba.chunks_exact_mut(4))
            {
                let alpha = 255
                    - (0..3)
                        .map(|c| white[c].saturating_sub(black[c]))
                        .max()
                        .unwrap();
                if alpha > 0 {
                    for c in 0..3 {
                        out[c] = (u32::from(black[2 - c]) * 255 / u32::from(alpha)).min(255) as u8;
                    }
                    out[3] = alpha;
                }
            }
            Ok((
                Image {
                    bytes: rgba,
                    width: w as u16,
                    height: h as u16,
                },
                vec2(source.0.xHotspot as f32, source.0.yHotspot as f32),
            ))
        }
    }

    #[test]
    fn system_pointer_images_keep_visible_pixels_transparency_and_hotspots() {
        for icon in [
            CursorIcon::Default,
            CursorIcon::Pointer,
            CursorIcon::Crosshair,
        ] {
            let (image, hotspot) = image(icon).unwrap();
            let visible = image.bytes.chunks_exact(4).filter(|p| p[3] > 0).count();
            assert!(visible > 2 && visible < usize::from(image.width) * usize::from(image.height));
            assert!((0.0..f32::from(image.width)).contains(&hotspot.x));
            assert!((0.0..f32::from(image.height)).contains(&hotspot.y));
        }
    }
}
