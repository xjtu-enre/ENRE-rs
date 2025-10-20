// 实体总数：39
// 关系总数：78

// [关系] Use: 使用 ostd::sync 模块中的 SpinLock
use ostd::sync::SpinLock;

// [关系] Use: 使用 crate 中的多个模块和项
use crate::{
    device::tty::{PushCharError, Tty, TtyDriver},
    events::IoEvents,
    prelude::{Errno, Result, return_errno_with_message},
    process::signal::Pollee,
    util::ring_buffer::RingBuffer,
};

// [实体] Constant: BUFFER_CAPACITY
// [关系] Define: 定义常量 BUFFER_CAPACITY
const BUFFER_CAPACITY: usize = 8192;

/// A pseudoterminal driver.
///
/// This is contained in the PTY slave, but it maintains the output buffer and the pollee of the
/// master. The pollee of the slave is part of the [`Tty`] structure (see the definition of
/// [`PtySlave`]).
// [实体] Struct: PtyDriver
// [关系] Define: 定义结构体 PtyDriver
// [关系] Contain: PtyDriver 包含 output 和 pollee 字段
pub struct PtyDriver {
    // [实体] Variable: output (字段)
    output: SpinLock<RingBuffer<u8>>,
    // [实体] Variable: pollee (字段)
    pollee: Pollee,
}

/// A pseudoterminal slave.
// [实体] Type Alias: PtySlave
// [关系] Define: 定义类型别名 PtySlave
pub type PtySlave = Tty<PtyDriver>;

// [关系] Impl: 为 PtyDriver 实现方法
impl PtyDriver {
    // [实体] Function/Method: new
    // [关系] Define: 定义方法 new
    pub(super) fn new() -> Self {
        Self {
            // [关系] Call: 调用 SpinLock::new
            // [关系] Call: 调用 RingBuffer::new
            output: SpinLock::new(RingBuffer::new(BUFFER_CAPACITY)),
            // [关系] Call: 调用 Pollee::new
            pollee: Pollee::new(),
        }
    }

    // [实体] Function/Method: try_read
    // [关系] Define: 定义方法 try_read
    pub(super) fn try_read(&self, buf: &mut [u8]) -> Result<usize> {
        // [实体] Variable: buf (参数)
        // [实体] Variable: output (局部变量)
        // [实体] Variable: read_len (局部变量)
        if buf.is_empty() {
            return Ok(0);
        }

        // [关系] Call: 调用 self.output.lock()
        // [关系] UseVar: 使用变量 output
        let mut output = self.output.lock();
        // [关系] Call: 调用 output.is_empty()
        // [关系] UseVar: 使用变量 output
        if output.is_empty() {
            // [关系] Call: 调用宏 return_errno_with_message
            return_errno_with_message!(Errno::EAGAIN, "the buffer is empty");
        }

        // [关系] Call: 调用 output.len()
        // [关系] Call: 调用 buf.len()
        // [关系] UseVar: 使用变量 output, buf
        let read_len = output.len().min(buf.len());
        // [关系] Call: 调用 output.pop_slice
        // [关系] Modify: 修改 output 和 buf
        // [关系] UseVar: 使用变量 output, buf, read_len
        output.pop_slice(&mut buf[..read_len]).unwrap();

        // [关系] UseVar: 使用变量 read_len
        Ok(read_len)
    }

    // [实体] Function/Method: pollee
    // [关系] Define: 定义方法 pollee
    pub(super) fn pollee(&self) -> &Pollee {
        // [关系] Reference: 返回对 self.pollee 的引用
        // [关系] UseVar: 使用变量 self.pollee
        &self.pollee
    }

    // [实体] Function/Method: buffer_len
    // [关系] Define: 定义方法 buffer_len
    pub(super) fn buffer_len(&self) -> usize {
        // [关系] Call: 调用 self.output.lock()
        // [关系] Call: 调用 lock().len()
        // [关系] UseVar: 使用变量 self.output
        self.output.lock().len()
    }
}

