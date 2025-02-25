use std::time::{Instant, Duration};
use std::thread;
use std::sync::mpsc;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use walkdir::WalkDir;
use colored::*;
use sysinfo::Disks;
use rayon::prelude::*;

pub struct Finder;

impl Finder {
    /// Obtiene todas las unidades de disco disponibles
    pub fn obtener_unidades() -> Vec<String> {
        let disks = Disks::new_with_refreshed_list();
        let mut unidades = Vec::new();
        
        for disco in disks.list() {
            if let Some(ruta) = disco.mount_point().to_str() {
                unidades.push(ruta.to_string());
            }
        }
        
        unidades
    }

    /// Crea un hilo para mostrar el tiempo transcurrido
    fn crear_hilo_tiempo(inicio: Instant, rx: mpsc::Receiver<()>) {
        thread::spawn(move || {
            let mut ultimo_segundo = 0;
            loop {
                if rx.try_recv().is_ok() {
                    break;
                }
                
                let transcurrido = inicio.elapsed().as_secs();
                if transcurrido > ultimo_segundo {
                    print!("\rTiempo transcurrido: {} segundos", transcurrido);
                    io::stdout().flush().unwrap();
                    ultimo_segundo = transcurrido;
                }
                thread::sleep(Duration::from_millis(100));
            }
            println!(); // Nueva línea al terminar
        });
    }

    /// Busca archivos que contengan el patrón especificado en su nombre
    /// y muestra sus rutas completas
    pub fn buscar_archivos(patron: &str) {
        let encontrados = Arc::new(AtomicBool::new(false));
        let inicio = Instant::now();
        
        // Obtener todas las unidades
        let unidades = Self::obtener_unidades();
        println!("Unidades detectadas: {}", unidades.join(", ").yellow());
        println!("(Esto puede tardar varios minutos ya que buscará en todas las unidades)");
        println!("Usando {} threads para la búsqueda", rayon::current_num_threads());
        
        // Crear un canal para comunicación entre hilos
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(tx);
        
        // Crear el hilo para mostrar el tiempo
        Self::crear_hilo_tiempo(inicio, rx);
        
        // Buscar en cada unidad en paralelo
        unidades.par_iter().for_each(|unidad| {
            println!("\nBuscando en unidad: {}", unidad.yellow());
            
            // Recorrer el directorio y todos sus subdirectorios
            WalkDir::new(unidad)
                .follow_links(true)
                .same_file_system(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .for_each(|entrada| {
                    if let Some(nombre) = entrada.file_name().to_str() {
                        if nombre.to_lowercase() == patron.to_lowercase() {
                            println!("\nEncontrado en: {}", entrada.path().display().to_string().green());
                            encontrados.store(true, Ordering::Relaxed);
                        }
                    }
                });
        });
        
        // Detener el contador de tiempo
        let _ = tx.send(());
        thread::sleep(Duration::from_millis(200));
        
        // Mostrar resultado final
        let duracion = inicio.elapsed();
        println!("\n{}", "=".repeat(50));
        if encontrados.load(Ordering::Relaxed) {
            println!("Búsqueda completada en {} segundos.", duracion.as_secs().to_string().yellow());
        } else {
            println!("{}", "No se encontraron archivos que coincidan con el patrón.".red());
            println!("Tiempo total de búsqueda: {} segundos", duracion.as_secs().to_string().yellow());
        }
    }
}
