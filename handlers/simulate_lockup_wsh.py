#!/usr/bin/python

from mod_pywebsocket import msgutil
from mod_pywebsocket import common
from mod_pywebsocket import stream 

def web_socket_do_extra_handshake(request):
    request.ws_cookie = request.headers_in.get('Cookie')

def web_socket_transfer_data(request):
    msgutil.send_message(request, '(none)')