// [实体] Trait: TtyDriver (通过 impl 知道其存在)
// [关系] Impl: 实现 TtyDriver trait for PtyDriver
impl TtyDriver for PtyDriver {
    // [实体] Function/Method: push_output
    // [关系] Define: 定义方法 push_output
    fn push_output(&self, chs: &[u8]) -> core::result::Result<usize, PushCharError> {
        // [实体] Variable: chs (参数)
        // [实体] Variable: output (局部变量)
        // [实体] Variable: len (局部变量)
        // [实体] Variable: ch (循环变量)
        // [关系] Call: 调用 self.output.lock()
        // [关系] UseVar: 使用变量 self.output
        let mut output = self.output.lock();

        let mut len = 0;
        // [关系] UseVar: 使用变量 chs
        for ch in chs {
            // [关系] UseVar: 使用变量 ch, output
            // [关系] Call: 调用 output.capacity(), output.len()
            // TODO: This is termios-specific behavior and should be part of the TTY implementation
            // instead of the TTY driver implementation. See the ONLCR flag for more details.
            if *ch == b'\n' && output.capacity() - output.len() >= 2 {
                // [关系] Call: 调用 output.push
                // [关系] Modify: 修改 output
                output.push(b'\r').unwrap();
                // [关系] Call: 调用 output.push
                // [关系] Modify: 修改 output
                output.push(b'\n').unwrap();
            // [关系] UseVar: 使用变量 ch, output
            // [关系] Call: 调用 output.is_full()
            } else if *ch != b'\n' && !output.is_full() {
                // [关系] Call: 调用 output.push
                // [关系] Modify: 修改 output
                output.push(*ch).unwrap();
            } else if len == 0 {
                // [关系] UseVar: 使用变量 len
                return Err(PushCharError);
            } else {
                break;
            }
            // [关系] Modify: 修改 len
            // [关系] UseVar: 使用变量 len
            len += 1;
        }

        // [关系] Call: 调用 self.pollee.notify
        // [关系] UseVar: 使用变量 self.pollee, IoEvents::IN
        self.pollee.notify(IoEvents::IN);
        // [关系] UseVar: 使用变量 len
        Ok(len)
    }

    // [实体] Function/Method: drain_output
    // [关系] Define: 定义方法 drain_output
    fn drain_output(&self) {
        // [关系] Call: 调用 self.output.lock().clear()
        // [关系] UseVar: 使用变量 self.output
        self.output.lock().clear();
        // [关系] Call: 调用 self.pollee.invalidate()
        // [关系] UseVar: 使用变量 self.pollee
        self.pollee.invalidate();
    }

    // [实体] Function/Method: echo_callback
    // [关系] Define: 定义方法 echo_callback
    fn echo_callback(&self) -> impl FnMut(&[u8]) + '_ {
        // [实体] Variable: output (局部变量)
        // [实体] Variable: has_notified (局部变量)
        // [关系] Call: 调用 self.output.lock()
        // [关系] UseVar: 使用变量 self.output
        let mut output = self.output.lock();
        let mut has_notified = false;

        // [实体] Variable: chs (闭包参数)
        // [实体] Variable: ch (循环变量)
        move |chs| {
            // [关系] UseVar: 使用变量 chs
            for ch in chs {
                // [关系] UseVar: 使用变量 ch, output
                // [关系] Call: 调用 output.push
                // [关系] Modify: 修改 output
                let _ = output.push(*ch);
            }

            // [关系] UseVar: 使用变量 has_notified
            if !has_notified {
                // [关系] Call: 调用 self.pollee.notify
                // [关系] UseVar: 使用变量 self.pollee, IoEvents::IN
                self.pollee.notify(IoEvents::IN);
                // [关系] Modify: 修改 has_notified
                // [关系] UseVar: 使用变量 has_notified
                has_notified = true;
            }
        }
    }

    // [实体] Function/Method: can_push
    // [关系] Define: 定义方法 can_push
    fn can_push(&self) -> bool {
        // [实体] Variable: output (局部变量)
        // [关系] Call: 调用 self.output.lock()
        // [关系] UseVar: 使用变量 self.output
        let output = self.output.lock();
        // [关系] Call: 调用 output.capacity(), output.len()
        // [关系] UseVar: 使用变量 output
        output.capacity() - output.len() >= 2
    }

    // [实体] Function/Method: notify_input
    // [关系] Define: 定义方法 notify_input
    fn notify_input(&self) {
        // [关系] Call: 调用 self.pollee.notify
        // [关系] UseVar: 使用变量 self.pollee, IoEvents::OUT
        self.pollee.notify(IoEvents::OUT);
    }

    // [实体] Function/Method: set_font
    // [关系] Define: 定义方法 set_font
    // [实体] Variable: _font (参数)
    fn set_font(&self, _font: aster_console::BitmapFont) -> Result<()> {
        // [关系] Call: 调用宏 return_errno_with_message
        // [关系] UseVar: 使用变量 _font (虽然未使用，但定义了)
        return_errno_with_message!(Errno::ENOTTY, "the console has no support for font setting");
    }
}
