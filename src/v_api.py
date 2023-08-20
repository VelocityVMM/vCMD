'''Velocity API implementation'''
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

import time
import threading
from datetime import datetime
import requests
from .log import err, info, debug

class VAPI:
    '''Represents an API connection to velocity'''
    REAUTH_INTERVAL: int = 50

    class Response:
        '''A response from the Velocity API'''
        def __init__(self, code: int, json: any):
            self.code = code
            self.json = json

    class Authkey:
        '''An authkey handed out by the velocity API'''
        def __init__(self, key: str, expires: int):
            self.key = key
            self.expires = expires

        def info(self) -> str:
            '''Returns information about this authkey'''
            return f"{self.key}, expires: {datetime.fromtimestamp(self.expires)}"

    def __init__(self, hostname: str, port: int):
        self._hostname = hostname
        self._port = port
        self._url = f"http://{self._hostname}:{self._port}"
        self._authkey: VAPI.Authkey = None
        self._auththread = threading.Thread(target=self.auto_reauth, daemon=True)
        self._auththread.start()

    def auto_reauth(self):
        '''Runs in a loop, reauthenticating this instance constantly'''
        while True:
            time.sleep(VAPI.REAUTH_INTERVAL)
            if not self._authkey is None:
                self.reauth(quiet=True)

    def request(self, method: str, endpoint: str, data: any, quiet: bool = False):
        '''Sends a request to the Velocity API'''
        if not quiet:
            debug(f"({method} - {self._url}) <- {endpoint}: {data}")
        res = requests.request(method, f"{self._url}{endpoint}", data=data, timeout=10)

        try:
            json = res.json()
        except requests.JSONDecodeError:
            json = None

        if not quiet:
            debug(f"({method} - {self._url}) -> {endpoint} ({res.status_code}): {json}")
        return VAPI.Response(res.status_code, json)

    def auth(self, username: str, password: str, quiet: bool = False) -> bool:
        '''Authenticates this API instance'''

        response = self.request("POST", "/u/auth", {
            "username": username,
            "password": password
        }, quiet=quiet)

        if not response.code == 200:
            return False

        self._authkey = VAPI.Authkey(response.json["authkey"], response.json["expires"])

        if not quiet:
            info(f"Authenticated as {username}: '{self._authkey.info()}")
        return True

    def reauth(self, quiet: bool = False) -> bool:
        '''Tries to reauthenticate using the current authkey'''

        if self._authkey is None:
            if not quiet:
                err("No authkey to reauthenticate")
            return False

        response = self.request("PATCH", "/u/auth", {
            "authkey": self._authkey.key
        }, quiet=quiet)

        if not response.code == 200:
            self._authkey = None
            return False

        self._authkey = VAPI.Authkey(response.json["authkey"], response.json["expires"])

        if not quiet:
            info(f"Reauthenticated: '{self._authkey.info()}")
        return True

    def deauth(self, quiet: bool = False):
        '''Deauthenticates the current authkey'''

        if not self._authkey is None:
            self.request("DELETE", "/u/auth", {
                "authkey": self._authkey.key
            }, quiet=quiet)

        if not quiet:
            info("Deauthenticated")
        self._authkey = None
