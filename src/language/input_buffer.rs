use std::str::from_utf8_unchecked;

pub struct InputBuffer {
    bytes: Vec<u8>,
    edit_start: usize,
}

impl InputBuffer {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(1024),
            edit_start: 0,
        }
    }

    pub(crate) fn update(
        &mut self,
        new_input: &str,
        edit_pos: usize,
        old_end: usize,
        new_end: usize,
    ) {
        let new_len = self.bytes.len() - (old_end - edit_pos) + (new_end - edit_pos);
        if self.bytes.capacity() < new_len {
            self.bytes.reserve(new_len - self.bytes.len());
        }
        self.bytes.truncate(edit_pos);
        let slice = new_input[edit_pos..new_end].as_bytes();
        self.bytes.extend_from_slice(slice);
        self.edit_start = edit_pos;
    }

    pub(crate) fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.bytes) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::input_buffer::InputBuffer;

    #[test]
    fn test_input_buffer_new() {
        let buffer = InputBuffer::new();
        assert_eq!(buffer.as_str(), "");
        assert_eq!(buffer.edit_start, 0);
    }

    #[test]
    fn test_input_buffer_update() {
        let mut buffer = InputBuffer::new();
        buffer.update("123", 0, 0, 3);
        assert_eq!(buffer.as_str(), "123");

        // Test partial update
        buffer.update("1245", 2, 3, 4);
        assert_eq!(buffer.as_str(), "1245");
    }

    #[test]
    fn test_input_buffer_update_spaces() {
        let mut buffer = InputBuffer::new();
        buffer.update("1 1", 0, 0, 3);
        assert_eq!(buffer.as_str(), "1 1");

        buffer.update("1 1 1", 3, 3, 5);
        assert_eq!(buffer.as_str(), "1 1 1");

        buffer.update("1 1 1 1", 5, 5, 7);
        assert_eq!(buffer.as_str(), "1 1 1 1");
    }
}
