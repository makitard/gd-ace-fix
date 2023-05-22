use retour::static_detour;
use std::error::Error;

static_detour! {
    static d_collectItem: unsafe extern "thiscall" fn(*const (), i32, i32);
    static d_collectedObject: unsafe extern "thiscall" fn(*const (), *const ());
}

macro_rules! init_gd_hook {
    ($base:expr, $d:ident, $off:expr, $h:ident) => {
        $d.initialize(std::mem::transmute($base + $off), $h)?
            .enable()?;
    };
}

pub(super) unsafe fn init() -> Result<(), Box<dyn Error>> {
    let base =
        windows::Win32::System::LibraryLoader::GetModuleHandleA(windows::core::PCSTR(0 as _))?.0
            as usize;

    init_gd_hook!(base, d_collectItem, 0x111890, h_collectItem);

    init_gd_hook!(base, d_collectedObject, 0x111830, h_collectedObject);

    Ok(())
}

fn h_collectItem(this: *const (), item: i32, count: i32) {
    unsafe {
        d_collectItem.call(this, item.clamp(0, 1099), count);
    }
}

fn h_collectedObject(this: *const (), object: *const ()) {
    unsafe {
        #[allow(non_snake_case)]
        let m_itemBlockAID = (object as usize + 0x52c) as *mut i32;
        if m_itemBlockAID as usize != 0 {
            *m_itemBlockAID = (*m_itemBlockAID).clamp(0, 1099);
        }
        d_collectedObject.call(this, object);
    }
}
