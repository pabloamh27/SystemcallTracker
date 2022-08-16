/*  Estudiante y autor: Pablo Muñoz Hidalgo
    Carnet: 2020031899
    Profesor: Kevin Moraga García 
    Materia: Sistemas Operativos
    Grupo: 20
    Universidad: ITCR
    Año: 2022
*/

//Modulos y bibliotecas a usar
mod nombres_syscalls;

use std::io::Read;
use nix::sys::wait::wait;
use linux_personality::personality;
use nix::sys::ptrace;
use std::os::unix::process::CommandExt;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use nix::unistd::{fork, ForkResult, Pid};
use std::process::{exit, Command};
use std::io::{self, stdin};

/*
FUNCION:
    Funcion que se encarga de ejecutar la syscall que se le pasa por parametro
    y retorna el valor de la syscall.
PARAMETROS:
    argumentos: Vec<String> -> Vector de argumentos que se le pasan a la syscall
*/
fn comprobador (mut argumentos : Vec<String>) {

    //Condicional para verificar que el usuario ingrese "rastreador" como primer instrucción
    if argumentos [0] != "rastreador" {
        println!("El comando debe inciar con la palabra 'rastreador'");
        exit(1);
    }
    //Agrega un argumento de prog vacío para que el programa no se cierre
    if argumentos.len() <= 4 {
        let comodin = " ".to_string();
        argumentos.push(comodin);
    }
    //Verifica que el usuario haya ingresado una opcion de rastreador válida
    if argumentos [1] == "-v"{
        rastreador(&argumentos [2], argumentos_comando(argumentos.to_vec()));
        exit(1);
    }
    //Verifica que el usuario haya ingresado una opcion de rastreador válida
    if argumentos [1] == "-V" {
        rastreador_con_pausa(&argumentos [2], argumentos_comando(argumentos.to_vec()));
        exit(1);
    }
    //Error en caso de que el usuario no ingrese una opcion de rastreador válida
    else{
        println!("El comando debe tener una opcion valida: -v o -V");
        exit(1);
    }

}

/*FUNCION:
    Esta funcion se encarga de convertir los argumentos de la función menu a un vector de strings
    que contiene los argumentos del comando que se ejecutará.
    PARAMETROS:
        Vec<String> -> Vector de strings que contiene los argumentos del comando que se ejecutará
    Retorno:
        Vec<String> -> Vector de strings que contiene los argumentos del comando que se ejecutará
*/
fn argumentos_comando (argumentos : Vec<String>) -> Vec<String> {
    let mut argumentos_comando : Vec<String> = Vec::new();
    for i in 3..argumentos.len() {
        argumentos_comando.push(argumentos [i].to_string());
    }
    return argumentos_comando;
}

/*FUNCION:
    Funcion que ejecuta el rastreador con la opcion -v
PARAMETROS:
    prog: String -> Nombre del syscall a rastrear
    argumentos_prog: Vec<String> -> Vector de argumentos del programa a ejecutar
*/
fn rastreador(prog: &String, argumentos_prog: Vec<String>) {
    match unsafe {fork()} 
    {
        //Si el fork es exitoso, ejecuta el hijo
        Ok(ForkResult::Child) => {
            //Ejecuta el hijo
            ejecutar_programa(&prog, argumentos_prog);
        }
        //Empieza a sacar informacion del hijo y la muestra al usuario
        Ok(ForkResult::Parent { child }) => {
            let vector_syscalls = rastrear(child);
            println!("\n\nLa cantidad de syscalls utilizados por cada uno son: \n");
            for i in 0..vector_syscalls.len() {
                println!("Nombre del syscall: {:?} \nCantidad: {:?}", vector_syscalls[i].0, vector_syscalls[i].1);
            }
        }
        //Error en caso de que el fork falle
        Err(err) => {
            println!("El fork ha fallado! {}", err);
            exit(1);
        }
    }
}

/*
FUNCION:
    Funcion que ejecuta el rastreador con pausa usando la opcion -V
PARAMETROS:
    prog: String -> Nombre del syscall a rastrear
    argumentos_prog: Vec<String> -> Vector de argumentos del programa a ejecutar
*/
fn rastreador_con_pausa(prog: &String, argumentos_prog: Vec<String>) {
    match unsafe {fork()} 
    {
        //Si el fork es exitoso, ejecuta el hijo
        Ok(ForkResult::Child) => {
            //Ejecuta el programa
            ejecutar_programa(&prog, argumentos_prog);
        }
        //Empieza a sacar informacion del hijo y la muestra al usuario
        Ok(ForkResult::Parent { child }) => {
            let vector_syscalls = rastrear_con_pausa(child);
            println!("\n\nLa cantidad de syscalls utilizados por cada uno son: \n");
            for i in 0..vector_syscalls.len() {
                println!("Nombre del syscall: {:?} \nCantidad: {:?}", vector_syscalls[i].0, vector_syscalls[i].1);
            }
        }
        //Error en caso de que el fork falle
        Err(err) => {
            println!("El fork ha fallado! {}", err);
            exit(1);
        }
    }
}

