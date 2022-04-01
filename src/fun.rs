use crate::est;

pub fn verif_espacio(memoria: &Vec<i32>, tam_proceso: i32) -> i32{
    //? posicion = -1, no hay espacio
    let mut contador = 0;
    let mut posicion = -1;

    for (indice, proceso) in memoria.iter().enumerate() {
        if *proceso == 0  {
            if contador == 0 {
                posicion = indice as i32;
            }
            contador += 1;
            if contador >= tam_proceso  {
                return posicion;
            }
        }
        else{
            contador = 0;
            posicion = -1;
        }
    }

    return -1;
}

pub fn agregar_memoria(memoria: &Vec<i32>, posicion: i32, pid: i32, tam_proceso: i32) -> Vec<i32> {
    let mut copia_memoria = memoria.clone();

    for i in 0..tam_proceso {
        copia_memoria[(posicion + i) as usize] = pid;
    }

    return copia_memoria;
}

pub fn eliminar_memoria(memoria: &Vec<i32>, pid: i32) -> Vec<i32> {
    let mut copia_memoria = memoria.clone();

    for (indice, proceso) in memoria.iter().enumerate() {
        if *proceso == pid {
            copia_memoria[indice] = 0;
        }
    }

    return copia_memoria;
}

pub fn mostrar_procesos(tabla: &Vec<est::Proceso>){

    println!("----------------------------------------------------------------------------");

    println!("\tID Archivo\tBytes");
    for proceso in tabla.iter() {
        println!("\t{}\t\t{}", proceso.pid, proceso.tam);
    }
        println!("----------------------------------------------------------------------------");


}

pub fn mostrar_tabla_byte(tabla: &Vec<est::Proceso>){

    println!("------------------------------------");
    println!("\tID Archivo\tTamaño");
    for proceso in tabla.iter() {
        println!("\t{}\t{} BYTES", proceso.pid, proceso.tam);
    }
    println!("------------------------------------");

}

pub fn calcular_bloques(memoria: &Vec<i32>, tam_de_bloques: i32, total_de_bloques: i32) -> Vec<est::Bloque> {

    let tam_memoria = memoria.len() as i32;
    let tam_ultimo_bloque = tam_de_bloques - ((tam_de_bloques * total_de_bloques) - tam_memoria);

    let mut memoria_bloques = vec![est::Bloque{pid: 0, tam_total: tam_de_bloques, tam_ocupado: 0}; total_de_bloques as usize];
    memoria_bloques[(total_de_bloques-1) as usize].tam_total = tam_ultimo_bloque;

    return memoria_bloques;
}

pub fn tam_en_bloques(tam_proceso: i32, tam_bloques: i32) -> i32 {

    let tama = ((tam_proceso as f64) / (tam_bloques as f64)).ceil();
    
    return tama as i32;


}

pub fn verif_esp_bloq(memoria: &Vec<est::Bloque>, archivo: est::Proceso) -> i32 {

    let mut espacio = 0;
    let mut posicion_insercion = -1;
    let mut tam_disponible = 0;
    
    for (indice, proceso) in memoria.iter().enumerate() {
        if proceso.pid == 0 {
            if espacio == 0 {
                posicion_insercion = indice as i32;
            }
            
            espacio += 1; 
            tam_disponible += proceso.tam_total;

            if (espacio >= archivo.bloques) && tam_disponible >= archivo.tam { 
                return posicion_insercion;
            }
        }
        else if proceso.pid != 0 {
            espacio = 0;
            posicion_insercion = -1;
            tam_disponible = 0;
        }
    }

    return -1;
}

