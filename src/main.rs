use std::env;
use std::io::{self, Write};
use colored::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::error::Error;

mod system_info;
mod finder;

use system_info::SystemMonitor;
use finder::Finder;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn mostrar_menu() -> io::Result<()> {
    println!("\n{}", "=== MENÚ PRINCIPAL ===".green());
    println!("1. {}", "Buscar archivo".yellow());
    println!("2. {}", "Monitorear sistema".blue());
    println!("3. {}", "Salir".red());
    print!("\nSeleccione una opción (1-3): ");
    io::stdout().flush()
}

fn analizar_sistema() {
    let monitor = SystemMonitor::new();
    
    // Configurar el manejador de Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\n{}", "Deteniendo monitoreo...".yellow());
        r.store(false, Ordering::SeqCst);
    }).expect("Error configurando el manejador de Ctrl+C");

    // Iniciar el monitoreo
    monitor.monitor_system(running);
}

fn buscar_archivo() {
    print!("\nIngrese el nombre del archivo a buscar: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let patron = input.trim();
    
    if patron.is_empty() {
        println!("{}", "Error: Debe ingresar un nombre de archivo.".red());
        return;
    }
    
    println!("Buscando archivo con nombre exacto: {}", patron.yellow());
    Finder::buscar_archivos(patron);
}

fn main() -> Result<()> {
    // Si se proporcionan argumentos, asumimos que es una búsqueda directa
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let patron = &args[1];
        println!("Buscando archivo con nombre exacto: {}", patron.yellow());
        Finder::buscar_archivos(patron);
        return Ok(());
    }

    loop {
        if let Err(e) = mostrar_menu() {
            eprintln!("Error al mostrar el menú: {}", e);
        }
        
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error al leer la entrada: {}", e);
            continue;
        }
        
        match input.trim() {
            "1" => buscar_archivo(),
            "2" => analizar_sistema(),
            "3" => {
                println!("{}", "¡Hasta luego!".green());
                break;
            }
            _ => println!("{}", "Opción no válida. Por favor, seleccione 1, 2 o 3.".red()),
        }
    }
    
    Ok(())
}