/*
FUNCION:
    Funcion que ejecuta el hijo y lo rastrea
PARAMETROS:
    child: Pid -> Pid del hijo
Retorno:
    Vec<(String, i128)> -> Vector de pares de strings y enteros que contiene el nombre del syscall y la cantidad de veces que se ha utilizado
*/
fn rastrear(id_hijo: Pid) -> Vec<(String, i128)>{
    let mut vec_syscalls: Vec<(String, i128)> = Vec::new();
    loop {
        wait().unwrap();
        //Usa ptrace para enviar la informacion del hijo al vector de syscalls
        match ptrace::getregs(id_hijo) {
            Ok(regs) => {
                vec_syscalls = contador_syscalls(nombres_syscalls::NOMBRES_SYSCALLS[(regs.orig_rax) as usize], &mut vec_syscalls);
            }    
            Err(_) => break,
        }
        //Usa ptrace para recoger la informacion del hijo e imprimirla al usuario
        match ptrace::getregs(id_hijo) {
            Ok(x) => println!("
            \nNombre del syscall: {:?} 
            \nNúmero de syscall: {:?}
            \nValor de retorno: {:?}
            \nDetalles en crudo: {:?}
            \n=======================================================",
                nombres_syscalls::NOMBRES_SYSCALLS[(x.orig_rax) as usize],
                x.orig_rax,
                x.rax,
                x
            ),
            Err(_) => break,
        };
        //Usa ptrace para detener el hijo
        match ptrace::syscall(id_hijo, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    return vec_syscalls.to_vec();
}



/*
FUNCION:
    Funcion que ejecuta el hijo y lo rastrea con pausas implementando la funcion "continuar()"
PARAMETROS:
    child: Pid -> Pid del hijo
Retorno:
    Vec<(String, i128)> -> Vector de pares de strings y enteros que contiene el nombre del syscall y la cantidad de veces que se ha utilizado
*/
fn rastrear_con_pausa(id_hijo: Pid) ->  Vec<(String, i128)>{
    let mut vec_syscalls: Vec<(String, i128)> = Vec::new();
    loop {
        wait().unwrap();
        //Usa ptrace para enviar la informacion del hijo al vector de syscalls
        match ptrace::getregs(id_hijo) {
            Ok(regs) => {
                vec_syscalls = contador_syscalls(nombres_syscalls::NOMBRES_SYSCALLS[(regs.orig_rax) as usize], &mut vec_syscalls);
            }    
            Err(_) => break,
        }
        //Usa ptrace para recoger la informacion del hijo e imprimirla al usuario
        match ptrace::getregs(id_hijo) {
            Ok(x) => println!("
            \nNombre del syscall: {:?} 
            \nNúmero de syscall: {:?}
            \nValor de retorno: {:?}
            \nDetalles en crudo: {:?}
            \n=======================================================",
                nombres_syscalls::NOMBRES_SYSCALLS[(x.orig_rax) as usize],
                x.orig_rax,
                x.rax,
                x
            ),
            Err(_) => break,
        };
        //Espera input del usuario para continuar con el rastreo
        pausa();
        //Usa ptrace para detener el hijo
        match ptrace::syscall(id_hijo, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
    return vec_syscalls.to_vec();
}

/*
FUNCION:
    Funcion que ejecuta el hijo y ejecuta el programa pasado por parametro
*/
fn ejecutar_programa(prog: &String, argumentos_prog: Vec<String>) {
    ptrace::traceme().unwrap();
    //Ejecuta el programa que el usuario ingreso
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    Command::new(prog).args(argumentos_prog).exec();
    exit(0)
}

/*
FUNCION:
    Funcion que contabiliza la cantidad de veces que se ha utilizado un syscall
PARAMETROS:
    syscall: String -> Nombre del syscall
    vector_syscalls: Vec<(String, i128)> -> Vector de pares de strings y enteros que contiene el nombre del syscall y la cantidad de veces que se ha utilizado
Retorno:
    Vec<(String, i128)> -> Vector de pares de strings y enteros que contiene el nombre del syscall y la cantidad de veces que se ha utilizado
*/
fn contador_syscalls(syscall: &str, vector_syscalls: &mut Vec<(String, i128)>)  -> Vec<(String, i128)> {
    let mut contador: i128 = 0;
    //Recorre el vector de syscalls
    for i in 0..vector_syscalls.len() {
        //Si el syscall es igual, suma 1 al contador
        if vector_syscalls[i].0 == syscall {
            contador = vector_syscalls[i].1;
            contador += 1;
            vector_syscalls[i].1 = contador;
            break;
        }
    }
    //Si el syscall no esta en el vector, lo agrega
    if contador == 0 {
        vector_syscalls.push((syscall.to_string(), 1));
    }
    //Devuelve el vector con los syscalls actualizados
    let vec_syscalls: Vec<(String, i128)> = vector_syscalls.to_vec();
    return vec_syscalls;
}


/*
FUNCION:
    Funcion que espera input del usuario para continuar con el rastreo
*/
fn pausa() {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();  // Clono la configuración original para luego restablecerlo para la siguiente ejecucion
    new_termios.c_lflag &= !(ICANON | ECHO); 
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1]; 
    //Espera input del usuario para continuar con el rastreo
    println!("Presione cualquier tecla para continuar... \n");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap(); // Restablezco la configuración original
                                       
}


fn main() {
    //Recibe el input del usuario y lo guarda en una variable de tipo String para poder usarlo en el siguiente paso
    let mut input = String::new();
    println!("Bienvenido al rastreador, ingrese su comando de la forma: rastreador <opcion de rastreador> <programa a ejecutar> <argumentos para el programa>: ");
    stdin().read_line(&mut input).unwrap();
    let argumentos: Vec<String> = input.split_whitespace().map(str::to_string).collect();
    //Llama al menu de opciones
    comprobador(argumentos);

}