pub fn agregar_bloques(memoria: &Vec<est::Bloque>, posicion:i32 , archivo: &est::Proceso) -> Vec<est::Bloque>{
    let mut cop_memoria = memoria.clone();
    let pos_final = archivo.bloques + posicion - 1;

    if archivo.bloques == 1 {
        cop_memoria[(pos_final) as usize].pid = archivo.pid;
        cop_memoria[(pos_final) as usize].tam_ocupado = archivo.tam;
    }
    else {

        for indice in 0..(archivo.bloques-1){
            cop_memoria[(indice + posicion) as usize].pid = archivo.pid;
            cop_memoria[(indice + posicion) as usize].tam_ocupado = cop_memoria[(indice + posicion) as usize].tam_total;
        }

        let pos = posicion+archivo.bloques - 1;
        cop_memoria[(pos) as usize].pid = archivo.pid;

        if archivo.no_utilizado == 0 {
            cop_memoria[(pos) as usize].tam_ocupado = cop_memoria[(pos) as usize].tam_total;

        }
        else{
            cop_memoria[(pos) as usize].tam_ocupado = archivo.no_utilizado;
        }

    }
    
    return cop_memoria;
}

pub fn eliminar_tabla(tabla: &Vec<est::Proceso>, id_archivo:i32) -> Vec<est::Proceso> {
    let mut cop_tabla = tabla.clone();

    for (indice, archivo) in tabla.iter().enumerate() {
        if archivo.pid == id_archivo {
            cop_tabla.remove(indice);  
        }
    }
    return cop_tabla;
}

pub fn eliminar_memoria_bloques(memoria: &Vec<est::Bloque>, id_archivo:i32) -> Vec<est::Bloque>{

    let mut cop_memoria = memoria.clone();

    for (indice, archivo) in memoria.iter().enumerate() {
        if archivo.pid == id_archivo {
            cop_memoria[indice].pid = 0;
            cop_memoria[indice].tam_ocupado = 0;
        }
    }

    return cop_memoria;
}

pub fn ver_tabla_bloques(tabla: &Vec<est::Proceso>, tam_bloque: i32, memoria: &Vec<est::Bloque>){

    let mut tam_ocupado = 0;
    let mut tam_desp = 0;
    let mut bloques_usados = 0;
    let cop_memoria = memoria.clone();
    let  ultimo_archivo = cop_memoria[cop_memoria.len() - 1];

    println!("----------------------------------------------------------------------------");
    println!("\tID Archivo\tBloques\t   Tamaño\t   Espacio no utilizado");

    for archivo in tabla.iter() {
        
        if archivo.pid == ultimo_archivo.pid{
            tam_ocupado += archivo.tam;
            bloques_usados += archivo.bloques;
            if archivo.no_utilizado == 0{
                println!("\t  {}\t\t  {}\t   {} bytes\t\t{} bytes",archivo.pid, archivo.bloques, archivo.tam, archivo.no_utilizado);
            }
            else{
                tam_desp += (archivo.no_utilizado-ultimo_archivo.tam_total)*-1;
                println!("\t  {}\t\t  {}\t   {} bytes\t\t{} bytes",archivo.pid, archivo.bloques, archivo.tam, (archivo.no_utilizado-ultimo_archivo.tam_total)*-1);
            }
        }
        else{
            tam_ocupado += archivo.tam;
            bloques_usados += archivo.bloques;

            if  archivo.no_utilizado !=0 {
                tam_desp += (archivo.no_utilizado-tam_bloque)*-1;
                println!("\t  {}\t\t  {}\t   {} bytes\t\t{} bytes",archivo.pid, archivo.bloques, archivo.tam, (archivo.no_utilizado-tam_bloque)*-1);
            }
            else {
                tam_desp += archivo.no_utilizado;
                println!("\t  {}\t\t  {}\t   {} bytes\t\t{} bytes",archivo.pid, archivo.bloques, archivo.tam, (archivo.no_utilizado));
            }
        }
        
    }
    println!("----------------------------------------------------------------------------");
    println!("Total:   Bloques Usados: {},   Bytes Usados: {},   Bytes No Usados: {}", bloques_usados, tam_ocupado, tam_desp);
    println!("----------------------------------------------------------------------------");
}
