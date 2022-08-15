mod system_call_names;

use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
//use std::fmt::Arguments;
//use std::os::linux::process;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use std::io::stdin;

fn main() {
    //Recibe el input del usuario y lo guarda en una variable de tipo String para poder usarlo en el siguiente paso
    let mut input = String::new();
    println!("Bienvenido al rastreador, ingrese su comando de la forma: rastreador <argumento> <Prog> <argumento_de_Prog>");
    stdin().read_line(&mut input).unwrap();
    splitter(&input);

    fn splitter (input: &String) {
        //Separar el input en un vector de strings con el split del espacio
        let argumentos: Vec<String> = input.split_whitespace().map(str::to_string).collect();
    
        //Condicional para verificar que el usuario ingrese "rastreador" como primer instrucción
        if argumentos [0] != "rastreador" {
            println!("El comando debe inciar con la palabra 'rastreador'");
            exit(1);
        }
        
        let prog = argumentos [2].clone(); //Por el momento solo recibe la "Prog" pero no sus argumentos.
        //Verifica que el usuario haya ingresado una opcion de rastreador válida
        if argumentos [1] == "-v"{
            rastreador(&prog);
            exit(1);
        }
        //Verifica que el usuario haya ingresado una opcion de rastreador válida
        if argumentos [1] == "-V" {
            rastreador(&prog);
            exit(1);
        }
        //Error en caso de que el usuario no ingrese una opcion de rastreador válida

        else{
            println!("El comando debe tener una opcion valida: -v o -V");
            exit(1);
        }

    }
    

    
}

//Funcion que ejecuta el rastreador
fn rastreador(system_call_name: &String) {
    match unsafe { fork() } {
        //Si el fork es exitoso, ejecuta el hijo
        Ok(ForkResult::Child) => {
            //Ejecuta el hijo
            ejecutar_hijo(&system_call_name);
        }
        Ok(ForkResult::Parent { child }) => {
            rastrear_hijo(child);
        }
        //Error en caso de que el fork falle
        Err(err) => {
            panic!("El fork ha fallado! {}", err);
        }
    }
}

fn rastrear_hijo(child: Pid) {
    loop {
        wait().unwrap();
        //Usa ptrace para recoger el valor de la system call que se esta ejecutando en el hijo
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

//Funcion que ejecuta el hijo
fn ejecutar_hijo(system_call_name: &String) {
    ptrace::traceme().unwrap();
    //Ejecuta el programa que el usuario ingreso
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();
    Command::new(system_call_name).exec();

    exit(0)
}