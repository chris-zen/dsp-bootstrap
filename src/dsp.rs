pub fn process(in_buffer: &[f32], out_buffer: &mut [f32], frames: usize) {
  // TODO Add your code here
  out_buffer.copy_from_slice(in_buffer); // simple pass-through
}