from enum import Enum


class AlignmentEnum(Enum):
    CE = "CE"
    CN = "CN"
    CG = "CG"
    NE = "NE"
    N = "N"
    NG = "NG"
    LE = "LE"
    LN = "LN"
    LG = "LG"
    NO = "NO"  # no alignment
    ANY = "ANY"  # can be every alignment
