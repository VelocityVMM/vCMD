'''Logging functions'''
COL_BLU = "\033[94m"
COL_GRN = "\033[92m"
COL_RED = "\033[91m"
COL_CLR = "\033[0m"

def p_n(text):
    '''Print newline'''
    print(text)

def pn_c(col, text):
    '''Print newline with color'''
    p_n(f"{col}{text}{COL_CLR}")

def debug(text):
    '''Print in debug color'''
    pn_c(COL_BLU, f"{text}")

def info(text):
    '''Print in info color'''
    pn_c(COL_GRN, f"{text}")

def err(text):
    '''Print in error color'''
    pn_c(COL_RED, f"{text}")
