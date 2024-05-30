# Proyecto final
> [!NOTE]  
> El mirror se ejecuta en un contenedor de Docker y se inicia al momento de crear el contenedor.

**Materia:** Administración de base de datos  
**Docente:** César David Sáenz  
**Equipo:** Red Hat Data Grid  

El siguiente proyecto trata sobre un punto de venta para una farmacia. Este proyecto se realizó junto con el proyecto de la materia Ingeniería de Software, en el que se nos pedía crear un sistema para un negocio pequeño que no contara con uno. Uno de los integrantes trabajaba en una farmacia en la que no tenían un sistema para poder atender a los clientes, por lo que tomamos la decisión de crear uno.

## Tecnologías:
- Docker
- Postgres DB
- Rust con el framework [Actix](https://actix.rs/)
- Node.js con el framework [Astro](https://astro.build/)

## Comandos:
### Docker
Para iniciar el proyecto se debe correr el siguiente comando:
```bash
docker compose up
```

Para detener el proyecto se debe correr el siguiente comando:
```bash
docker compose down
```