#!/usr/bin/env python3
"""Prepara contenido aislado para probar eventos sin cambiar content/base."""
from pathlib import Path
import shutil
import sys
from contenido_para_demo import copiar_base_demo

raiz = Path(__file__).resolve().parent.parent
destino = Path(sys.argv[1] if len(sys.argv) > 1 else '/tmp/desert-tiger-eventos-demo').resolve()
if destino.exists():
    raise SystemExit(f'El destino ya existe: {destino}. Elegí otra carpeta.')
copiar_base_demo(raiz, destino)
shutil.copytree(raiz / 'content/demos/eventos/events', destino / 'events')
print(f'Contenido de prueba preparado en: {destino}')
print('Ejecutá el juego con --content seguido de esa ruta.')
