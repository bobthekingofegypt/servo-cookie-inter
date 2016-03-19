#!/usr/bin/python

from mod_pywebsocket import msgutil
from mod_pywebsocket import common
from mod_pywebsocket import stream 

def web_socket_do_extra_handshake(request):
    request.ws_cookie = request.headers_in.get('Cookie')

def web_socket_transfer_data(request):
    while True:
        body = stream.create_closing_handshake_body(1000, '')
        text_frame = stream.create_text_frame("(none)")
        close_frame = stream.create_close_frame(body)
        request.ws_stream._write(text_frame + close_frame)

        request.server_terminated = True
        line = request.ws_stream.receive_message()
        return

