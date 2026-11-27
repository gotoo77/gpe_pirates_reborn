#!/usr/bin/env python3
"""Capture real browser events via an existing Chromium CDP endpoint.
Requires the optional, already installed websocket-client Python module.
Run Chromium separately with --remote-debugging-port=9223 --remote-allow-origins=*.
No browser installation, launch or gameplay modification here.
"""
import base64
import json
import pathlib
import time
import urllib.request
import websocket

OUT = pathlib.Path("target/s1/browser")
OUT.mkdir(parents=True, exist_ok=True)
request = urllib.request.Request(
    "http://127.0.0.1:9223/json/new?http://127.0.0.1:18766/ui-spike.html", method="PUT")
with urllib.request.urlopen(request) as response:
    target = json.load(response)
ws = websocket.create_connection(target["webSocketDebuggerUrl"], timeout=30)
sequence = 0

def call(method, **params):
    global sequence
    sequence += 1
    ws.send(json.dumps(dict(id=sequence, method=method, params=params)))
    while True:
        response = json.loads(ws.recv())
        if response.get("id") == sequence:
            if "error" in response:
                raise RuntimeError(response["error"])
            return response.get("result", {})

def capture(name):
    time.sleep(0.3)
    data = call("Page.captureScreenshot", format="png")["data"]
    (OUT / (name + ".png")).write_bytes(base64.b64decode(data))

def mouse(kind, x, y, button="none", buttons=0):
    call("Input.dispatchMouseEvent", type=kind, x=x, y=y, button=button, buttons=buttons, clickCount=1)

call("Page.enable")
call("Emulation.setDeviceMetricsOverride", width=960, height=540, deviceScaleFactor=1, mobile=False)
time.sleep(4)
capture("01-initial")
frame_times = call("Runtime.evaluate", expression="""new Promise(resolve=>{let values=[],last;function tick(t){if(last!==undefined)values.push(t-last);last=t;if(values.length<120)requestAnimationFrame(tick);else{values.sort((a,b)=>a-b);resolve({raf_median_ms:values[60],raf_p95_ms:values[114],samples:120})}}requestAnimationFrame(tick)})""", awaitPromise=True, returnByValue=True)
(OUT / "raf.json").write_text(json.dumps(frame_times, indent=2))
mouse("mouseMoved", 50, 50)
mouse("mousePressed", 50, 50, "left", 1)
capture("02-pressed")
mouse("mouseReleased", 50, 50, "left")
capture("03-click")
call("Input.dispatchKeyEvent", type="keyDown", key=" ", code="Space", windowsVirtualKeyCode=32)
time.sleep(0.3)
call("Input.dispatchKeyEvent", type="keyUp", key=" ", code="Space", windowsVirtualKeyCode=32)
capture("04-space")
mouse("mousePressed", 50, 50, "left", 1)
mouse("mouseMoved", 10, 120, buttons=1)
mouse("mouseReleased", 10, 120, "left")
capture("05-outside")
call("Emulation.setTouchEmulationEnabled", enabled=True)
call("Input.dispatchTouchEvent", type="touchStart", touchPoints=[dict(x=50,y=50)])
call("Input.dispatchTouchEvent", type="touchEnd", touchPoints=[])
capture("06-touch")
call("Input.dispatchTouchEvent", type="touchStart", touchPoints=[dict(x=50,y=50)])
call("Input.dispatchTouchEvent", type="touchCancel", touchPoints=[])
capture("07-cancel")
call("Emulation.setDeviceMetricsOverride", width=640, height=360, deviceScaleFactor=1, mobile=False)
capture("08-resize")
print(json.dumps(call("Runtime.evaluate", expression="JSON.stringify({canvas:[...document.querySelectorAll('canvas')].map(c=>[c.width,c.height]), webgpu:!!navigator.gpu})", returnByValue=True)))
call("Page.close")
ws.close()
