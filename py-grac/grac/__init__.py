from enum import Enum

from .grac import *

__doc__ = grac.__doc__
if hasattr(grac, "__all__"):
    __all__ = grac.__all__

# fmt: off
class Diacritic(Enum):
    ACUTE = '\u0301'          # οξεία (oxia)
    GRAVE = '\u0300'          # βαρεία (varia)
    CIRCUMFLEX = '\u0342'     # περισπωμένη (perispomeni)
    IOTA_SUBSCRIPT = '\u0345' # υπογεγραμμένη (ypogegrammeni)
    DIAERESIS = '\u0308'      # διαλυτικά (diaeresis)
    SMOOTH = '\u0313'         # ψιλή (psili)
    ROUGH = '\u0314'          # δασεία (dasia)
# fmt: on
