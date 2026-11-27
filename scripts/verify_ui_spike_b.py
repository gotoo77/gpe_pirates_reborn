#!/usr/bin/env python3
"""Bounded S1-B observations on an already running Chromium CDP endpoint.

Requires existing websocket-client and Pillow; installs/launches nothing.
Default server: 18767, CDP: 9224. Synthetic browser events are not hardware tests.
"""
import argparse
import base64
import hashlib
import io
import json
from pathlib import Path
import time
import urllib.request

from PIL import Image
import websocket


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dom-only", action="store_true")
    parser.add_argument("--out", default="target/s1-b-final/browser")
    args = parser.parse_args()
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    endpoint = "http://127.0.0.1:9224"

    def new_page(url):
        request = urllib.request.Request(f"{endpoint}/json/new?{url}", method="PUT")
        with urllib.request.urlopen(request, timeout=10) as response:
            return json.load(response)

    target = new_page("http://127.0.0.1:18767/ui-spike-b.html")
    ws = websocket.create_connection(target["webSocketDebuggerUrl"], timeout=15,
                                     origin="http://localhost:9224")
    sequence = 0
    observations = []

    def call(method, **params):
        nonlocal sequence
        sequence += 1
        ws.send(json.dumps(dict(id=sequence, method=method, params=params)))
        while True:
            response = json.loads(ws.recv())
            if response.get("id") == sequence:
                if "error" in response:
                    raise RuntimeError(response["error"])
                return response.get("result", {})

    def evaluate(expression):
        result = call("Runtime.evaluate", expression=expression, returnByValue=True)
        if "exceptionDetails" in result:
            raise RuntimeError(result["exceptionDetails"])
        return result["result"].get("value")

    def dom():
        return evaluate("""(()=>{const c=document.querySelector('canvas'),r=c.getBoundingClientRect();
        return {viewport:[innerWidth,innerHeight],canvas:[c.width,c.height],
        rect:[r.x,r.y,r.width,r.height],focused:document.hasFocus(),webgpu:!!navigator.gpu}})()""")

    def size(width, height):
        call("Emulation.setDeviceMetricsOverride", width=width, height=height,
             deviceScaleFactor=1, mobile=False)
        time.sleep(0.35)

    def capture(name, delay=0.15):
        time.sleep(delay)
        raw = base64.b64decode(call("Page.captureScreenshot", format="png")["data"])
        (out / f"{name}.png").write_bytes(raw)
        image = Image.open(io.BytesIO(raw)).convert("RGB")
        observations.append(dict(name=name, dom=dom()))
        return image

    def counter(image):
        # Constant-size comparisons prove no extra dispatch while held/cancelled.
        width, height = image.size
        scale = min(width / 960, height / 540)
        x, y = (width - 960 * scale) / 2, (height - 540 * scale) / 2
        box = tuple(round(v) for v in (x+32*scale, y+478*scale,
                                      x+400*scale, y+518*scale))
        return hashlib.sha256(image.crop(box).tobytes()).hexdigest()

    def color(image, point, expected):
        width, height = image.size
        scale = min(width / 960, height / 540)
        x = round((width-960*scale)/2 + point[0]*scale)
        y = round((height-540*scale)/2 + point[1]*scale)
        actual = image.getpixel((x, y))
        assert all(abs(a-b) <= 3 for a, b in zip(actual, expected)), (point, actual, expected)

    def mouse(kind, x=430, y=255, buttons=0):
        button = "left" if kind != "mouseMoved" else "none"
        call("Input.dispatchMouseEvent", type=kind, x=x, y=y,
             button=button, buttons=buttons, clickCount=1)
        time.sleep(0.07)

    def click(x=430, y=255):
        mouse("mousePressed", x, y, 1)
        mouse("mouseReleased", x, y)

    def key(key, code, vk, held=False):
        call("Input.dispatchKeyEvent", type="keyDown", key=key, code=code,
             windowsVirtualKeyCode=vk)
        time.sleep(0.08)
        if not held:
            call("Input.dispatchKeyEvent", type="keyUp", key=key, code=code,
                 windowsVirtualKeyCode=vk)
            time.sleep(0.08)

    try:
        call("Page.enable")
        call("Page.bringToFront")
        size(960, 540)
        time.sleep(4)
        initial = capture("01-initial")
        assert dom()["webgpu"]
        if args.dom_only:
            size(640, 360)
            capture("02-resize-before")
            print(json.dumps(observations, indent=2))
            return
        zero = counter(initial)
        color(initial, (520, 280), (30, 43, 60))
        color(initial, (550, 430), (58, 58, 58))
        mouse("mouseMoved")
        color(capture("02-hover"), (520, 280), (45, 75, 110))
        mouse("mousePressed", buttons=1)
        pressed = capture("03-pressed")
        color(pressed, (520, 280), (50, 145, 120))
        assert counter(pressed) == zero
        mouse("mouseReleased")
        one = counter(capture("04-click", 0.01))
        assert one != zero
        for key_name, code, vk, point in [
            ("ArrowRight", "ArrowRight", 39, (660, 210)),
            ("ArrowLeft", "ArrowLeft", 37, (330, 220)),
            ("ArrowUp", "ArrowUp", 38, (360, 130)),
            ("ArrowDown", "ArrowDown", 40, (330, 220)),
            ("ArrowLeft", "ArrowLeft", 37, (50, 230)),
            ("ArrowRight", "ArrowRight", 39, (330, 220)),
        ]:
            key(key_name, code, vk)
            image = capture(f"05-nav-{len(observations)}")
            color(image, point, (120, 235, 255))
            assert counter(image) == one
        click(430, 405)  # Initially disabled.
        assert counter(capture("06-disabled")) == one
        key("d", "KeyD", 68)
        key("ArrowDown", "ArrowDown", 40)
        color(capture("07-down-enabled"), (300, 370), (120, 235, 255))
        key("ArrowUp", "ArrowUp", 38)
        color(capture("08-down-return"), (330, 220), (120, 235, 255))
        key("Enter", "Enter", 13, held=True)
        two = counter(capture("09-enter"))
        assert two != one
        for _ in range(4):
            call("Input.dispatchKeyEvent", type="keyDown", key="Enter", code="Enter",
                 windowsVirtualKeyCode=13, autoRepeat=True)
        assert counter(capture("10-held-enter")) == two
        call("Input.dispatchKeyEvent", type="keyUp", key="Enter", code="Enter", windowsVirtualKeyCode=13)
        mouse("mousePressed", buttons=1)
        mouse("mouseMoved", 600, 330, 1)
        mouse("mouseReleased", 600, 330)
        assert counter(capture("11-outside")) == two
        mouse("mousePressed", buttons=1)
        key("Escape", "Escape", 27)
        mouse("mouseReleased")
        assert counter(capture("12-cancel")) == two
        mouse("mousePressed", buttons=1)
        key("r", "KeyR", 82)
        mouse("mouseReleased")
        assert counter(capture("13-suspend")) == two
        # A separate front tab causes actual document blur/focus notifications.
        mouse("mousePressed", buttons=1)
        blank = new_page("about:blank")
        other = websocket.create_connection(blank["webSocketDebuggerUrl"], timeout=10,
                                            origin="http://localhost:9224")
        other.send(json.dumps(dict(id=1, method="Page.bringToFront")))
        other.recv()
        time.sleep(0.3)
        blurred = dom()
        observations.append(dict(name="focus-lost", dom=blurred))
        assert not blurred["focused"], blurred
        call("Page.bringToFront")
        time.sleep(0.3)
        mouse("mouseReleased")
        assert counter(capture("14-focus-return-release")) == two
        click()
        three = counter(capture("15-fresh-click"))
        assert three != two
        other.send(json.dumps(dict(id=2, method="Page.close")))
        other.close()
        call("Emulation.setTouchEmulationEnabled", enabled=True)
        call("Input.dispatchTouchEvent", type="touchStart", touchPoints=[dict(x=430,y=255)])
        color(capture("16-touch-pressed"), (520, 280), (50, 145, 120))
        call("Input.dispatchTouchEvent", type="touchCancel", touchPoints=[])
        assert counter(capture("17-touch-cancel")) == three
        call("Input.dispatchTouchEvent", type="touchStart", touchPoints=[dict(x=430,y=255)])
        time.sleep(0.1)
        call("Input.dispatchTouchEvent", type="touchEnd", touchPoints=[])
        four = counter(capture("18-touch-end"))
        assert four != three
        call("Emulation.setTouchEmulationEnabled", enabled=False)
        for width, height in [(640, 360), (800, 600)]:
            size(width, height)
            before = capture(f"19-resize-{width}")
            info = dom()
            assert info["rect"] == [0, 0, width, height], info
            assert info["canvas"] == [width, height], info
            scale = min(width/960, height/540)
            x, y = (width-960*scale)/2, (height-540*scale)/2
            click(x+430*scale, y+255*scale)
            after = capture(f"20-resize-click-{width}")
            assert counter(after) != counter(before)
            color(after, (520, 280), (45, 75, 110))
            # Click a decoration/outside all targets at the resized surface.
            stable = counter(after)
            click(x+600*scale, y+330*scale)
            assert counter(capture(f"21-resize-outside-{width}")) == stable
        print("PASS: B browser assertions (synthetic events, software WebGPU)")
    finally:
        (out / "observations.json").write_text(json.dumps(observations, indent=2))
        call("Page.close")
        ws.close()


if __name__ == "__main__":
    main()
