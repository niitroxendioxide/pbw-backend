## PBW Backend

Backend en Rust para ejecutar scripts Lua que dibujan sobre una grilla de píxeles y exponer el resultado vía WebSocket, con opción de subir la imagen generada a un bucket S3/MinIO.

### Tecnologías usadas
- **Lenguaje**: Rust (edition 2024)
- **Runtime async**: `tokio`
- **Servidor HTTP/WebSocket**: `warp` + `tokio-tungstenite`
- **Motor de scripts**: `mlua` (Lua 5.4 embebido)
- **Serialización**: `serde`, `serde_json`
- **Imágenes**: `image` (genera PNG/GIF a partir de la grilla)
- **Storage**: `aws-sdk-s3` contra MinIO/S3
- **Config**: `dotenvy` para variables de entorno

### ¿Qué hace el proyecto?
- **Expone un WebSocket** en `ws://0.0.0.0:8080/render`.
- **Recibe scripts Lua** y una dimensión de grilla (\(n \times n\)).
- **Ejecuta el script** contra un objeto `grid` seguro (sin `os`, `io`, `debug`) que permite:
  - `set_pixel`, `set_pixel_rgba`
  - `set_area`
  - manejo de **frames** (`create_frame`, `switch_frame`) para animaciones.
- **Devuelve los frames** al cliente por WebSocket como mensajes JSON con los datos RGBA de cada frame.
- Opcionalmente, **renderiza la grilla a imagen** (PNG si es un frame, GIF si son varios) y:
  - **sube el archivo a MinIO/S3**
  - responde al cliente con la **URL pública** de la imagen.

### Variables de entorno principales
Se leen desde `.env` (usando `dotenvy`):
- **`MINIO_ENDPOINT`**: endpoint interno para subir archivos (`http://localhost:9000` por defecto).
- **`MINIO_PUBLIC_ENDPOINT`**: base URL pública que se devuelve al cliente (por defecto `/minio`).
- **`MINIO_ACCESS_KEY`** y **`MINIO_SECRET_KEY`**: credenciales de acceso al bucket `images`.

### Cómo correr el proyecto en local
1. **Instalar Rust** (stable) y `cargo`.
2. Asegurarte de tener un servicio **MinIO o S3-compatible** corriendo y accesible.
3. Crear un archivo `.env` en la raíz del proyecto con, al menos:
   ```bash
   MINIO_ENDPOINT=http://localhost:9000
   MINIO_PUBLIC_ENDPOINT=http://localhost:9000
   MINIO_ACCESS_KEY=tu_access_key
   MINIO_SECRET_KEY=tu_secret_key
   ```
4. Compilar y ejecutar:
   ```bash
   cargo run --release
   ```
5. Conectarte desde un cliente WebSocket a:
   ```text
   ws://localhost:8080/render
   ```

### Cómo correr con Docker
1. Construir la imagen:
   ```bash
   docker build -t gridbackend .
   ```
2. Ejecutar el contenedor, pasando el `.env` (o variables):
   ```bash
   docker run --env-file .env -p 8080:8080 gridbackend
   ```
   - El binario se ejecuta por defecto con `PORT=60016` pero el WebSocket usa el endpoint `0.0.0.0:8080` definido en código; ajusta el mapeo de puertos si cambias esto.

### Protocolo básico de mensajes
- **Cliente → servidor** (`ClientMessage`):
  ```json
  {
    "action": 0,                // 0 = ProcessSourceCode, 1 = PostToBucket, 2 = RenderPreview
    "data": {
      "source": "lua code...",
      "dimension": 64
    }
  }
  ```
- **Servidor → cliente** (`ServerMessage`):
  - Frames de imagen:
    ```json
    {
      "action": 0,
      "data": {
        "frame": {
          "frame_data": [[r,g,b,a], ...],
          "frame_id": 0
        }
      }
    }
    ```
  - URL de imagen subida:
    ```json
    {
      "action": 2,
      "data": {
        "urlBucket": "http://minio/..."
      }
    }
    ```

### Notas
- La dimensión de la grilla permitida va de **1** a **512**.
- El servidor limpia acceso a APIs peligrosas de Lua (`os`, `io`, `debug`) antes de ejecutar el script del usuario.
