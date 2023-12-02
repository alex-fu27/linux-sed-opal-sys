#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/*
from     include/uapi/linux/sed-opal.h
#define IOC_OPAL_SAVE		    _IOW('p', 220, struct opal_lock_unlock)
#define IOC_OPAL_LOCK_UNLOCK	    _IOW('p', 221, struct opal_lock_unlock)
#define IOC_OPAL_TAKE_OWNERSHIP	    _IOW('p', 222, struct opal_key)
#define IOC_OPAL_ACTIVATE_LSP       _IOW('p', 223, struct opal_lr_act)
#define IOC_OPAL_SET_PW             _IOW('p', 224, struct opal_new_pw)
#define IOC_OPAL_ACTIVATE_USR       _IOW('p', 225, struct opal_session_info)
#define IOC_OPAL_REVERT_TPR         _IOW('p', 226, struct opal_key)
#define IOC_OPAL_LR_SETUP           _IOW('p', 227, struct opal_user_lr_setup)
#define IOC_OPAL_ADD_USR_TO_LR      _IOW('p', 228, struct opal_lock_unlock)
#define IOC_OPAL_ENABLE_DISABLE_MBR _IOW('p', 229, struct opal_mbr_data)
#define IOC_OPAL_ERASE_LR           _IOW('p', 230, struct opal_session_info)
#define IOC_OPAL_SECURE_ERASE_LR    _IOW('p', 231, struct opal_session_info)
#define IOC_OPAL_PSID_REVERT_TPR    _IOW('p', 232, struct opal_key)
#define IOC_OPAL_MBR_DONE           _IOW('p', 233, struct opal_mbr_done)
#define IOC_OPAL_WRITE_SHADOW_MBR   _IOW('p', 234, struct opal_shadow_mbr)
#define IOC_OPAL_GENERIC_TABLE_RW   _IOW('p', 235, struct opal_read_write_table)
#define IOC_OPAL_GET_STATUS         _IOR('p', 236, struct opal_status)
#define IOC_OPAL_GET_LR_STATUS      _IOW('p', 237, struct opal_lr_status)
#define IOC_OPAL_GET_GEOMETRY       _IOR('p', 238, struct opal_geometry)
#define IOC_OPAL_DISCOVERY          _IOW('p', 239, struct opal_discovery)
#define IOC_OPAL_REVERT_LSP         _IOW('p', 240, struct opal_revert_lsp)
*/

use nix::ioctl_write_ptr;

#[cfg(any(feature="linux_6_1", feature="linux_6_4"))]
use nix::ioctl_read_buf;

ioctl_write_ptr!(ioc_opal_save, b'p', 220, opal_lock_unlock);
ioctl_write_ptr!(ioc_opal_lock_unlock, b'p', 221, opal_lock_unlock);
ioctl_write_ptr!(ioc_opal_take_ownership, b'p', 222, opal_key);
ioctl_write_ptr!(ioc_opal_activate_lsp, b'p', 223, opal_lr_act);
ioctl_write_ptr!(ioc_opal_set_pw, b'p', 224, opal_new_pw);
ioctl_write_ptr!(ioc_opal_activate_usr, b'p', 225, opal_session_info);
ioctl_write_ptr!(ioc_opal_revert_tpr, b'p', 226, opal_key);
ioctl_write_ptr!(ioc_opal_lr_setup, b'p', 227, opal_user_lr_setup);
ioctl_write_ptr!(ioc_opal_add_usr_to_lr, b'p', 228, opal_lock_unlock);
ioctl_write_ptr!(ioc_opal_enable_disable_mbr, b'p', 229, opal_mbr_data);
ioctl_write_ptr!(ioc_opal_erase_lr, b'p', 230, opal_session_info);
ioctl_write_ptr!(ioc_opal_secure_erase_lr, b'p', 231, opal_session_info);
ioctl_write_ptr!(ioc_opal_psid_revert_tpr, b'p', 232, opal_key);
ioctl_write_ptr!(ioc_opal_mbr_done, b'p', 233, opal_mbr_done);
ioctl_write_ptr!(ioc_opal_write_shadow_mbr, b'p', 234, opal_shadow_mbr);
ioctl_write_ptr!(ioc_opal_generic_table_rw, b'p', 235, opal_read_write_table);

#[cfg(feature="linux_6_1")]
ioctl_read_buf!(ioc_opal_get_status, b'p', 236, opal_status);

cfg_if::cfg_if! {
    if #[cfg(feature="linux_6_4")] {
        ioctl_write_ptr!(ioc_opal_get_lr_status, b'p', 237, opal_lr_status);
        ioctl_read_buf!(ioc_opal_get_geometry, b'p', 238, opal_geometry);
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature="linux_6_6")] {
        ioctl_write_ptr!(ioc_opal_discovery, b'p', 239, opal_discovery);
        ioctl_write_ptr!(ioc_opal_revert_lsp, b'p', 240, opal_revert_lsp);
    }
}
