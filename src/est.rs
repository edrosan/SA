
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Proceso {
    pub pid: i32,
    pub tam: i32,
    pub bloques: i32,
    pub no_utilizado: i32,//tamaño del ultimo bloque
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Bloque {
    pub pid: i32,
    pub tam_total: i32,
    pub tam_ocupado: i32,
}