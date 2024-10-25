pub fn rgb_to_rgba(rgb_buffer: Vec<u8>) -> Vec<u8> {
    let rgb_len = rgb_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((rgb_len / 3) * 4); // Ogni pixel RGB diventa RGBA

    // Itera i pixel RGB e aggiungi il canale Alpha
    for rgb_chunk in rgb_buffer.chunks_exact(3) {
        rgba_buffer.push(rgb_chunk[0]); // Red
        rgba_buffer.push(rgb_chunk[1]); // Green
        rgba_buffer.push(rgb_chunk[2]); // Blue
        rgba_buffer.push(255);          // Alpha (opaco)
    }

    rgba_buffer
}

// Funzione da BGR0 a RGBA
pub fn bgr0_to_rgba(bgr0_buffer: Vec<u8>) -> Vec<u8> {
    let bgr0_len = bgr0_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((bgr0_len / 4) * 4); // Ogni pixel BGR0 diventa RGBA

    for bgr0_chunk in bgr0_buffer.chunks_exact(4) {
        rgba_buffer.push(bgr0_chunk[2]); // Red
        rgba_buffer.push(bgr0_chunk[1]); // Green
        rgba_buffer.push(bgr0_chunk[0]); // Blue
        rgba_buffer.push(255);           // Alpha (opaco)
    }

    rgba_buffer
}

// Funzione da RGBx a RGBA
pub fn rgbx_to_rgba(rgbx_buffer: Vec<u8>) -> Vec<u8> {
    let rgbx_len = rgbx_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((rgbx_len / 4) * 4); // Ogni pixel RGBx diventa RGBA

    for rgbx_chunk in rgbx_buffer.chunks_exact(4) {
        rgba_buffer.push(rgbx_chunk[0]); // Red
        rgba_buffer.push(rgbx_chunk[1]); // Green
        rgba_buffer.push(rgbx_chunk[2]); // Blue
        rgba_buffer.push(255);           // Alpha (opaco)
    }

    rgba_buffer
}

// Funzione da XBGR a RGBA
pub fn xbgr_to_rgba(xbgr_buffer: Vec<u8>) -> Vec<u8> {
    let xbgr_len = xbgr_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((xbgr_len / 4) * 4); // Ogni pixel XBGR diventa RGBA

    for xbgr_chunk in xbgr_buffer.chunks_exact(4) {
        rgba_buffer.push(xbgr_chunk[3]); // Red
        rgba_buffer.push(xbgr_chunk[2]); // Green
        rgba_buffer.push(xbgr_chunk[1]); // Blue
        rgba_buffer.push(255);           // Alpha (opaco)
    }

    rgba_buffer
}

// Funzione da BGRx a RGBA
pub fn bgrx_to_rgba(bgrx_buffer: Vec<u8>) -> Vec<u8> {
    let bgrx_len = bgrx_buffer.len();
    let mut rgba_buffer = Vec::with_capacity((bgrx_len / 4) * 4); // Ogni pixel BGRx diventa RGBA

    for bgrx_chunk in bgrx_buffer.chunks_exact(4) {
        rgba_buffer.push(bgrx_chunk[2]); // Red
        rgba_buffer.push(bgrx_chunk[1]); // Green
        rgba_buffer.push(bgrx_chunk[0]); // Blue
        rgba_buffer.push(255);           // Alpha (opaco)
    }

    rgba_buffer
}

// Funzione da BGRA a RGBA
pub fn bgra_to_rgba(bgra_buffer: Vec<u8>) -> Vec<u8> {
    let bgra_len = bgra_buffer.len();
    let mut rgba_buffer = Vec::with_capacity(bgra_len); // Ogni pixel BGRA diventa RGBA

    for bgra_chunk in bgra_buffer.chunks_exact(4) {
        rgba_buffer.push(bgra_chunk[2]); // Red
        rgba_buffer.push(bgra_chunk[1]); // Green
        rgba_buffer.push(bgra_chunk[0]); // Blue
        rgba_buffer.push(bgra_chunk[3]); // Alpha
    }

    rgba_buffer
}
