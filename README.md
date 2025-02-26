## CLI R-PY

`r-py` es una herramienta de línea de comandos ligera inspirada en el comportamiento de `npm` de Node.js, diseñada para proyectos de Python. Permite gestionar y ejecutar comandos complejos de manera más sencilla mediante alias configurables.

Con `r-py`, puedes definir comandos predefinidos en un archivo de configuración y luego ejecutarlos fácilmente con un alias. Esto hace que ejecutar tareas repetitivas sea más rápido y menos propenso a errores, sin necesidad de recordar largos comandos.

### ¿Cómo funciona?

Al definir tus comandos en el archivo de configuración `cmd.ini` al nivel del archivo binario (o sea donde se está ejecutando el CLI), puedes crear alias para ejecutar tareas frecuentes. Por ejemplo, en lugar de escribir el largo comando:

```bash
uvicorn app.main:app --reload
```

Puedes simplemente ejecutar:

```bash
rpy start
```

### Ventajas

-   **Simplificación**: Olvídate de recordar y escribir comandos largos o complejos cada vez que los necesites.

-   **Personalización**: Puedes fácilmente modificar el comportamiento de tus comandos mediante la configuración de los alias.

-   **Automatización**: Organiza tus tareas en un solo archivo y ejecútalas con una simple llamada a `rpy <alias>`.

-   **Versatilidad**: `rpy` se adapta a cualquier proyecto Python, permitiéndote gestionar múltiples scripts de forma eficiente.

### Comandos predeterminados

El binario de `rpy` incluye de forma predeterminada los siguientes comandos:

-   **install [paquete]**: Instala las dependencias de Python. Si no se proporciona un paquete específico (ejemplo: `r-py install discord.py`), se instalarán todas las dependencias listadas en `requirements.txt`.

-   **init**: Inicializa un proyecto básico de Python en el directorio actual, creando los archivos esenciales.

-   **new <nombre>**: Crea un nuevo proyecto con el nombre especificado, generando una estructura de archivos inicial.

### Ejemplo de uso

1. **Crea un archivo** `cmd.ini` en tu proyecto al nivel del binario donde defines los alias y los comandos correspondientes:

```ini
start=uvicorn app.main:app --reload
stop=killall uvicorn
test=pytest
```

2. **Ejecuta el comando deseado**:

```bash
rpy start
```

Esto ejecutara el comando `uvicorn app.main:app --reload`, de manera más concisa.

### Instalación

> Puedes instalar este binario de forma rápida ejecutando `cargo install r-py`

Para instalar y usar `r-py`, simplemente descarga el archivo binario en las versiones del repositorio o sigue estos pasos:

1. Clona el repositorio:

```bash
git clone https://github.com/HormigaDev/r-py.git
cd r-py
```

2. Compílalo con cargo (asumiendo que ya tienes el ecosistema de rust):

```bash
cargo build --release
```

3. Copia el binario a una ubicación accesible, por ejemplo:

```bash
sudo cp ./target/release/r-py /usr/local/bin/
```

4. Ahora puedes usar `r-py` desde cualquier parte de tu sistema:

```bash
r-py start
```

### Contribución

Si quieres contribuir al proyecto, puedes hacerlo abriendo issues o pull requests en el repositorio de GitHub: https://github.com/HormigaDev/r-py.

### License

Este proyecto está bajo la licencia **MIT**. Puedes ver más detalles en el archivo LICENSE.
