"""Copia la escena de pruebas mecánicas sin activar la apertura de campaña."""
from pathlib import Path
import re
import shutil


def copiar_base_demo(raiz: Path, destino: Path) -> None:
    shutil.copytree(raiz / 'content/base', destino,
                    ignore=shutil.ignore_patterns('events', 'collectives'))
    world = destino / 'world.ron'
    texto = re.sub(r'colectivos:\s*\[[^]]*\]', 'colectivos: []', world.read_text(), count=1)
    texto = re.sub(r'companeros_iniciales:\s*\[[^]]*\]',
                   'companeros_iniciales: ["bandido", "sirviente"]', texto, count=1)
    world.write_text(texto)
    for nombre in ('principe_perdido', 'bandido', 'sirviente'):
        actor = destino / f'actors/{nombre}.ron'
        actor.write_text(actor.read_text().replace(
            'ubicacion_inicial: EnLugar("costa_ambar")',
            'ubicacion_inicial: EnLugar("zona_arida")', 1))
    actor = destino / 'actors/dahran.ron'
    actor.write_text(actor.read_text().replace(
        'ubicacion_inicial: EnColectivo("compania_dahran")',
        'ubicacion_inicial: EnLugar("zarabad")', 1))
