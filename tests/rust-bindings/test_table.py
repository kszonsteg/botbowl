def test_import_table_tile():
    from botbowl.rust_core.table import Tile

    Tile.HOME


def test_import_table():
    from botbowl.rust_core import table

    table.Tile.HOME
