# Tarea 1 
- Estudiante: Pablo Alberto Muñoz Hidalgo
- Profesor: Kevin Moraga García
- Curso y Universidad: ITCR Sistemas Operativos
- Año: 2022
# Introducción
El problema a resolver en esta ocasión es realizar un "syscall tracker" o rastreador de llamadas al sistema, este deberá rastrear las llamadas al sistema que realice un programa, tomando en cuenta parámetros que le introduzca el usuario, este rastreador tendrá dos modos ("-V" y "-v") los cuales tienen funciones diferentes. Una vez rastreados debe imprimir en terminal los resultados encontrados.



# Ambiente de desarrollo
Se estará utilizando Ubuntu 20.04.4 LTS y como IDE se utilizará Visual Studio Code. Además de un repositorio en github.

## Estructuras de datos usadas y funciones:


## Instrucciones para ejecutar el programa:
Paso 1: Ejecutar el programa
Paso 2: Ingresar el texto "rastreador <_opción del rastreador_> <_programa a ejecutar_> <_argumentos para el programa a ejecutar (opcional)_>" 
Paso 3: Presionar la tecla Enter
*En caso de que se haya seleccionado la opcion de rastreador "-V" presionar la tecla enter despues de cada impresion en pantalla*
Ejemplo de linea a ejecutar "rastreador -v ls /home/user/Desktop/"

## Actividades realizadas por estudiante:
|Fecha|Hora de Inicio|Hora de Finalización|Actividad realizada|
|-----|----------|-------|-------|
|9/08/2022|7:00 PM|8:00 PM|Creación del git y descargar add-ons de Visual Studio Code|
|10/08/2022|7:00 PM|9:30 PM|Investigar sobre syscalls y su comportamiento|
|11/08/2022|6:00 PM|9:00 PM|Investigación sobre ptrace, strace y lurk, además como adaptarlo a Rust|
|14/08/2022|6:30 PM| 9:30 PM|Primer commit, creación de la documentación y avance en syscalls, además de investigación en "split()"|
|14/08/2022|9:30 PM|11:00 PM|Hacer comprobaciones necesarias para el corrido del programa, el programa funciona sin argumentos en el "Prog"|
|15/08/2022|8:00 AM|10:00 AM|Se logró pasar argumentos al comando y se trabajó en las opciones de rastreador|
|15/08/2022|2:00 PM|5:00 PM|Se implementa el continuar presionando cualquier botón y se cuentan los syscalls de cada tipo, se suman y se imprimen en pantalla|
|15/08/2022|5:15 PM|8:30 PM|Release final con todo funcionando como lo solicita la especificacion|

## Autoevaluación:
|Opción -v|Opción -V|Ejecución de Prog|Análisis de syscalls|Documentación|
|-----|------|-------|-----|------|
|10/10|20/20|20/20|30/30|20/20|


## Lecciones Aprendidas:
En esta tarea se aprendió el funcionamiento de los "syscalls" y como se comportan estos a la hora de ejecutar un programa, además de aprender lo básico sobre estos también se sacó provecho del lenguaje Rust, un lenguaje útil e intuitivo teniendo en cuenta experiencias pasadas con C y Java. Por último se aprecia mucho la aplicación de los conocimientos adquiridos en las lecturas, ya que muchas veces esto se queda en teoría pero con esta tarea todo pasó a la parte práctica y es muy satisfactorio haberla finalizado exitosamente.


## Bibliografía:
[1] "lib.rs.html -- source". Docs.rs. [https://docs.rs/linux/0.0.1/src/linux/lib.rs.html#1-21](https://docs.rs/linux/0.0.1/src/linux/lib.rs.html#1-21) (accedido el 16 de agosto de 2022).
[2] "Communicating with the OS - The Node Experiment - Exploring Async Basics with Rust". Site not found · GitHub Pages. [https://cfsamson.github.io/book-exploring-async-basics/3_1_communicating_with_the_os.html](https://cfsamson.github.io/book-exploring-async-basics/3_1_communicating_with_the_os.html) (accedido el 16 de agosto de 2022).
[3] "strace(1): trace system calls/signals - Linux man page". Linux Documentation. [https://linux.die.net/man/1/strace](https://linux.die.net/man/1/strace) (accedido el 16 de agosto de 2022).
[4] "linux::syscall - Rust". Docs.rs. [https://docs.rs/linux/0.0.1/linux/syscall/index.html](https://docs.rs/linux/0.0.1/linux/syscall/index.html) (accedido el 16 de agosto de 2022).
[5] "System programming in Rust, take 2". 128nops and counting. [https://carstein.github.io/2022/05/29/rust-system-programming-2.html](https://carstein.github.io/2022/05/29/rust-system-programming-2.html) (accedido el 16 de agosto de 2022).
[6] "GitHub - JakWai01/lurk: A pretty (simple) alternative to strace". GitHub. [https://github.com/JakWai01/lurk](https://github.com/JakWai01/lurk) (accedido el 16 de agosto de 2022).
[7] "Implementing strace in Rust". Jakob Waibel. [https://jakobwaibel.com/2022/06/06/ptrace/](https://jakobwaibel.com/2022/06/06/ptrace/) (accedido el 16 de agosto de 2022).
[8] "How to Split a String in Rust? (Explained with Examples)". Become A Better Programmer - Trust The Process. [https://www.becomebetterprogrammer.com/split-string-rust/](https://www.becomebetterprogrammer.com/split-string-rust/) (accedido el 16 de agosto de 2022).
[9] "What are some good ways to implement a read line or a sleep timer in Rust". Stack Overflow. [https://stackoverflow.com/questions/66823720/what-are-some-good-ways-to-implement-a-read-line-or-a-sleep-timer-in-rust](https://stackoverflow.com/questions/66823720/what-are-some-good-ways-to-implement-a-read-line-or-a-sleep-timer-in-rust) (accedido el 16 de agosto de 2022).
[10] "How can I read one character from stdin without having to hit enter?" Stack Overflow. [https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter](https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter) (accedido el 16 de agosto de 2022).
[11] "Storing Lists of Values with Vectors - The Rust Programming Language". Learn Rust - Rust Programming Language. [https://doc.rust-lang.org/book/ch08-01-vectors.html](https://doc.rust-lang.org/book/ch08-01-vectors.html) (accedido el 16 de agosto de 2022).
