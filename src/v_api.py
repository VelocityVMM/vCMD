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

import requests
from .log import debug

class VAPI:
    '''Represents an API connection to velocity'''

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

    def __init__(self, hostname: str, port: int):
        self._hostname = hostname
        self._port = port
        self._url = f"http://{self._hostname}:{self._port}"
        self._authkey: VAPI.Authkey = None

    def request(self, method: str, endpoint: str, data: any):
        '''Sends a request to the Velocity API'''
        debug(f"({method} - {self._url}) <- {endpoint}: {data}")
        res = requests.request(method, f"{self._url}{endpoint}", data=data, timeout=10)

        json = res.json()

        debug(f"({method} - {self._url}) -> {endpoint} ({res.status_code}): {json}")
        return VAPI.Response(res.status_code, json)

    def auth(self, username: str, password: str) -> bool:
        '''Authenticates this API instance'''

        response = self.request("POST", "/u/auth", {
            "username": username,
            "password": password
        })

        if not response.code == 200:
            return False

        self._authkey = VAPI.Authkey(response.json["authkey"], response.json["expires"])
