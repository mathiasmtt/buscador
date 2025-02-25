# Buscador de Archivos en Rust

Un buscador de archivos eficiente implementado en Rust que utiliza paralelismo para buscar archivos en todas las unidades disponibles del sistema.

## Características

- Búsqueda en paralelo utilizando múltiples hilos
- Insensible a mayúsculas/minúsculas
- Muestra el tiempo de búsqueda en tiempo real
- Detecta automáticamente todas las unidades disponibles
- Interfaz de línea de comandos simple

## Requisitos

- Rust 1.70 o superior
- Cargo (incluido con Rust)

## Dependencias

- walkdir = "2.4.0" - Para recorrer directorios
- colored = "2.1.0" - Para colorear la salida
- sysinfo = "0.30.5" - Para detectar unidades de disco
- rayon = "1.8" - Para paralelismo

## Uso

El programa puede ejecutarse de dos formas:

### 1. Modo Interactivo

```bash
cargo run
```

Esto mostrará un menú interactivo con las siguientes opciones:
1. Buscar archivo - Permite buscar un archivo por nombre en todas las unidades
2. Analizar sistema - Muestra información detallada del sistema
3. Salir - Termina el programa

### 2. Modo Directo (Búsqueda rápida)

```bash
cargo run -- <nombre_archivo>
```

Por ejemplo:
```bash
cargo run -- documento.txt
```

Este modo realiza directamente la búsqueda del archivo especificado sin mostrar el menú.

## Estructura del Proyecto

El proyecto está organizado en módulos para una mejor mantenibilidad:

- `src/main.rs` - Punto de entrada de la aplicación
- `src/finder.rs` - Módulo para la búsqueda de archivos
- `src/system_info.rs` - Módulo para monitoreo del sistema
- `Cargo.toml` - Archivo de configuración y dependencias
- `README.md` - Este archivo

### Módulo Finder

El módulo `finder.rs` implementa la funcionalidad de búsqueda de archivos con las siguientes características:

- Búsqueda paralela en todas las unidades disponibles
- Monitoreo del tiempo de búsqueda en tiempo real
- Detección automática de unidades
- Búsqueda insensible a mayúsculas/minúsculas

### Ejemplo de Uso del Finder

```rust
use finder::Finder;

// Buscar un archivo
Finder::buscar_archivos("documento.txt");
```

## Módulo de Monitoreo del Sistema

El módulo `system_info.rs` proporciona un monitor en tiempo real que muestra:

- Información básica del sistema (nombre, kernel, SO, hostname)
- Estado de la memoria (RAM y swap)
- Uso de CPU en tiempo real
- Información de red (bytes recibidos/transmitidos)
- Temperatura de componentes

#### Características del Monitor

- Actualización en tiempo real cada segundo
- Interfaz limpia y organizada
- Solo se actualizan los valores numéricos, manteniendo la estructura fija
- Control mediante Ctrl+C para detener el monitoreo

#### Ejemplo de Salida del Monitor

```
=== MONITOR DEL SISTEMA ===

Presione Ctrl+C para detener el monitoreo

Información del Sistema:
  Nombre del Sistema: Darwin
  Versión del Kernel: 22.3.0
  Versión del SO: 13.3.1
  Nombre del Host: MacBook-Pro

Información de Memoria:
  Memoria Total: 16384 MB
  Memoria Usada: 8192 MB
  Swap Total: 4096 MB
  Swap Usada: 1024 MB

Información de CPU:
  Número total de CPUs: 8
  CPU 0: 25%
  CPU 1: 30%
  CPU 2: 15%
  ...

Información de Red:
  Interfaz: en0
    Recibido: 1024 MB
    Transmitido: 512 MB
  ...

Información de Componentes:
  CPU: 45°C
  GPU: 55°C
  ...
```

### Monitor del Sistema

El monitor del sistema proporciona información en tiempo real sobre:

- **Información General del Sistema**
  - Nombre del sistema
  - Versión del kernel
  - Sistema operativo
  - Nombre del host

- **Memoria**
  - Memoria total
  - Memoria en uso
  - Swap total
  - Swap en uso

- **CPU**
  - Uso de cada núcleo en porcentaje
  - Frecuencia en MHz

- **Discos**
  - Punto de montaje
  - Espacio total
  - Espacio disponible

- **Red**
  - Interfaces de red
  - Datos recibidos
  - Datos enviados

Para acceder al monitor del sistema, seleccione la opción "Analizar sistema" en el menú interactivo.

### Formato de Visualización

```
=== INFORMACIÓN DEL SISTEMA ===

Sistema:         Darwin
Kernel:          24.3.0
Sistema Op:      14.3.1
Host:            Desktop.local

=== MEMORIA ===

Total:           16384 MB
En uso:          10852 MB
Swap Total:      0 MB
Swap en uso:     0 MB

=== CPU ===

CPU | Uso % | MHz
-----------------
  0 | 34.2  | 4.0
  1 | 26.1  | 4.0
  2 | 20.8  | 4.0
  3 | 13.9  | 4.0

=== DISCOS ===

Montaje: /
Total:       500 GB
Disponible:  200 GB

=== RED ===

Interfaz: en0
Recibido:     1024 KB
Enviado:      512 KB
```

### Características

- Interfaz simple y robusta
- Formato consistente y fácil de leer
- Actualización cada 2 segundos de:
  - Información del sistema operativo
  - Uso de memoria RAM y swap
  - Uso de CPU por núcleo
  - Espacio en disco
  - Tráfico de red

### Ejemplo de Uso

```rust
use buscador::system_info::SystemMonitor;

fn main() {
    // Crear una nueva instancia del monitor
    let mut monitor = SystemMonitor::new();
    
    // Iniciar el monitoreo (se actualiza cada 2 segundos)
    monitor.monitor_system().unwrap();
}
```

Para detener el monitor, presiona Ctrl+C.

## Control de Versiones

El proyecto usa Git para control de versiones. Cada commit representa una versión estable del código.
