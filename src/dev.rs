use std::ptr;

pub struct BlkidDev(libblkid_rs_sys::blkid_dev);

pub struct BlkidDevIter(libblkid_rs_sys::blkid_dev_iterate);

impl BlkidDevIter {
    pub(crate) fn new(iter: libblkid_rs_sys::blkid_dev_iterate) -> Self {
        BlkidDevIter(iter)
    }
}

impl Iterator for BlkidDevIter {
    type Item = BlkidDev;

    fn next(&mut self) -> Option<Self::Item> {
        let mut dev: libblkid_rs_sys::blkid_dev = ptr::null_mut();
        if unsafe { libblkid_rs_sys::blkid_dev_next(self.0, &mut dev as *mut _) } < 0 {
            None
        } else {
            Some(BlkidDev(dev))
        }
    }
}

impl Drop for BlkidDevIter {
    fn drop(&mut self) {
        unsafe { libblkid_rs_sys::blkid_dev_iterate_end(self.0) }
    }
}
