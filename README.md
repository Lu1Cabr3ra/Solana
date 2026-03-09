Descripción del Proyecto_SL

Funcionalidades (CRUD) El programa permite gestionar "Notas" personalizadas almacenadas directamente en la blockchain de Solana:

Create (Crear): Genera una nueva nota utilizando PDAs (Program Derived Addresses) para asegurar que cada nota sea única por usuario e ID.

Read (Leer): Los datos se almacenan de forma estructurada para ser consultados desde el cliente.

Update (Actualizar): Permite al autor de la nota modificar el contenido del texto.

Delete (Borrar): Cierra la cuenta de la nota y devuelve los fondos (SOL) al autor, optimizando el uso de espacio en la red.

Aspectos Técnicos PDA (Program Derived Address): Se utilizan semillas (seeds) basadas en la dirección del usuario y un ID numérico para derivar la dirección de la cuenta de forma determinística.

Seguridad: Implementa restricciones de acceso (has_one = autor) para garantizar que solo el creador de la nota pueda editarla o eliminarla.

Gestión de Rent: Se define un espacio fijo de space = 8 + 32 + 8 + 200 bytes para cubrir el discriminador, la llave pública, el ID y el contenido.
