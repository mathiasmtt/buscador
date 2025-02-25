use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use colored::*;
use sysinfo::{System, Disks, Networks};

pub struct SystemMonitor {
    sistema: System,
    intervalo: Duration,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let sistema = System::new();
        
        SystemMonitor {
            sistema,
            intervalo: Duration::from_secs(1),
        }
    }
    
    pub fn set_intervalo(&mut self, segundos: u64) {
        self.intervalo = Duration::from_secs(segundos);
    }
    
    pub fn obtener_info_completa(&mut self) -> String {
        // Refrescar datos del sistema
        self.sistema.refresh_all();
        
        let mut info = String::new();
        
        // Información del sistema
        info.push_str("=== INFORMACIÓN DEL SISTEMA ===\n");
        info.push_str(&format!("Sistema:         {}\n", System::name().unwrap_or_else(|| String::from("Desconocido"))));
        info.push_str(&format!("Kernel:          {}\n", System::kernel_version().unwrap_or_else(|| String::from("Desconocido"))));
        info.push_str(&format!("Sistema Op:      {}\n", System::os_version().unwrap_or_else(|| String::from("Desconocido"))));
        info.push_str(&format!("Host:            {}\n", System::host_name().unwrap_or_else(|| String::from("Desconocido"))));
        
        // Información de memoria
        let total_memoria = self.sistema.total_memory();
        let uso_memoria = self.sistema.used_memory();
        let total_swap = self.sistema.total_swap();
        let uso_swap = self.sistema.used_swap();
        
        info.push_str("\n=== MEMORIA ===\n");
        info.push_str(&format!("Total:           {} MB\n", formato_bytes(total_memoria)));
        info.push_str(&format!("En uso:          {} MB\n", formato_bytes(uso_memoria)));
        info.push_str(&format!("Swap Total:      {} MB\n", formato_bytes(total_swap)));
        info.push_str(&format!("Swap en uso:     {} MB\n", formato_bytes(uso_swap)));
        
        // Información de CPU
        info.push_str("\n=== CPU ===\n");
        info.push_str("CPU | Uso % | MHz\n");
        info.push_str("-----------------\n");
        
        for (i, cpu) in self.sistema.cpus().iter().enumerate() {
            info.push_str(&format!("{:3} | {:.1}  | {:.1}\n", 
                i, 
                cpu.cpu_usage(), 
                cpu.frequency()));
        }
        
        // Información de discos
        info.push_str("\n=== DISCOS ===\n");
        let disks = Disks::new();
        
        for disk in disks.list() {
            let total = disk.total_space();
            let disponible = disk.available_space();
            
            if let Some(mount_point) = disk.mount_point().to_str() {
                info.push_str(&format!("Montaje: {}\n", mount_point));
                info.push_str(&format!("Total:       {} GB\n", total / 1_073_741_824));
                info.push_str(&format!("Disponible:  {} GB\n", disponible / 1_073_741_824));
            }
        }
        
        // Información de red
        info.push_str("\n=== RED ===\n");
        let networks = Networks::new();
        
        for (nombre, datos) in networks.iter() {
            info.push_str(&format!("Interfaz: {}\n", nombre));
            info.push_str(&format!("Recibido:     {} KB\n", datos.received() / 1024));
            info.push_str(&format!("Enviado:      {} KB\n", datos.transmitted() / 1024));
        }
        
        info
    }
    
    pub fn monitor_system(&self, running: Arc<AtomicBool>) {
        println!("{}", "Iniciando monitoreo del sistema...".blue());
        println!("{}", "Presione Ctrl+C para detener".yellow());
        
        // Crear una nueva instancia de System para el monitoreo
        let mut sistema = System::new();
        
        while running.load(Ordering::SeqCst) {
            sistema.refresh_all();
            
            // Uso de CPU (promedio de todos los cores)
            let uso_cpu: f32 = sistema.cpus().iter()
                .map(|cpu| cpu.cpu_usage())
                .sum::<f32>() / sistema.cpus().len() as f32;
            
            // Uso de memoria
            let total_memoria = sistema.total_memory();
            let uso_memoria = sistema.used_memory();
            let porcentaje_memoria = (uso_memoria as f64 / total_memoria as f64) * 100.0;
            
            // Mostrar información
            println!("CPU: {}%  |  Memoria: {}/{} MB ({}%)", 
                format!("{:.1}", uso_cpu).green(),
                formato_bytes(uso_memoria),
                formato_bytes(total_memoria),
                format!("{:.1}", porcentaje_memoria).yellow());
            
            thread::sleep(self.intervalo);
        }
        
        println!("{}", "Monitoreo finalizado".green());
    }
}

// Función auxiliar para formatear bytes en MB
fn formato_bytes(bytes: u64) -> String {
    format!("{}", bytes / 1_048_576)
}