use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use colored::*;
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

pub struct SystemMonitor {
    sistema: System,
    intervalo: Duration,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let sistema = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        
        SystemMonitor {
            sistema,
            intervalo: Duration::from_secs(1),
        }
    }
    
    pub fn set_intervalo(&mut self, segundos: u64) {
        self.intervalo = Duration::from_secs(segundos);
    }
    
    pub fn monitor_system(&self, running: Arc<AtomicBool>) {
        println!("{}", "Iniciando monitoreo del sistema...".blue());
        println!("{}", "Presione Ctrl+C para detener".yellow());
        
        // Crear una nueva instancia de System para el monitoreo
        let mut sistema = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        
        while running.load(Ordering::SeqCst) {
            sistema.refresh_specifics(RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()));
            
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
                formato_bytes(uso_memoria).blue(),
                formato_bytes(total_memoria),
                format!("{:.1}", porcentaje_memoria).yellow());
            
            thread::sleep(self.intervalo);
        }
        
        println!("{}", "Monitoreo finalizado".green());
    }
}

// Función auxiliar para formatear bytes en MB
fn formato_bytes(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / 1_048_576.0)
}