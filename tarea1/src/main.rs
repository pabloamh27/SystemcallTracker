/*  Estudiante y autor: Pablo Muñoz Hidalgo
    Carnet: 2020031899
    Profesor: Kevin Moraga García 
    Materia: Sistemas Operativos
    Grupo: 20
    Universidad: ITCR
    Año: 2022
 */

//Modulos y bibliotecas a usar
mod system_call_names;

use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::io::stdin;

fn main() {
    //Recibe el input del usuario y lo guarda en una variable de tipo String para poder usarlo en el siguiente paso
    let mut input = String::new();
    println!("Bienvenido al rastreador, ingrese su comando de la forma: rastreador <argumento> <Prog> <argumento_de_Prog>");
    stdin().read_line(&mut input).unwrap();
    let argumentos: Vec<String> = input.split_whitespace().map(str::to_string).collect();
    menu(argumentos);  
}

//Este es el menú de opciones que se muestra al usuario y comprueba que los inputs estén bien escritos
fn menu (mut argumentos : Vec<String>) {

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
        rastreador(&argumentos [2], &argumentos [3]);
        exit(1);
    }
    //Verifica que el usuario haya ingresado una opcion de rastreador válida
    if argumentos [1] == "-V" {
        rastreador_con_pausa(&argumentos [2], &argumentos [3]);
        exit(1);
    }
    //Error en caso de que el usuario no ingrese una opcion de rastreador válida
    else{
        println!("El comando debe tener una opcion valida: -v o -V");
        exit(1);
    }

}

//Funcion que ejecuta el rastreador
fn rastreador(system_call_name: &String, argumentos_prog: &String) {
    match unsafe { fork() } {
        //Si el fork es exitoso, ejecuta el hijo
        Ok(ForkResult::Child) => {
            //Ejecuta el hijo
            ejecutar_hijo(&system_call_name, &argumentos_prog);
        }
        //Empieza a sacar informacion del hijo y la muestra al usuario
        Ok(ForkResult::Parent { child }) => {
            rastrear(child);
        }
        //Error en caso de que el fork falle
        Err(err) => {
            panic!("El fork ha fallado! {}", err);
        }
    }
}

//Funcion que ejecuta el rastreador con pausa
fn rastreador_con_pausa(system_call_name: &String, argumentos_prog: &String) {
    match unsafe { fork() } {
        //Si el fork es exitoso, ejecuta el hijo
        Ok(ForkResult::Child) => {
            //Ejecuta el hijo
            ejecutar_hijo(&system_call_name, &argumentos_prog);
        }
        //Empieza a sacar informacion del hijo y la muestra al usuario
        Ok(ForkResult::Parent { child }) => {
            rastrear_con_pausa(child);
        }
        //Error en caso de que el fork falle
        Err(err) => {
            panic!("El fork ha fallado! {}", err);
        }
    }
}

//Rastrea e imprime la informacion del hijo
fn rastrear(child: Pid) {
    loop {
        wait().unwrap();
        //Usa ptrace para recoger la informacion del hijo e imprimirla al usuario
        match ptrace::getregs(child) {
            Ok(x) => println!(
                "{:?} {:?}",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],
                x
            ),
            Err(_) => break,
        };
        //Usa ptrace para detener el hijo
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

//Rastrea e imprime la informacion del hijo con pausa
fn rastrear_con_pausa(child: Pid) {
    loop {
        wait().unwrap();
        //Usa ptrace para recoger la informacion del hijo e imprimirla al usuario
        //para contar los syscalls usar ptrace y sus getters.
        match ptrace::getregs(child) {
            Ok(x) => println!(
                "{:?} {:?}",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize],
                x
            ),
            Err(_) => break,
        };
        //Espera input del usuario para continuar con el rastreo
        println!("Presione enter para continuar");
        let mut pasar_siguiente = String::new();
        stdin().read_line(&mut pasar_siguiente).unwrap();
        //Usa ptrace para detener el hijo
        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

//Funcion que ejecuta el hijo
fn ejecutar_hijo(system_call_name: &String, argumentos_prog: &String) {
    ptrace::traceme().unwrap();
    //Ejecuta el programa que el usuario ingreso
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    Command::new(system_call_name).arg(argumentos_prog).exec();

    exit(0)
}