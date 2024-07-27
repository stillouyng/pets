#!/usr/bin/env python3
# -*- coding: utf-8 -*-


import sys
from datetime import datetime

import keyboard
import numpy as np
import cv2
import os
from mss.windows import MSS as mss
from PIL import Image


def create_folders() -> bool:
    if not os.path.exists("Video"):
        os.makedirs("Video")
    return True


def create_filename() -> str:
    now = datetime.now()
    filename = f"{now.hour}-{now.minute}-{now.second}-{now.day}{now.month}{now.year}"
    return filename


class Capturer:

    def __init__(self, width, height, fps=30.0):
        self.monitor = {
            'left': 1920 - width,
            'top': 1080 - height,
            'width': width,
            'height': height
        }

        filename = create_filename()

        self.sct = mss()
        fourcc = cv2.VideoWriter_fourcc(*"XVID")
        self.vid = cv2.VideoWriter(
            filename=f"Video/{filename}.avi",
            fourcc=fourcc,
            fps=fps,
            frameSize=(
                self.monitor['width'],
                self.monitor['height']
            ),
            isColor=True
        )

    def mainloop(self):
        while True:
            if keyboard.is_pressed('f9'):
                break
            sct_img = self.sct.grab(self.monitor)
            sct_img_size = (sct_img.width, sct_img.height)

            img = Image.frombytes(
                'RGB',
                sct_img_size,
                sct_img.rgb
            )

            frame = cv2.cvtColor(
                np.array(img),
                cv2.COLOR_BGR2RGB
            )

            self.vid.write(frame)


if __name__ == '__main__':
    create_folders()

    w, h = 1919, 1079
    capturer = Capturer(width=w, height=h, fps=50.0)
    try:
        capturer.mainloop()
    except KeyboardInterrupt:
        print('Interrupted')
