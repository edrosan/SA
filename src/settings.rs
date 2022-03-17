

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Setting {
    pub byte: bool,
    pub bloque: bool,
    pub tam_bloque: i32,
}


pub fn def_setting(byte: bool, bloque: bool, tam_bloque: i32) -> Setting {
    return Setting{ byte, bloque, tam_bloque }
}