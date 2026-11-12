#!/usr/bin/env python3
"""Prepara un escenario aislado de incorporación y órdenes fechadas."""
from pathlib import Path
import shutil
import sys
from contenido_para_demo import copiar_base_demo

raiz = Path(__file__).resolve().parent.parent
destino = Path(sys.argv[1] if len(sys.argv) > 1 else '/tmp/desert-tiger-colectivos-demo').resolve()
if destino.exists():
    raise SystemExit(f'El destino ya existe: {destino}. Elegí otra carpeta.')
copiar_base_demo(raiz, destino)
for carpeta in ('collectives', 'events'):
    shutil.copytree(raiz / 'content/demos/colectivos' / carpeta, destino / carpeta)
world = destino / 'world.ron'
world.write_text(world.read_text().replace('colectivos: []', 'colectivos: ["compania_dahran"]', 1))
actor = destino / 'actors/dahran.ron'
actor.write_text(actor.read_text().replace('ubicacion_inicial: EnLugar("zarabad")', 'ubicacion_inicial: EnColectivo("compania_dahran")'))
print(f'Contenido de prueba preparado en: {destino}')
print(f'cargo run -p desert-tiger-terminal --bin desert-tiger-terminal -- --content {destino}')
