#!python3
'''vCMD - Velocity API command line interface'''
# MIT License
#
# Copyright (c) 2023 Max Kofler
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import sys
import getpass
from cmd import Cmd
from .v_api import VAPI

print("VCMD 0.1 - Velocity API command line interface")
print("Copyright (c) 2023 Max Kofler (https://maxkofler.eu)")
print()

if len(sys.argv) < 3:
    print("Need the hostname and port")
    exit(-1)

class VCMD(Cmd):
    '''The main command class'''

    prompt = "vCMD> "

    def __init__(self):
        super().__init__()
        self._api = VAPI(sys.argv[1], sys.argv[2])

    def do_exit(self, _):
        '''Exit this application'''
        return True

    def do_auth(self, arg):
        '''Authenticate to a Velocity instance'''
        args = arg.split()

        if len(args) < 1:
            print("1 argument required: <username>")
            return

        password = getpass.getpass("Password: ")

        if not self._api.auth(args[0], password):
            print("Failed to authenticate")

    def do_reauth(self, _):
        '''Reauthenticate the current session'''

        if not self._api.reauth():
            print("Failed to reauthenticate")

    def do_deauth(self, _):
        '''Deauthenticate this session (log off)'''

        self._api.deauth()

    do_EOF = do_exit
