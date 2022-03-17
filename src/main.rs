use std::io;

mod est;
mod fun;
mod settings;

fn main() {
    let mut run: bool = true;
    let mut pid = 218;

    let mut config = settings::def_setting(false, false, 0);
    let mut buffer;

    const TAM: i32 = 10;
    let mut ram = vec![0; TAM as usize];
    let mut tabla: Vec<est::Proceso> = Vec::new();

    let mut memoria_bloques;

    while run {
        println!("---------------------------");
        println!("1.BYTE");
        println!("2.Bloques");
        println!("3.Salir");

        buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Ha ocurrido un error");

        match buffer.trim().parse::<i32>().unwrap() {
            1 => {
                config = settings::def_setting(true, false, 0);
            }
            2 => {
                println!("Ingresa el tamaño de los bloques");
                buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Ha ocurrido un error");

                let tam = buffer.trim().parse::<i32>().unwrap();
                config = settings::def_setting(false, true, tam);
            }
            3 => {
                run = false;
            }
            _ => println!("Opcion no valida"),
        }

        //-------------------------------------------------------------------------------------------------------------------------------
        if config.byte {
            while run {
                println!("");
                println!("1.Insertar archivo");
                println!("2.Eliminar archivo");
                println!("3.Visualizar almacenamiento");
                println!("4.Salir");

                buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Ha ocurrido un error");

                match buffer.trim().parse::<i32>().unwrap() {
                    1 => {
                        //Insertar archivo
                        println!("Ingrese el tamaño del archivo.");
                        buffer = String::new();
                        io::stdin()
                            .read_line(&mut buffer)
                            .expect("Ha ocurrido un error");

                        let tam_proceso = buffer.trim().parse::<i32>().unwrap();
                        let posicion = fun::verif_espacio(&ram, tam_proceso);

                        if posicion != -1 {
                            ram = fun::agregar_memoria(&ram, posicion, pid, tam_proceso);
                            tabla.push(est::Proceso {
                                pid: pid,
                                tam: tam_proceso,
                                bloques: 0,
                                no_utilizado: 0,
                            });
                            pid += 1;
                            println!("{:?}", ram);
                        }
                        else {
                            println!("No se puede insertar");
                        }
                    }
                    2 => {
                        //Eliminar archivo
                        if tabla.len() == 0 {
                            println!("");
                            println!("No hay archivos a eliminar");
                            println!("");
                        } else {
                            fun::mostrar_procesos(&tabla);
                            println!("Ingrese el PID del archivo a eliminar.");
                            buffer = String::new();
                            io::stdin()
                                .read_line(&mut buffer)
                                .expect("Ha ocurrido un error");
                            let pid_eliminar = buffer.trim().parse::<i32>().unwrap();
                            ram = fun::eliminar_memoria(&ram, pid_eliminar);
                            tabla = fun::eliminar_tabla(&tabla, pid_eliminar);
                        }
                    }
                    3 => {
                        //Visualizar
                        println!("RAM: BYTES");
                        println!("{:?}", ram);
                        fun::mostrar_tabla_byte(&tabla);
                        println!("");
                    }
                    4 => {
                        //Salir
                        run = false;
                    }
                    _ => println!("Opcion no valida"),
                }
            }
        }
        //-------------------------------------------------------------------------------------------------------------------------------
        else if config.bloque {
            let total_bloques_memoria = (TAM as f64 / config.tam_bloque as f64).ceil() as i32;
            let mut ram_bloques = vec![0; total_bloques_memoria as usize];

            memoria_bloques = fun::calcular_bloques(&ram, config.tam_bloque, total_bloques_memoria);

            while run {
                println!("");
                println!("1.Insertar archivo");
                println!("2.Eliminar archivo");
                println!("3.Visualizar almacenamiento");
                println!("4.Salir");

                buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Ha ocurrido un error");

                match buffer.trim().parse::<i32>().unwrap() {
                    //------------------------------Insertar Archivo
                    1 => {
                        //Insertar archivo
                        println!("");
                        println!("Ingrese tamaño del archivo");
                        buffer = String::new();
                        io::stdin()
                            .read_line(&mut buffer)
                            .expect("Ha ocurrido un error");

                        let tam_proceso = buffer.trim().parse::<i32>().unwrap();
                        let tam_en_bloques = fun::tam_en_bloques(tam_proceso, config.tam_bloque);
                        let tam_no_util = tam_proceso % config.tam_bloque;

                        let archivo = est::Proceso {
                            pid: pid + 1,
                            tam: tam_proceso,
                            bloques: tam_en_bloques,
                            no_utilizado: tam_no_util,
                        };

                        let posicion_insercion = fun::verif_esp_bloq(&memoria_bloques, archivo);

                        // si hay espacio se agrega
                        if posicion_insercion != -1 {
                            pid += 1;
                            tabla.push(archivo);
                            ram_bloques = fun::agregar_memoria(
                                &ram_bloques,
                                posicion_insercion,
                                pid,
                                archivo.bloques,
                            );
                            memoria_bloques = fun::agregar_bloques(
                                &memoria_bloques,
                                posicion_insercion,
                                &archivo,
                            );
                        }
                    }
                    2 => {
                        //Eliminar archivo
                        fun::mostrar_procesos(&tabla);
                        println!("Ingrese el PID del archivo a eliminar.");
                        buffer = String::new();
                        io::stdin()
                            .read_line(&mut buffer)
                            .expect("Ha ocurrido un error");
                        let archivo_eliminar = buffer.trim().parse::<i32>().unwrap();

                        ram_bloques = fun::eliminar_memoria(&ram_bloques, archivo_eliminar);
                        tabla = fun::eliminar_tabla(&tabla, archivo_eliminar);

                        memoria_bloques =
                            fun::eliminar_memoria_bloques(&memoria_bloques, archivo_eliminar);
                    }
                    3 => {
                        //Visualizar
                        println!("----------------------------------------------------------------------------");
                        println!("Archivos:\n{:?}", ram_bloques);
                        fun::ver_tabla_bloques(&tabla, config.tam_bloque);
                    }
                    4 => {
                        //Salir
                        run = false;
                    }
                    _ => println!("Opcion no valida"),
                }
            }
        }
    }

    println!("{:?}", config);
}
