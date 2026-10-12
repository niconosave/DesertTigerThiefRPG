# Documentación técnica

Este directorio traduce el [diseño general](../diseno-general.md) a decisiones de
implementación verificables. El
[diseño técnico general](./diseno-tecnico-general.md) define la arquitectura y
los límites compartidos del proyecto.

La estructura prevista es:

```text
tecnica/
├── diseno-tecnico-general.md
└── componentes/
    └── <documentos técnicos específicos>.md
```

El diseño técnico general define la organización global, los límites entre
partes y las reglas compartidas. Los documentos de `componentes/` diseñarán en
detalle una parte concreta del código o de los datos.

Todos seguirán la [metodología de desarrollo](../metodologia-desarrollo.md). Los
planes deberán enlazar sus secciones exactas en vez de repetir decisiones.
